#!/bin/bash

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT


readonly MISSPELL_DOWNLOAD="https://github.com/client9/misspell/releases/download/v0.3.4/misspell_0.3.4_linux_64bit.tar.gz"
readonly PROJECT_ROOT=$(pwd)
readonly TMP_DIR=$PROJECT_ROOT/tmp
readonly MISSPELL_TARBALL="$TMP_DIR/misspell.tar.bz2"
readonly MISSPELL="$TMP_DIR/misspell"

cleanup() {
  trap - SIGINT SIGTERM ERR EXIT
  # script cleanup here
}


source $PROJECT_ROOT/scripts/lib.sh
setup_colors

FLAGS=""

download() {
	if [ ! -e $MISSPELL ]; 
	then 
		msg "${GREEN}- Downloading misspell"
		wget --quiet  --output-doc=$MISSPELL_TARBALL $MISSPELL_DOWNLOAD;
		cd $TMP_DIR
		tar -xf $MISSPELL_TARBALL;
		cd $PROJECT_ROOT
	fi
}

spell_check_codespell() {
	codespell $FLAGS $PROJECT_ROOT/tests
	codespell $FLAGS $PROJECT_ROOT/docs/
	codespell $FLAGS --ignore-words-list crate .$PROJECT_ROOT/src
	codespell $FLAGS --ignore-words-list crate .$PROJECT_ROOT/README.md
}

spell_check_misspell() {
	mkdir $TMP_DIR || true
	download
	$MISSPELL $FLAGS $PROJECT_ROOT/docs
	$MISSPELL $FLAGS $PROJECT_ROOT/tests
	$MISSPELL $FLAGS -i crate $PROJECT_ROOT/src
	$MISSPELL $FLAGS -i crate $PROJECT_ROOT/README.md
}

check_arg $1

if match_arg $1 '-w' '--write'
then
	msg "${GREEN}- Checking and correcting typos"
	FLAGS="-w"
	spell_check_misspell
	spell_check_codespell
elif match_arg $1 '-c' '--check'
then
	msg "${GREEN}- Scaning for typos"
	spell_check_misspell
	spell_check_codespell
else
	msg "${RED}[!] Undefined option"
	exit 1
fi
