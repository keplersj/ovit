#!/bin/bash

prog=${0##*/}
bindir=${0%%/$prog}

function usage() {
    cat 1>&2 <<EOF
Usage: $prog [-v] [-h] [-o <path|address:port>] <fsid>

Options:                                      {version %s - %s}
        -v  Verbose output (stderr)
        -o <path|address:port>  Write output to the specified file or to a
                                TCP connection to the specified host and port

        Output goes to stdout unless otherwise specified.

Examples:
	$prog 21000
        $prog -o 192.168.1.10:6900 21000
EOF
}

cmdline=("$bindir/mfs_uberexport" "-t")
n=2;
while getopts hvo:x o
do  case "$o" in
    h)  usage; exit 0;;
    o)  cmdline[$n]="-o";        n=$[$n+1]
        cmdline[$n]="$OPTARG";   n=$[$n+1];;
    v)  cmdline[$n]="-v";        n=$[$n+1];;
    x)  cmdline[$n]="-X";        n=$[$n+1];;
  [?])  usage; exit 1;;
    esac
done
shift $[$OPTIND-1]
if [ $# -ne 1 ]; then
    usage; exit 1;
fi
cmdline[$n]="-xR"; n=$[$n+1];
cmdline[$n]="$1"; n=$[$n+1];
exec "${cmdline[@]}"
