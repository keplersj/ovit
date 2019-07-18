#!/bin/bash

set -x
prog=${0##*/}
bindir=${0%%/$prog}

function usage() {
    cat 1>&2 <<EOF
usage: $prog [-v] [-s] [-h] <fsid>...

options:
        -v Verbose output (stderr)
        -s Dump raw tyStream data to stdout (required argument)
	-h Help... This text.

Example:
	$prog -s 1 2 3 4 5
EOF
}

cmdline=("$bindir/mfs_uberexport")
n=1;
ready=0
while getopts hvsc: o
do  case "$o" in
    h)  usage; exit 0;;
    s)  ready=1;;
    v)  cmdline[$n]="-v";  n=$[$n+1];;
    [?]) usage; exit 1;;
    esac
done
shift $[$OPTIND-1]
if [ $# -le 0 ]; then
    usage;
    exit 1;
fi
exec "${cmdline[@]}" "$@"
