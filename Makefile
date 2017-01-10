#!/bin/sh

set -e

mkdir -p all-of-mono
	cd all-of-mono && for lib in $^ ; do ar x ../$$lib ; done
	cd all-of-mono && ar r ../liball-of-mono.a *.o
	mv -f liball-of-mono.a ${OUT_DIR}
	rm -rf all-of-mono
