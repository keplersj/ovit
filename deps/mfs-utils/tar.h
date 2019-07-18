#ifndef _TAR_H_
#define _TAR_H_

#define  RECORDSIZE  512
#define  NAMSIZ      100
#define  TUNMLEN      32
#define  TGNMLEN      32

union _tar_record {
    char        charptr[RECORDSIZE];
    struct header {
        char    name[NAMSIZ];
        char    mode[8];
        char    uid[8];
        char    gid[8];
        char    size[12];
        char    mtime[12];
        char    chksum[8];
        char    linkflag;
        char    linkname[NAMSIZ];
        char    magic[8];
        char    uname[TUNMLEN];
        char    gname[TGNMLEN];
        char    devmajor[8];
        char    devminor[8];
    } header;
};

/* The checksum field is filled with this while the checksum is computed. */
#define    CHKBLANKS    "        "        /* 8 blanks, no null */

/* The magic field is filled with this if uname and gname are valid. */
#define    TMAGIC    "ustar  "        /* 7 chars and a null */

/* The magic field is filled with this if this is a GNU format dump entry */
#define    GNUMAGIC  "GNUtar "        /* 7 chars and a null */

/* The linkflag defines the type of file */
#define  LF_OLDNORMAL '\0'       /* Normal disk file, Unix compatible */
#define  LF_NORMAL    '0'        /* Normal disk file */
#define  LF_LINK      '1'        /* Link to previously dumped file */
#define  LF_SYMLINK   '2'        /* Symbolic link */
#define  LF_CHR       '3'        /* Character special file */
#define  LF_BLK       '4'        /* Block special file */
#define  LF_DIR       '5'        /* Directory */
#define  LF_FIFO      '6'        /* FIFO special file */
#define  LF_CONTIG    '7'        /* Contiguous file */

/* Further link types may be defined later. */

/* Bits used in the mode field - values in octal */
#define  TSUID    04000        /* Set UID on execution */
#define  TSGID    02000        /* Set GID on execution */
#define  TSVTX    01000        /* Save text (sticky bit) */

/* File permissions */
#define  TUREAD   00400        /* read by owner */
#define  TUWRITE  00200        /* write by owner */
#define  TUEXEC   00100        /* execute/search by owner */
#define  TGREAD   00040        /* read by group */
#define  TGWRITE  00020        /* write by group */
#define  TGEXEC   00010        /* execute/search by group */
#define  TOREAD   00004        /* read by other */
#define  TOWRITE  00002        /* write by other */
#define  TOEXEC   00001        /* execute/search by other */

void create_tarheader(tar_record *rec, char *name, int size);
void write_tar_padding(int output_fd, int size);

#endif
