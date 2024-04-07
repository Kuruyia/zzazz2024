#!/usr/bin/env sh
cargo afl build
cargo afl fuzz -i in -o out -P exploit -D -T all target/debug/gbhttpd --rom-file-path gbhttp.gb