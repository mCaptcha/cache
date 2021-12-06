#!/bin/bash

set -Eeuo pipefail

readonly GRCOV_DOWNLOAD="https://github.com/mozilla/grcov/releases/download/v0.8.2/grcov-linux-x86_64.tar.bz2"
readonly PROJECT_ROOT=$(pwd)
readonly TMP_DIR=$PROJECT_ROOT/tmp
readonly GRCOV_TARBAL="$TMP_DIR/grcov.tar.bz2"
readonly GRCOV="$TMP_DIR/grcov"

source $(pwd)/scripts/lib.sh


clean_up() {
    trap - SIGINT SIGTERM ERR EXIT
	cd $PROJECT_ROOT
	/bin/rm default.profraw  lcov.info *.profraw || true
	cd target
	/bin/rm default.profraw  lcov.info *.profraw || true
}

trap cleanup SIGINT SIGTERM ERR EXIT
setup_colors

download() {
	if [ ! -e $GRCOV ]; 
	then 
	msg "${GREEN}- Downloading grcov"
		wget --quiet  --output-doc=$GRCOV_TARBAL $GRCOV_DOWNLOAD;
		cd $TMP_DIR
		tar -xf $GRCOV_TARBAL;
		cd $PROJECT_ROOT
	fi
}

build_and_test() {
	export RUSTFLAGS="-Zinstrument-coverage"
	cd $PROJECT_ROOT

	msg "${GREEN}- Building project"
	cargo build

	export LLVM_PROFILE_FILE="target/mcatpcha-cache-%p-%m.profraw"

	msg "${GREEN}- Running tests"
    cargo test --lib

	msg "${GREEN}- Generating coverage"
	$GRCOV target/ --binary-path  \
		./target/debug/ \
		-s . -t lcov --branch \
		--ignore-not-existing \
		--ignore "../*" -o target/lcov.info
}

run_coverage() {
	cd $PROJECT_ROOT
	mkdir $TMP_DIR || true
	clean_up
	download
	build_and_test
}

check_arg $1

if match_arg $1 '-c' '--coverage'
then
	run_coverage
else
	msg "${RED}[!] Undefined option"
	exit 1
fi
