#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

#include "mfs.h"
#include "tar.h"

/* Calculate the header checksum and store it */
static void checksum(tar_record *rec)
{
	int chksum = 0;
	int i;

	for (i = 0; i < sizeof(tar_record); i++)
		chksum += rec->charptr[i];

	sprintf(rec->header.chksum, "%07o", chksum);
}

/* Build the tar header, giving defaults for all but name and size */
void create_tarheader(tar_record *rec, char *name, int size)
{
	memset(rec, 0, sizeof(tar_record));

	strncpy(rec->header.name, name, sizeof(rec->header.name));
	strcpy(rec->header.mode, "0100644");
	strcpy(rec->header.uid, "0000000");
	strcpy(rec->header.gid, "0000000");
	sprintf(rec->header.size, "%011o", size);
	sprintf(rec->header.mtime, "%011lo", time(NULL));
	sprintf(rec->header.chksum, "        ");
	rec->header.linkflag = LF_NORMAL;
	strcpy(rec->header.magic, TMAGIC);
	strcpy(rec->header.uname, "tivo");
	strcpy(rec->header.gname, "tivo");
	checksum(rec);
}

/* write out padding bytes at the end of a tar archive; use this after writing size data bytes */
void write_tar_padding(int output_fd, int size)
{
    char *zilch;
    int slack;

    if (size % RECORDSIZE) {
        zilch = calloc(RECORDSIZE, 1);
        if (zilch == NULL) {
            perror("calloc failed\n");
            exit(1);
        }

        slack = RECORDSIZE - (size % RECORDSIZE);
        write(output_fd, zilch, slack);

        free(zilch);
        zilch = NULL;
    }
}
