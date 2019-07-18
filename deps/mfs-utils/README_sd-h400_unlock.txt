Synopsis:
---------

sd-h400_unlock is a utility to unlock the 80 hour capacity lock on
Toshiba SD-H400 Tivo/DVD combo units running the 5.1.1b version of the
TiVo software.  At this time, this is the only TiVo I know of that has
a capacity lock.

Caveats
-------

This software is modifying the TiVo Media File System.  This could
result in a corrupted MFS, resulting in a TiVo that won't boot.  Never
run utilities such as this on your original disk.  Always make a
backup.

The action taken by this tool could be undone by a future TiVo
software update.  I don't know what will happen to your recordings
beyond the 80 hour mark should this happen.  They might get
immediately purged.

Basic Usage:
------------

Copy and expand your TiVo disk to a new larger disk in the usual way
with mfstools (following the Hinsdale guide, for example).  Be sure to
use an LBA48 linux kernel if your new disk is greater than 137GB.  The
lba48 iso from ptvupgrade.com is suitable.
http://www.ptvupgrade.com/support/bigdisk/index.html.

You'll need to get the sd-h400_unlock program onto the linux
system you are using.  If you are booting from an CD image such as the
ptvupgrade lba48 CD, you can transfer it to linux via floppy.  The
Hinsdale guide describes this transfer for other utilities.  It's the
same for sd-h400_unlock.

Once the copy is complete, run this sd-h400_unlock program with the
"-w" switch and the linux device designation for the new disk.  For
example, if your new disk is the secondary master ide device, use
/dev/hdc.

  % sd-h400_unlock -w /dev/hdc

You'll see some output identifying what is being modified followed by
a "Success" or "Failure" message.  If all went well, you saw "Success"
and you are ready to put the drive back into the SD-H400 where you
will see the expanded capacity.

sd-h400_unlock will fail if it can not find the
/Config/DiskConfigurations/Active object in MFS.  This could happen if
you are trying to run it on a TiVo with a different software version,
or if you've already deleted the object using a different method,
described later in the Theory of Operations" section.

Advanced Usage:
----------------

In the Basic Usage example, we took all the program defaults.  There
are program options to control the MFS path of the object modified,
and allocation sizes for the User and TiVo Clips.

Here's the advanced usage information.  You can also get this via 
"sd-h400_unlock -h":

Usage: sd-h400_unlock [-p path] [-c tivoclipsKB] [-u userKB] [-w] [device]

  This program unlocks the 80 hour lock on a Toshiba SD-H400 TiVo by modifying
  the Active DiskConfiguration object in the Media File System (MFS).

  The device should be a device file such as /dev/hdc.  If not present, the
  vplay MFS_DEVLIST environment variable is used.

  The default is to modify "/Config/DiskConfigurations/Active" and 
  set the TiVoClips size to 10000000 and the User size to -1.  This reserves 
  10 million K bytes for tivo clips (showcases and ads), and expands the  user
  area to fill the remaining  available space.

  Without the -w option, the program is running in test mode: it will show you
  the changes it would make, but it won't write them to disk.

Theory of Operation
-------------------

The TiVo software allocates the Media File System (MFS) disk space
into three divisions: User clips, TiVo Clips (Ads, Showcases, etc),
and Replaceable Backgrounds.  On most TiVo software versions the
subdivision is determined from a table compiled into the main tivo
application (tivoapp).  MuscleNerd's "Recording space allocation
table" posting in the alt.org forums provides more detail about this
table.

The 5.1.1b software on the SD-H400 appears to also use an object in
MFS to determine the allocation to these three regions:
"/Config/DiskConfigurations/Active".  Some other TiVo software
versions include this object, but as far as I know 5.1.1b is the only
TiVo software version that uses it to enforce a capacity limit.

There are several ways to eliminate the capacity lock by manipulating
this object.  The first approach I tried was to just remove this
object from MFS using tivosh and the "RubbishObjectByFsid" function.
That approach works fine, but it requires that you have shell access
to your tivo.  Getting shell access requires circumventing TiVo
security checks and is more complicated than the solution offered by
the PC version of the sd-h400_unlock program.

The sd-h400_unlock program modifes the values within the Active
object.  There are two values changed: the SizeInKBs for the TivoClips
"Partition" and the SizeInKBs for the User "Partition".  Note that
although the term "Partition" is used, these are virtual partitions
within the MFS file system.  They are not paritions in the sense of
pdisk.

You can set these two values to whatever you want.  -1 means to expand
to fill all remaining MFS media space.  The default values set the
TiVo Clips allocation to 10000000 KBs and allocates the rest to User.
The 10000000 value was choosen since this is what is used in other
TiVo software versions for large disks.  You can try a smaller TiVo
Clips partition to recover a little more space for User recordings.
If you make it too small you might have problems with Showcases, or
cause error condition messages to be sent back to TiVo in the daily
calls.

License
-------

This software is distributed under the GPL V2: 

http://www.gnu.org/licenses/gpl.html

Acknowledgements
----------------

This program is based on mfs_dumpobj originally written by Andrew
Tridgell.  The majority of the code was taken directly from his
"vplay" software, available from CVS in module tivo:
http://www.samba.org/samba/cvs.html

The version of util.c here was taken from a DealDataBase posting by
AllDeadHomiez: http://www.dealdatabase.com/form/showthread.php?t=34943
ADH was also responsible for suggesting a PC unlock utility and for
providing information and guidance along the way.

MuscleNerd's "Recording space allocation" post on alt.org was a key
insight in helping me to discover where the capacity lock was imposed.

I read through parts of Tiger's mfstools source to get a better
understanding of MFS inode CRCs.  Without the right inode CRCs, any
modificiations I made to MFS objects were undone as soon as the TiVo
touched it!


--Jamie on DealDataBase.com
