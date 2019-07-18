#!/bin/sh

MYARCH=`uname -ms | tr ' ' -`
if [ $# -gt 0 -a "$1" = "-all" ]; then
   ALL=1
fi

TMPDIR=/tmp
DATESTR=`date +'%Y%m%d'`

# Add a letter if we've already built a package today
MOD=
MODINDX=1
SRCPKG=$TMPDIR/mfs-utils_src-$DATESTR$MOD.tar.bz2
while [ -e $SRCPKG ]; do
  MOD=`expr substr "abcdefghijklmnopqrstuvwxyz" $MODINDX 1`;
  MODINDX=$[$MODINDX+1];
  SRCPKG=$TMPDIR/mfs-utils_src-$DATESTR$MOD.tar.bz2
done

NOARCHPKG=$TMPDIR/mfs-utils_noarch-$DATESTR$MOD.tar.bz2
MIPSPKG=$TMPDIR/mfs-utils_bin.mips-$DATESTR$MOD.tar.bz2
PPCPKG=$TMPDIR/mfs-utils_bin.ppc-$DATESTR$MOD.tar.bz2
I386PKG=$TMPDIR/mfs-utils_bin.$MYARCH-$DATESTR$MOD.tar.bz2

make clean
(cd ..; tar cvf - --exclude CVS mfs-utils | bzip2 -9 > $SRCPKG)
(cd ..; shopt -s nullglob; tar cvf - mfs-utils/{,contrib/}{*.txt,README*,*.patch} | bzip2 -9 >$NOARCHPKG)

if [ `uname` = Darwin ] ; then
make
else
make STATIC=1
fi
if [ ! -z "$ALL" ]; then
  make ARCH=mips
  make ARCH=ppc
fi

# Remove some programs that aren't needed in the distribution
rm -f bin.*/{mfs_bitmap,mfs_findzero,mfs_getslice,mfs_poke,mfs_purge,sd-h400_unlock,mfs_dump}
rm -f bin.$MYARCH/{vserver,tserver,NowShowing,mfs_tzoffset,vplay,vplit,vplayer}
rm -f bin.{mips,ppc}/{vplay,vsplit}

(cd ..; tar cvf - mfs-utils/bin.$MYARCH | bzip2 -9 > $I386PKG)
if [ ! -z "$ALL" ]; then
  (cd ..; tar cvf - mfs-utils/bin.mips | bzip2 -9 > $MIPSPKG)
  (cd ..; tar cvf - mfs-utils/bin.ppc  | bzip2 -9 > $PPCPKG)
fi

make clean

