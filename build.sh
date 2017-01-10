#!/bin/sh

set -e

mkdir -p all-of-mono
cd all-of-mono
for lib in ../../mono_framework/dist/mono/*.a
do
    ar x ${lib}
done
ar r ../liball-of-mono.a *.o
cd ..
rm -rf all-of-mono

mv -f liball-of-mono.a ${OUT_DIR}
