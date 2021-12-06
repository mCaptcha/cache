#!/bin/bash

setup_colors() {
  if [[ -t 2 ]] && [[ -z "${NO_COLOR-}" ]] && [[ "${TERM-}" != "dumb" ]]; then
    NOCOLOR='\033[0m' RED='\033[0;31m' GREEN='\033[0;32m' ORANGE='\033[0;33m' BLUE='\033[0;34m' PURPLE='\033[0;35m' CYAN='\033[0;36m' YELLOW='\033[1;33m'
  else
    NOCOLOR='' RED='' GREEN='' ORANGE='' BLUE='' PURPLE='' CYAN='' YELLOW=''
  fi
}

msg() {
  echo >&2 -e "${1-}"
}

get_file_name() {
	basename -- $1
}

check_arg(){
    if [ -z $1 ]
    then
        help
        exit 1
    fi
}

match_arg() {
    if [ $1 == $2 ] || [ $1 == $3 ]
    then
        return 0
    else
        return 1
    fi
}
