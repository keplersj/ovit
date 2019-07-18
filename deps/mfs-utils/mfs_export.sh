#!/bin/bash

prog=${0##*/}
bindir=${0%%/$prog}

function usage() {
    cat 1>&2 <<EOF
usage: $prog [options] <path|fsid> <dest>

   options:
        -s <start>                     start offset
        -c <count>                     number of bytes (defaults to all)
EOF
}

cmdline=("$bindir/mfs_uberexport")
n=2;
while getopts hs:c: o
do  case "$o" in
    h)  usage; exit 0;;
    s)  cmdline[$n]="-s";      n=$[$n+1]
        cmdline[$n]="$OPTARG"; n=$[$n+1];;
    c)  cmdline[$n]="-c";      n=$[$n+1]
        cmdline[$n]="$OPTARG"; n=$[$n+1];;
  [?])  usage; exit 1;;
    esac
done
shift $[$OPTIND-1]
if [ $# -lt 1 -o $# -gt 2 ]; then
  usage;
  exit 1;
fi
if [ $# -eq 2 ]; then
  cmdline[$n]="-o";    n=$[$n+1]
  cmdline[$n]="$2";    n=$[$n+1]
fi
cmdline[$n]="$1"       n=$[$n+1]
exec "${cmdline[@]}"
