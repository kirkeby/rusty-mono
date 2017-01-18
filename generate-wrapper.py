import logging
import os.path
from os.path import commonprefix
import sys
from textwrap import dedent

from clang.cindex import (
    AccessSpecifier, CursorKind, Index, StorageClass, TypeKind, Type,
)

log = logging.getLogger(__name__)


interesting_kinds = set([
    CursorKind.CLASS_DECL,
    CursorKind.STRUCT_DECL,
    CursorKind.CONSTRUCTOR,
    CursorKind.DESTRUCTOR,
    CursorKind.CXX_METHOD,
])

visible_accesses = set([
    AccessSpecifier.PUBLIC,
    AccessSpecifier.INVALID,
])

operator_names = {
    '==': 'equals',
    '!=': 'not_equals',
    '()': 'call',
    '=': 'set',
    '[]': 'index',
}


def main():
    index = Index.create()
    lib_rs = empty_mod()
    stubs_cc = empty_stubs()

    clang_args = [
        '-x', 'c++',
        # FIXME -- need a better way to find these
        '-I/usr/lib/llvm-3.8/lib/clang/3.8.0/include',
    ]
    include_paths = []
    paths = []
    for arg in sys.argv[1:]:
        if arg.startswith('-I'):
            include_paths.append(arg[2:])
        if arg.startswith('-'):
            clang_args.append(arg)
        else:
            paths.append(arg)

    for path in paths:
        real_path = find_include(include_paths, path)
        stubs_cc['includes'].add(path)
        tu = index.parse(real_path, args=clang_args)
        for diag in tu.diagnostics:
            log.info(
                '%s %s %s %s',
                diag.severity, diag.location, diag.spelling, diag.option)
            if diag.severity > 2:
                raise Exception('%s %s' % (diag.location, diag.spelling))
        good_stuff = [
            cursor
            for cursor in tu.cursor.walk_preorder()
            if is_interesting(cursor)
            and cursor.location.file.name == real_path
        ]
        generate_lib_rs(lib_rs, good_stuff)
        generate_stubs_cc(stubs_cc, good_stuff)

    with open('src/generated.rs', 'w') as rs:
        write_rs(rs, lib_rs)
    with open('src/generated.cc', 'w') as cc:
        write_cc(cc, stubs_cc)


def find_include(roots, name):
    for root in roots:
        path = os.path.join(root, name)
        if os.path.exists(path):
            return path
    raise Exception('no such header %s' % name)


def is_interesting(cursor):
    if cursor.kind not in interesting_kinds:
        return False
    if cursor.access_specifier not in visible_accesses:
        return False
    if cursor.kind in (CursorKind.CLASS_DECL, CursorKind.STRUCT_DECL):
        # Templates classes don't have a type, for now we skip them.
        if cursor.type.kind == TypeKind.INVALID:
            log.debug(
                'ignoring %s (invalid type, probably a template?!)',
                full_name(cursor))
            return False
        if not cursor.is_definition():
            log.debug(
                'ignoring non-declaration of class %s (%s)',
                full_name(cursor), cursor.location)
            return False
    elif cursor.kind == CursorKind.CONSTRUCTOR:
        if not is_real_class(cursor.semantic_parent):
            log.debug(
                'ignoring %s on abstract class',
                full_name(cursor))
            return False
        if cursor.semantic_parent.type.get_size() <= 0:
            log.debug(
                'ignoring %s on unsized class',
                full_name(cursor))
            return False
    else:
        # check that class is not a template
        return is_interesting(cursor.semantic_parent)
    return True


def generate_lib_rs(lib_rs, cursors):
    # Generate structs for classes
    classes = [
        cursor for cursor in cursors
        if cursor.kind in (CursorKind.CLASS_DECL, CursorKind.STRUCT_DECL)
    ]
    for cls in classes:
        log.debug(
            'generating struct for %s', full_name(cls))
        ns = get_mod(lib_rs, cls)
        ns['structs'][cls.displayname] = {
            'name': cls.displayname,
            'size': rs_struct_size(cls),
        }

    # Generate stubs (i.e. extern declarations) and impls for methods
    methods = all_methods(cursors)
    fn_names = generate_rust_names(methods)
    generate_impls(lib_rs, methods, fn_names)


def all_methods(cursors):
    methods = []

    classes = [
        cursor
        for cursor in cursors
        if cursor.kind == CursorKind.CLASS_DECL
    ]
    for cls in classes:
        named = {}
        # The order from class_methods ensures inherited members get
        # overridden.
        for member in class_methods(cls):
            if member.access_specifier not in visible_accesses:
                continue
            named[member.displayname] = member
        methods.extend((cls, member) for member in named.values())

    return methods


def class_methods(cls):
    for b in class_bases(cls):
        for c in b.get_children():
            if c.kind == CursorKind.CXX_METHOD:
                yield c
    for c in cls.get_children():
        if c.kind in (CursorKind.CONSTRUCTOR, CursorKind.DESTRUCTOR):
            yield c


def class_bases(cls):
    for c in cls.get_children():
        if c.kind == CursorKind.CXX_BASE_SPECIFIER:
            for b in class_bases(c.get_definition()):
                yield b
            yield b
    yield cls


def rs_struct_size(cls):
    if cls.type.get_size() <= 0:
        return None
    return cls.type.get_size()


def generate_rust_names(methods):
    # Generate mapping for full Rust fn names -> mangled-names
    names = {}
    for cls, method in methods:
        rust_name = full_name(cls)
        if method.kind == CursorKind.CONSTRUCTOR:
            rust_name += '::new'
        elif method.kind == CursorKind.DESTRUCTOR:
            rust_name += '::drop'
        elif method.kind == CursorKind.CXX_METHOD:
            rust_name += '::' + method.spelling
        names.setdefault(rust_name, []).append(method.mangled_name)

    # Generate mapping from mangled-names to unique Rust names, by stripping
    # off the longest common prefix from the mangled-names where Rust fn names
    # are not unique
    mapping = {}
    for rust_name, mangled_names in names.items():
        if len(mangled_names) == 1:
            mapping[mangled_names[0]] = rust_name
        else:
            common = commonprefix(mangled_names)
            for mangled in mangled_names:
                suffix = '_' + mangled[len(common):]
                # Empty-argument-list gets the non-mangled name
                if suffix == '_v':
                    suffix = ''
                mapping[mangled] = rust_name + suffix

    # Finally return mapping from mangled-names to local Rust fn-names
    return {
        mangled: rust_name.split('::')[-1]
        for mangled, rust_name in mapping.items()
    }


def generate_impls(lib_rs, methods, fn_names):
    # Generate stub and impls
    for cls, method in methods:
        log.debug('generating impl for method %s',
            full_name(method))
        ns = get_mod(lib_rs, cls)
        struct = rs_type(lib_rs, cls.type)

        # Add stub for method
        stub_params = ['o: *mut u8']
        stub_params.extend([
            'p{}: {}'.format(i, rs_type(lib_rs, arg_type))
            for i, arg_type in enumerate(method.type.argument_types())
        ])
        stub_result = rs_type(lib_rs, method.type.get_result())
        stub_name = 'stub_' + method.mangled_name
        ns['stubs'][stub_name] = {
            'name': stub_name,
            'pretty_name': full_name(method),
            'parameters': ', '.join(stub_params),
            'return_type': stub_result,
        }

        # Add impl for methods
        fn_visibility = 'pub'
        struct_name = impl_name = struct.split('::')[-1]
        if method.kind == CursorKind.DESTRUCTOR:
            fn_visibility = ''
            impl_name = 'Drop for ' + impl_name
        impl = ns['impls'].setdefault(impl_name, {})
        fn_name = fn_names[method.mangled_name]

        if fn_name.startswith('operator'):
            log.warn('ignoring %s', full_name(method))
            continue

        if method.kind == CursorKind.CONSTRUCTOR:
            fn_params = []
            stub_args = ['&mut o.raw[0] as *mut u8']
            fn_result = struct
            fn_body = '''
                let mut o = {struct[name]} {{ raw: [0; {struct[size]}] }};
                unsafe {{ {stub_name}({stub_args}); }}
                o
            '''
        else:
            fn_params = ['&mut self']
            stub_args = ['&mut self.raw[0] as *mut u8']
            fn_result = stub_result
            fn_body = 'unsafe {{ {stub_name}({stub_args}) }}'
        stub_args.extend(
            'p{}'.format(i)
            for i in range(len(stub_params) - 1)
        )
        fn_params.extend(stub_params[1:])
        fn_body = dedent(fn_body.lstrip('\n').rstrip().format(
            struct=ns['structs'][struct_name],
            stub_name=stub_name,
            stub_args=', '.join(stub_args),
        ))
        # Indent multi-line bodies correctly.
        fn_body = fn_body.replace('\n', '\n    ')

        fn = {
            'fn_visibility': fn_visibility,
            'fn_name': fn_name,
            'fn_body': fn_body,
            'stub_name': stub_name,
            'struct': struct,
            'parameters': ', '.join(fn_params),
            'return_type': fn_result,
        }
        impl[fn_name] = dedent('''\
            {fn_visibility} fn {fn_name}({parameters}) -> {return_type} {{
                {fn_body}
            }}
        ''').format(**fn)


def write_rs(f, rs, prefix=''):
    def W(s, prefix=prefix):
        for line in s.splitlines():
            f.write(prefix + line + '\n')

    if prefix == '':
        # FIXME - should probably mangle method-names in Rust'ic.
        W('#![allow(non_snake_case)]')
        # FIXME - should probably change the types in the stubs and add casts
        W('#![allow(improper_ctypes)]')

    for name, mod in sorted(rs['mods'].items()):
        W("pub mod {} {{".format(name))
        write_rs(f, mod, prefix + '    ')
        W("}")

    for _, struct in sorted(rs['structs'].items()):
        W('#[allow(dead_code)]')
        W("pub struct {} {{".format(struct['name']))
        # FIXME - should probably change the types in the stubs and add casts,
        # so we can get rid of this spurious dead-code.
        if struct.get('size') is None:
            W("    raw: [u8],")
        else:
            W("    raw: [u8; {}],".format(struct['size']))
        W("}")

    W('#[allow(dead_code)]')
    W('extern "C" {')
    for _, stub in sorted(rs['stubs'].items()):
        W('    // ' + stub['pretty_name'])
        W('    fn {name}({parameters}) -> {return_type};'.format(**stub))
    W('}')

    for name, fns in sorted(rs['impls'].items()):
        W('impl {} {{'.format(name))
        for _, body in sorted(fns.items()):
            W(body, prefix='    ' + prefix)
        W('}')



def empty_mod():
    return {
        'structs': {},
        'impls': {},
        'stubs': {},
        'mods': {},
    }


def rs_type(lib_rs, cursor):
    assert isinstance(cursor.kind, TypeKind), `cursor`

    if cursor.kind == TypeKind.UNEXPOSED:
        return rs_type(lib_rs, cursor.get_canonical())
    if cursor.kind == TypeKind.LVALUEREFERENCE:
        return '&' + rs_type(lib_rs, cursor.get_pointee())
    if cursor.kind == TypeKind.POINTER:
        return '*mut ' + rs_type(lib_rs, cursor.get_pointee())
    if cursor.kind == TypeKind.TYPEDEF:
        return rs_type(lib_rs, cursor.get_canonical())

    if cursor.kind == TypeKind.RECORD:
        cls = cursor.get_declaration()
        cls_name = cls.spelling
        ns = get_mod(lib_rs, cls)
        # Make sure we at least create useless structs for naming-purposes
        if cls_name not in ns['structs']:
            ns['structs'][cls_name] = {'name': cls_name}
        ns['structs'].setdefault(cls_name, {})
        t = '::generated::' + full_name(cls)
    elif cursor.kind == TypeKind.VOID:
        t = '()'
    elif cursor.kind == TypeKind.BOOL:
        t = 'bool'
    elif cursor.kind in (TypeKind.ULONG, TypeKind.UINT, TypeKind.USHORT, TypeKind.UCHAR):
        t = 'u{}'.format(cursor.get_size() * 8)
    elif cursor.kind in (TypeKind.LONG, TypeKind.INT, TypeKind.SHORT, TypeKind.CHAR_S):
        t = 'i{}'.format(cursor.get_size() * 8)
    elif cursor.kind == TypeKind.FLOAT:
        t = 'f{}'.format(cursor.get_size() * 8)
    elif cursor.kind == TypeKind.ENUM:
        # FIXME - create enums for enums
        t = 'i{}'.format(cursor.get_size() * 8)
    elif cursor.kind == TypeKind.FUNCTIONPROTO:
        return 'fn ({}) -> {}'.format(
            ', '.join(rs_type(lib_rs, a) for a in cursor.argument_types()),
            rs_type(lib_rs, cursor.get_result()))
    else:
        log.debug('no rs_type for %r (%r) %r',
            cursor, cursor.kind, cursor.spelling)
        raise ValueError('no rs_type for %s' % (cursor.kind))

    # FIXME - hrmmpf
    #if not cursor.is_const_qualified():
    #    return 'mut ' + t

    return t



def get_mod(root, cursor):
    pieces = []
    while cursor:
        if cursor.kind == CursorKind.NAMESPACE:
            pieces.append(cursor.displayname)
        cursor = cursor.semantic_parent
    for name in reversed(pieces):
        root = root['mods'].setdefault(name, empty_mod())
    return root


def empty_stubs():
    return {
        'includes': set(),
        'stubs': {},
    }


def generate_stubs_cc(stubs_cc, cursors):
    methods = all_methods(cursors)
    for cls, method in methods:
        name = 'stub_' + method.mangled_name
        return_type = cc_type(method.type.get_result())
        parameters = ['void *o']
        parameters.extend(
            cc_declaration(t, 'a{}'.format(i))
            for i, t in enumerate(method.type.argument_types())
        )

        class_name = full_name(method.semantic_parent)
        method_name = method.spelling
        method_args = ', '.join(
            'a{}'.format(i)
            for i in range(len(method.type.argument_types()))
        )
        body = '(({class_name} *) o)->{method_name}({method_args});'
        if method.kind == CursorKind.CONSTRUCTOR:
            if not is_real_class(cls):
                assert cls.spelling != 'TextLabelView'
                continue
            body = 'new(o) {class_name}({method_args});'
        body = body.format(
            class_name=class_name,
            method_name=method_name,
            method_args=method_args,
        )
        if return_type != 'void':
            body = 'return ' + body

        stubs_cc['stubs'][name] = {
            'name': name,
            'return_type': return_type,
            'parameters': ', '.join(parameters),
            'pretty_name': full_name(method),
            'body': body,
        }


def cc_declaration(t, name):
    t = cc_type(t)
    if '(*)' in t:
        # FIXME - nasty-hack to fix function-types
        return t.replace('(*)', '(*{})'.format(name))
    else:
        return '{} {}'.format(t, name)


def cc_type(cursor):
    return cursor.get_canonical().spelling


def is_real_class(cls):
    return not any(c.is_pure_virtual_method() for c in cls.get_children())


def write_cc(f, cc):
    f.write('#include <new>\n')
    for include in cc['includes']:
        f.write('#include "{}"\n'.format(include))

    for _, stub in sorted(cc['stubs'].items()):
        f.write(dedent('''
            /* {pretty_name} */
            extern "C"
            {return_type} {name}({parameters})
            {{
                {body}
            }}
        '''.format(**stub)))


def full_name(cursor, separator='::'):
    pieces = []
    while cursor:
        if cursor.kind == CursorKind.TRANSLATION_UNIT:
            break
        pieces.insert(0, cursor.displayname)
        cursor = cursor.semantic_parent
    return separator.join(pieces)


if __name__ == '__main__':
    logging.basicConfig(level=0)
    try:
        main()
    except Exception:
        import ipdb, traceback
        exc, _, tb = sys.exc_info()
        traceback.print_exc(exc)
        ipdb.post_mortem(tb)
