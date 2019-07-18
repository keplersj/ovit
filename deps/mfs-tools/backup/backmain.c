#ifdef HAVE_CONFIG_H
#include <config.h>
#endif
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>
#include <errno.h>
#include <inttypes.h>
#include <time.h>

#ifdef HAVE_ASM_TYPES_H
#include <asm/types.h>
#endif

#include "mfs.h"
#include "backup.h"
#include "macpart.h"

#define BUFSIZE 512 * 256

void
backup_usage (char *progname)
{
	fprintf (stderr, "%s %s\n", PACKAGE, VERSION);
	fprintf (stderr, "Usage: %s [options] Adrive [Bdrive]\n", progname);
	fprintf (stderr, "Options:\n");
	fprintf (stderr, " -h        Display this help message\n");
	fprintf (stderr, " -o file   Output to file, - for stdout\n");
	fprintf (stderr, " -1 .. -9  Compress backup, quick (-1) through best (-9)\n");
	fprintf (stderr, " -v        Do not include /var in backup\n");
	fprintf (stderr, " -d        Do not include /db (SQLite) in backup (Premiere and newer)\n");
	fprintf (stderr, " -s        Shrink MFS in backup (implied for v3 backups without -a flag)\n");
	fprintf (stderr, " -F format Backup using a specific backup format (v1, v3, winmfs)\n");
	fprintf (stderr, " -q        Do not display progress\n");
	fprintf (stderr, " -qq       Do not display anything but error messages\n");
#if DEPRECATED
	// Mostly used to copy loopsets when excluding recordings.  Loopsets are now handled automatically, so these are less useful now...
	fprintf (stderr, " -f max    Backup only fsids below max\n");
	fprintf (stderr, " -L max    Backup only streams less than max MiB\n");
#endif
	fprintf (stderr, " -t        Use total length of stream in calculations\n");
	fprintf (stderr, " -T        Backup total length of stream instead of used length\n");
	fprintf (stderr, " -a        Backup all streams\n");
	fprintf (stderr, " -i        Include all non-mfs partitions from Adrive (alternate, custom, etc.)\n");
#if DEPRECATED
	// C'mon, who would do this ???
	fprintf (stderr, " -D        Do not force loopset and demo files to be added\n");
#endif
}

static unsigned int
get_percent (uint64_t current, uint64_t max)
{
	unsigned int prcnt;
	if (max <= 0x7fffffff / 10000)
	{
		prcnt = current * 10000 / max;
	}
	else if (max <= 0x7fffffff / 100)
	{
		prcnt = current * 100 / (max / 100);
	}
	else
	{
		prcnt = current / (max / 10000);
	}

	return prcnt;
}

#define SABLOCKSEC 1630000

uint64_t
sectors_no_reserved (uint64_t sectors)
{
	if (sectors < 14 * 1024 * 1024 * 2)
		return sectors;
	if (sectors > 72 * 1024 * 1024 * 2)
		return sectors - 12 * 1024 * 1024 * 2;
	return sectors - (sectors - 14 * 1024 * 1024 * 2) / 4;
}

void
display_backup_info (struct backup_info *info)
{
	zone_header *hdr = 0;
	uint64_t sizes[32];
	int count = 0;
	int loop;
	uint64_t backuptot = 0;
	uint64_t backupmfs = 0;

	for (loop = 0; loop < info->nmfs; loop++)
	{
		backupmfs += info->mfsparts[loop].sectors;
	}

	while ((hdr = mfs_next_zone (info->mfs, hdr)) != 0)
	{
		unsigned int zonetype;
		uint64_t zonesize;
		uint64_t zonefirst;

		if (mfs_is_64bit (info->mfs))
		{
			zonetype = intswap32 (hdr->z64.type);
			zonesize = intswap64 (hdr->z64.size);
			zonefirst = intswap64 (hdr->z64.first);
		}
		else
		{
			zonetype = intswap32 (hdr->z32.type);
			zonesize = intswap32 (hdr->z32.size);
			zonefirst = intswap32 (hdr->z32.first);
		}

		if (zonetype == ztMedia)
		{
			if (zonefirst < backupmfs)
				backuptot += zonesize;
			sizes[count++] = zonesize;
		}
		else
			while (count > 1)
			{
				sizes[0] += sizes[--count];
			}
	}

	if (sizes[0] > 0)
	{
		uint64_t running = sizes[0];
		fprintf (stderr, "Source drive size is %" PRIu64 " hours\n", sectors_no_reserved (running) / SABLOCKSEC);
		if (count > 1)
			for (loop = 1; loop < count; loop++)
			{
				running += sizes[loop];
				fprintf (stderr, "       - Upgraded to %" PRIu64 " hours\n", sectors_no_reserved (running) / SABLOCKSEC);
			}
		if (info->back_flags & BF_SHRINK && info->format == bfV1)
			fprintf (stderr, "Backup image will be %" PRIu64 " hours\n", sectors_no_reserved (backuptot) / SABLOCKSEC);
	}
}

int
backup_main (int argc, char **argv)
{
	struct backup_info *info;
	int loop;
	unsigned int thresh = 0;
	unsigned int bflags = BF_BACKUPVAR;
	char threshopt = '\0';
	char *drive, *drive2;
	char *filename = 0;
	char *tmp;
	int quiet = 0;
	int compressed = 0;
	int norescheck = 0;
	unsigned int skipdb = 0;
	unsigned starttime = 0;

	enum backup_format selectedformat = bfV3;

	tivo_partition_direct ();

#if DEPRECATED
	while ((loop = getopt (argc, argv, "ho:123456789vsf:L:tTaqEF:idD")) > 0)
#else
	while ((loop = getopt (argc, argv, "ho:123456789vstTaqEF:id")) > 0)
#endif
	{
		switch (loop)
		{
		case 'o':
			filename = optarg;
			break;
		case '1':
		case '2':
		case '3':
		case '4':
		case '5':
		case '6':
		case '7':
		case '8':
		case '9':
			bflags |= BF_SETCOMP (loop - '0');
			compressed = 1;
			break;
		case 'i':
			bflags |= BF_BACKUPALL;
			break;
		case 'v':
			bflags &= ~BF_BACKUPVAR;
			break;
		case 'd':
			skipdb = 1;
			break;
		case 's':
			bflags |= BF_SHRINK;
			break;
		case 'f':
			if (threshopt)
			{
				fprintf (stderr, "%s: -f and -%c cannot be used together\n", argv[0], threshopt);
				return 1;
			}
			threshopt = loop;
			thresh = strtoul (optarg, &tmp, 10);
			if (*tmp)
			{
				fprintf (stderr, "%s: Non integer argument to -f\n", argv[0]);
				return 1;
			}
			break;
		case 'F':
			if (!strcasecmp (optarg, "v1"))
				selectedformat = bfV1;
			else if (!strcasecmp (optarg, "v3"))
				selectedformat = bfV3;
			else if (!strcasecmp (optarg, "v3p"))
			{
				// Special case to force BF_SQLITE flag.  Should only be needed to correct backing up an image that was restored with a previous version of mfstools that contained a bug.
				selectedformat = bfV3;
				bflags |= BF_SQLITE;
				fprintf (stderr, "Forcing BF_SQLITE flag\n", argv[0]);
			}
			else if (!strcasecmp (optarg, "winmfs"))
				selectedformat = bfWinMFS;
			else
			{
				fprintf (stderr, "%s: Argument to -F must be one of V1, V3, or WinMFS\n", argv[0]);
				return 1;
			}
			break;
		case 'L':
			if (threshopt)
			{
				fprintf (stderr, "%s: -L and -%c cannot be used together\n", argv[0], threshopt);
				return 1;
			}
			threshopt = loop;
			thresh = strtoul (optarg, &tmp, 10);
			thresh *= 1024 * 2;
			bflags |= BF_THRESHSIZE;
			if (*tmp)
			{
				fprintf (stderr, "%s: Non integer argument to -l\n", argv[0]);
				return 1;
			}
			break;
		case 't':
			bflags |= BF_THRESHTOT;
			break;
		case 'T':
			bflags += BF_STREAMTOT;
			break;
		case 'a':
			if (threshopt)
			{
				fprintf (stderr, "%s: -a and -%c cannot be used together\n", argv[0], threshopt);
				return 1;
			}
			threshopt = loop;
			thresh = ~0;
			// No need to check for resource files if we are including all streams
			norescheck = 1;
			break;
		case 'q':
			quiet++;
			break;
		case 'E':
			bflags |= BF_TRUNCATED;
			break;
		case 'D':
			norescheck = 1;
			break;
		default:
			backup_usage (argv[0]);
			return 1;
		}
	}

	if (!filename)
	{
		backup_usage (argv[0]);
		return 1;
	}

	drive = 0;
	drive2 = 0;
	if (optind < argc)
		drive = argv[optind++];
	if (optind < argc)
		drive2 = argv[optind++];
	if (optind < argc || !drive)
	{
		backup_usage (argv[0]);
		return 1;
	}

	switch (selectedformat)
	{
	case bfV1:
		info = init_backup_v1 (drive, drive2, bflags);
		break;
	case bfV3:
		// Shrinking implied with v3, unless -a flag was set.
		if (threshopt != 'a')
			bflags |= BF_SHRINK;
		info = init_backup_v3 (drive, drive2, bflags);
		break;
	case bfWinMFS:
		fprintf (stderr, "%s: Backup in WinMFS format not yet supported\n", argv[0]);
		return 1;
	}
	if (!info)
	{
		fprintf (stderr, "%s: Backup failed to startup.  Make sure you specified the right\ndevices, and that the drives are not locked.\n", argv[0]);
		return 1;
	}

	// Try to continue anyway despite error.
	if (bflags & BF_TRUNCATED && backup_has_error (info))
	{
		backup_perror (info, "WARNING");
		fprintf (stderr, "Attempting backup anyway\n");
		backup_check_truncated_volume (info);

		if (backup_has_error (info))
		{
			backup_perror (info, "Backup");
			return 1;
		}
	}

	if (backup_has_error (info))
	{
		backup_perror (info, "Backup");

		fprintf (stderr, "To attempt backup anyway, try again with -E.  -s is implied by -E.\n");
		return 1;
	}
	else
	{
		unsigned char buf[BUFSIZE];
		uint64_t cursec = 0, curcount;
		int fd;

		if (filename[0] == '-' && filename[1] == '\0')
			fd = 1;
		else
#if O_LARGEFILE
			fd = open (filename, O_WRONLY | O_CREAT | O_TRUNC | O_LARGEFILE, 0644);
#else
			fd = open (filename, O_WRONLY | O_CREAT | O_TRUNC, 0644);
#endif

		if (fd < 0)
		{
			perror (filename);
			return 1;
		}

		if (threshopt)
			backup_set_thresh (info, thresh);
		if (!norescheck)
			backup_set_resource_check(info);
		if (skipdb)
			backup_set_skipdb (info, skipdb);

		if (quiet < 2)
			fprintf (stderr, "Scanning source drive.  Please wait a moment.\n");

		if (backup_start (info) < 0)
		{
			if (backup_has_error (info))
				backup_perror (info, "Backup");
			else
				fprintf (stderr, "Backup failed.\n");
			return 1;
		}

		if (quiet < 1)
			display_backup_info (info);

		if (quiet < 2)
			fprintf (stderr, "Uncompressed backup size: %" PRIu64 " MiB\n", info->nsectors / 2048);

		starttime = time(NULL);

		while ((curcount = backup_read (info, buf, BUFSIZE)) > 0)
		{
			unsigned int prcnt, compr;
			if (write (fd, buf, curcount) != curcount)
			{
				fprintf (stderr, "Backup failed: %s: %s\n", filename, strerror(errno));
				return 1;
			}
			cursec += curcount / 512;
			prcnt = get_percent (info->cursector, info->nsectors);
			compr = get_percent (info->cursector - cursec, info->cursector);
			if (quiet < 1)
			{
				unsigned timedelta = time(NULL) - starttime;
				if (compressed)
				  fprintf (stderr, "     \rBacking up %" PRId64 " of %" PRId64 " MiB (%d.%02d%%) (%d.%02d%% comp)", info->cursector / 2048, info->nsectors / 2048, prcnt / 100, prcnt % 100, compr / 100, compr % 100);
				else
					fprintf (stderr, "     \rBacking up %" PRId64 " of %" PRId64 " MiB (%d.%02d%%)", info->cursector / 2048, info->nsectors / 2048, prcnt / 100, prcnt % 100);

				if (prcnt > 10 && timedelta > 15)
				{
					unsigned ETA = timedelta * (10000 - prcnt) / prcnt;
					fprintf (stderr, " %" PRId64 " MiB/sec (ETA %d:%02d:%02d)", info->cursector / timedelta / 2048, ETA / 3600, ETA / 60 % 60, ETA % 60);
				}
			}
		}

		if (quiet < 1)
			fprintf (stderr, "\n");

		if (curcount < 0)
		{
			if (backup_has_error (info))
				backup_perror (info, "Backup");
			else
				fprintf (stderr, "Backup failed.\n");
			return 1;
		}
	}

	if (backup_finish (info) < 0)
	{
		if (backup_has_error (info))
			backup_perror (info, "Backup");
		else
			fprintf (stderr, "Backup failed.\n");
		return 1;
	}

	if (info->back_flags & BF_TRUNCATED)
		fprintf (stderr, "***WARNING***\nBackup was made of an incomplete volume.  While the backup succeeded,\nit is possible there was some required data missing.  Verify your backup.\n");
	else if (quiet < 2)
	if (quiet < 2)
	{
		unsigned tot = time(NULL) - starttime;
		fprintf (stderr, "Backup done! (%d:%02d:%02d)\n", tot / 3600, tot / 60 % 60, tot % 60);
	}

	return 0;
}
