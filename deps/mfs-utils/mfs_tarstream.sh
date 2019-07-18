#!/bin/bash

set -x

prog=${0##*/}
bindir=${0%%/$prog}

function usage() {
    cat 1>&2 <<EOF
Usage: $prog [-v] [-s] [-a] [-o <path|address:port>] [-x] <fsid>...

Options:
        -v  Verbose output (stderr)
        -s  Dump tarred tyStream data (required argument)
        -a  Open output file for appending (requires [-o path])
        -o <path|address:port>  Write output to the specified file or to a
                                TCP connection to the specified host and port
        -x  Read stdin and place the contents in a file called showing.xml
            inside of the tar archive.

        Output goes to stdout unless otherwise specified.

Examples:
	$prog -s 1 2 3 4 5
        $prog -s -o 192.168.1.10:6900 5 8 11 19 
        $prog -s -a -o /dev/null 23 42 69

EOF
}

ready=0
cmdline=("$bindir/mfs_uberexport" "-t")
n=2;
while getopts hvsao:x o
do  case "$o" in
    v)  cmdline[$n]="-v";        n=$[$n+1];;
    s)  ready=1;;
    a)  cmdline[$n]="-a";        n=$[$n+1];;
    o)  cmdline[$n]="-o";        n=$[$n+1]
        cmdline[$n]="$OPTARG";   n=$[$n+1];;
    x)  cmdline[$n]="-x";        n=$[$n+1];;
    h)  usage; exit 0;;
  [?])  usage; exit 1;;
    esac
done
if [ $ready -eq 0 ]; then
   usage;
   exit 1;
fi
shift $[$OPTIND-1]

exec "${cmdline[@]}" "$@"
