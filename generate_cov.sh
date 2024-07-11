#!/bin/bash

rm -f *.profraw
rm -f *.profdata
cargo clean

OBJS=$(RUSTFLAGS="-C instrument-coverage -Z coverage-options=branch" cargo +nightly test --tests --no-run -- --test-threads=1 2>&1 | grep -oP '(?<=\()[^ ]+[\\\/][^ \)]+' | sed 's/^/--object /' | sed 's/\r//' | tr '\n' ' ')
RUSTFLAGS="-C instrument-coverage -Z coverage-options=branch,mcdc" cargo +nightly test --tests -- --test-threads=1
cargo profdata -- merge -sparse *.profraw -o default.profdata
cargo cov -- show -show-line-counts-or-regions -output-dir=html -format=html --ignore-filename-regex='.*cargo.*' --instr-profile=default.profdata $OBJS

rm -f *.profraw
rm -f *.profdata
cargo clean
