NAME=$(shell basename $(CURDIR))

MONO_PATH=../../mono_framework/dist/mono
CARGO_OUT=target/thumbv7m-none-eabi/debug

ARCH_OPTS=\
	-mcpu=cortex-m3 -mthumb -mthumb-interwork -march=armv7-m \
	-mfix-cortex-m3-ldrd
LD=arm-none-eabi-gcc

MONO_LIBS=\
	$(MONO_PATH)/monoCyLib.a \
	$(MONO_PATH)/CyComponentLibrary.a \
	$(MONO_PATH)/mbedlib.a \
	$(MONO_PATH)/mono_framework.a

.PHONY: all install clean ${NAME}.elf

all: ${NAME}.elf

# FIXME - the -Wl,--no-wchar-size-warning is to silence "uses 2-byte wchar_t
# yet the output is to use 4-byte wchar_t; use of wchar_t values across
# objects may fail" from linking. Should fix the actual problem.
${NAME}.elf:
	xargo build
	arm-none-eabi-gcc \
		-o $@ \
		${ARCH_OPTS} \
		-Wl,--no-wchar-size-warning \
		-Wl,--gc-sections \
		-Wl,--start-group \
		-specs=nano.specs \
		-T ${MONO_PATH}/include/mbed/target_cypress/cm3gcc.ld \
		${MONO_LIBS} \
		$(CARGO_OUT)/lib${NAME}.a \
		-lstdc++ \
		-Wl,--end-group

install: ${NAME}.elf
	monomake -p $^

clean:
	rm -rf target ${NAME}.elf
