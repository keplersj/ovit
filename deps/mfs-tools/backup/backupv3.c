#ifdef HAVE_CONFIG_H
#include <config.h>
#endif
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#if HAVE_MALLOC_H
#include <malloc.h>
#endif
#if HAVE_ERRNO_H
#include <errno.h>
#endif
#if HAVE_SYS_MALLOC_H
#include <sys/malloc.h>
#endif
#include <sys/types.h>
#ifdef HAVE_ASM_TYPES_H
#include <asm/types.h>
#endif
#include <fcntl.h>
#include <zlib.h>
#include <string.h>
#ifdef HAVE_LINUX_FS_H
#include <linux/fs.h>
#endif
#include <ctype.h>
#include <inttypes.h>

#include "mfs.h"
#include "macpart.h"
#include "backup.h"

/**************************************************************/
/* Add an inode to the list, allocating more space if needed. */
/* Keep the list in order of fsid*/
static int
backup_inode_list_add (unsigned **listinode, unsigned **listfsid, unsigned *allocated, int *total, unsigned inodeval, unsigned fsidval)
{
	int insertposmin = 0;
	int insertposmax = *total;

/* No space, (re)allocate space. */
	if (*allocated <= *total)
	{
		*allocated += 32;
		*listfsid = realloc (*listfsid, *allocated * sizeof (*listfsid));
		*listinode = realloc (*listinode, *allocated * sizeof (*listinode));
	}

/* Allocation error. */
	if (!*listfsid || !*listinode)
		return -1;

/* Search for the location this id fits in the list */
	while (insertposmin != insertposmax)
	{
		int curpos = (insertposmin + insertposmax) >> 1;
		int curval = (*listfsid)[curpos];
		if (curval < fsidval)
		{
			insertposmin = curpos + 1;
		}
		else
		{
			insertposmax = curpos;
		}
	}

/* Move all following entries after the entry */
	if (insertposmin < *total)
	{
		memmove (*listfsid + insertposmin + 1, *listfsid + insertposmin, (*total - insertposmin) * sizeof (**listfsid));
		memmove (*listinode + insertposmin + 1, *listinode + insertposmin, (*total - insertposmin) * sizeof (**listinode));
	}

	/* Add to the list size and insert the new entry */
	(*listinode)[insertposmin] = inodeval;
	(*listfsid)[insertposmin] = fsidval;
	(*total)++;

	return 0;
}

/*****************************************************************/
/* Scan the inode table and generate a list of inodes to backup. */
unsigned
backup_scan_inodes (struct backup_info *info)
{
	unsigned int loop, loop2;
	int ninodes = mfs_inode_count (info->mfs);
	uint64_t highest = 0;
	unsigned char inodebuf[512];
	unsigned *fsids = NULL;

	uint64_t appsectors = 0, mediasectors = 0, restoremediasectors = 0;
	unsigned int mediainodes = 0, appinodes = 0;

	unsigned allocated = 0;

	info->inodes = NULL;

/* Get the log type to use for inode updates */
	if (!info->mfs->inode_log_type)
	{
		info->err_msg = "Unable to determine transaction type for inode updates";
		return -1;
	}
	info->ilogtype = info->mfs->inode_log_type;

/* Add inodes. */
	for (loop = 0; loop < ninodes; loop++)
	{
		mfs_inode *inode = (mfs_inode *)inodebuf;

		int ret = mfs_read_inode_to_buf (info->mfs, loop, inode);

		if (mfs_has_error (info->mfs))
		{
			if (info->inodes)
				free (info->inodes);
			info->inodes = 0;
			return ~0;
		}

/* Don't think this should ever happen. */
		if (ret <= 0)
			continue;

/* Skip any inodes that are unallocated */
		if (!inode->fsid || !inode->refcount)
		{
			continue;
		}

/* Add the inode to the list, even if the data won't be backed up. */
		if (backup_inode_list_add (&info->inodes, &fsids, &allocated, &info->ninodes, loop, intswap32 (inode->fsid)) < 0)
		{
			info->err_msg = "Memory exhausted (Inode scan %d)";
			info->err_arg1 = (int64_t)(size_t)loop;
			if (info->inodes)
				free (info->inodes);
			if (fsids)
				free (fsids);
			info->inodes = NULL;
			return ~0;
		}

/* If it a stream, treat it specially. */
		if (inode->type == tyStream)
		{
			unsigned int streamsize;

			if (info->back_flags & (BF_THRESHTOT | BF_STREAMTOT))
				streamsize = intswap32 (inode->blocksize) / 512 * intswap32 (inode->size);
			else
				streamsize = intswap32 (inode->blocksize) / 512 * intswap32 (inode->blockused);

/* Ignore streams with no allocated data, or bigger than the threshhold. */
			if (streamsize == 0 || 
					((((info->back_flags & BF_THRESHSIZE) && streamsize > info->thresh) ||
					(!(info->back_flags & BF_THRESHSIZE) && intswap32 (inode->fsid) > info->thresh)) && !(is_resource(intswap32 (inode->fsid))) ))
			{
				/* Clear out the data in the inode and write it back to */
				/* memory for backup to read later */
				
				// Series 1 observed to reboot when playing a recording when the TyStream is skipped and size is set to 0.  So, we will set size == 1 
				// (as long as BF_STREAMTOT is not set, as that would cause restore to fail).  This should prevent unwanted reboots in most cases,
				// except, perhaps, if using -T without -a on the Series 1 (which seems unlikely)
				if (!(info->back_flags & BF_NOBSWAP) && !(info->back_flags & BF_STREAMTOT))
				{
					if (inode->size > 0)
						inode->size=intswap32 (1);
				}
				else
				{
				inode->size = 0;
				}
				inode->blockused = 0;
				inode->numblocks = 0;
				mfs_write_inode (info->mfs, inode);
				continue;
			}

/* If the total size is only for comparison, get the used size now. */
			if ((info->back_flags & (BF_THRESHTOT | BF_STREAMTOT)) == BF_THRESHTOT)
				streamsize = intswap32 (inode->blocksize) / 512 * intswap32 (inode->blockused);

/* Count the inode's sectors in the total. */
			mediasectors += streamsize;
			restoremediasectors += intswap32 (inode->blocksize) / 512 * intswap32 (inode->size);
			mediainodes++;

#if DEBUG
			fprintf (stderr, "Inode %d (%d) added\n", intswap32 (inode->inode), intswap32 (inode->fsid));
#endif
		}
		else if (inode->type != tyStream && !(inode->inode_flags & intswap32 (INODE_DATA) || inode->inode_flags & intswap32 (INODE_DATA2)) && inode->size)
		{
/* Count the space used by non-stream inodes */
			appsectors += (intswap32 (inode->size) + 511) / 512;
			appinodes++;

		}

/* Either an application data inode or a stream inode being backed up. */
		for (loop2 = 0; loop2 < intswap32 (inode->numblocks); loop2++)
		{
			uint64_t thiscount;
			uint64_t thissector;

			if (mfs_is_64bit (info->mfs))
			{
				thiscount = intswap32 (inode->datablocks.d64[loop2].count);
				thissector = sectorswap64 (inode->datablocks.d64[loop2].sector);
			}
			else
			{
				thiscount = intswap32 (inode->datablocks.d32[loop2].count);
				thissector = intswap32 (inode->datablocks.d32[loop2].sector);
			}

			if (highest < thiscount + thissector)
			{
				highest = thiscount + thissector;
			}
		}
	}

// Make sure all needed data is present.
	if (info->back_flags & BF_TRUNCATED)
	{
		uint64_t set_size = mfs_volume_set_size (info->mfs);
		if (highest > set_size)
		{
			info->err_msg = "Required data at %ld beyond end of the device (%ld)";
			info->err_arg1 = (int64_t)highest;
			info->err_arg2 = (int64_t)set_size;

			if (info->inodes)
				free (info->inodes);
			if (fsids)
				free (fsids);
			info->inodes = NULL;
			return ~0;
		}
	}

	// Record highest block to backup.
	if ((info->back_flags & BF_SHRINK) && highest > info->shrink_to)
		info->shrink_to = highest;

#if DEBUG
	fprintf (stderr, "Backing up %" PRIu64 " media sectors (%d inodes), %" PRIu64 " app sectors (%d inodes) and %d inode sectors\n", mediasectors, mediainodes, appsectors, appinodes, info->ninodes);
#endif

/* Count the space for the inodes themselves */
	info->nsectors += info->ninodes + appsectors + mediasectors;
	info->appsectors = appsectors;
	//info->mediasectors = mediasectors;
	// NOTE: We are now setting the previous unused info->mediasectors to inode->size so that we can determine the correct minimum restore size.  Backup reporting should still reflect the actual backup size.
	//       Any backups created with the actual mediasectors that were backed up (original behavior) without the -T flag will report an incorrect minimum single drive sector size during restore and
	//       will fail unless there is actually enough space on the drive to allow for the total inode->blockused.
	info->mediasectors = restoremediasectors;
	info->appinodes = appinodes;
	info->mediainodes = mediainodes;

	if (fsids)
		free (fsids);
	return info->ninodes;
}

/***************************************************************/
/* Scan the zone maps for vital stats needed to reproduce them */
int
backup_info_scan_zone_maps (struct backup_info *info)
{
	zone_header *zone = NULL;
	int cur;

	/* Start by counting the zone maps */
	while ((zone = mfs_next_zone (info->mfs, zone)) != NULL)
		info->nzones++;

	info->zonemaps = calloc (sizeof (struct zone_map_info), info->nzones);
	if (!info->zonemaps)
	{
		info->err_msg = "Memory exhausted (Zone scan)";
		return -1;
	}

	zone = NULL;
	for (cur = 0; cur < info->nzones; cur++)
	{
		unsigned int *fsmemptrs;
		unsigned int numbitmaps;
		unsigned int fsmem_addr;

		zone = mfs_next_zone (info->mfs, zone);

		if (mfs_is_64bit (info->mfs))
		{
			info->zonemaps[cur].map_length = intswap32 (zone->z64.length);
			info->zonemaps[cur].zone_type = intswap32 (zone->z64.type);
			info->zonemaps[cur].min_au = intswap32 (zone->z64.min);
			info->zonemaps[cur].size = intswap64 (zone->z64.size);
			numbitmaps = intswap32 (zone->z64.num);
			fsmemptrs = (unsigned int *)(&zone->z64 + 1);
		}
		else
		{
			info->zonemaps[cur].map_length = intswap32 (zone->z32.length);
			info->zonemaps[cur].zone_type = intswap32 (zone->z32.type);
			info->zonemaps[cur].min_au = intswap32 (zone->z32.min);
			info->zonemaps[cur].size = intswap32 (zone->z32.size);
			numbitmaps = intswap32 (zone->z32.num);
			fsmemptrs = (unsigned int *)(&zone->z32 + 1);
		}

		fsmem_addr = intswap32 (*fsmemptrs);
		/* Subtract the size of the zone map header from the fsmem address */
		fsmem_addr -= (unsigned char *)fsmemptrs - (unsigned char *)zone;
		/* Subtract the space for the bitmap pointers fromt he fsmem address */
		fsmem_addr -= numbitmaps * 4;
		info->zonemaps[cur].fsmem_base = fsmem_addr;
	}

	return 0;
}

/******************************************/
/* Add some named data to the backup info */
void
backup_info_add_extra (struct backup_info *info, char *type, void *data, int datalength)
{
	int extrainfosize = offsetof (struct extrainfo, data);
	extrainfosize += (strlen (type) + 3) & ~3;
	extrainfosize += (datalength + 3) &~3;

	info->extrainfo = realloc (info->extrainfo, sizeof (*info->extrainfo) * (info->nextrainfo + 1));
	info->extrainfo[info->nextrainfo] = calloc (extrainfosize, 1);
	info->extrainfo[info->nextrainfo]->typelength = strlen (type);
	info->extrainfo[info->nextrainfo]->datalength = datalength;
	memcpy (&info->extrainfo[info->nextrainfo]->data[0], type, strlen (type));
	memcpy (&info->extrainfo[info->nextrainfo]->data[(strlen (type) + 3) & ~3], data, datalength);
	info->nextrainfo++;
	info->extrainfosize += extrainfosize;
}

/*****************************************/
/* Add a named string to the backup info */
void
backup_info_add_extra_string (struct backup_info *info, char *type, char *data)
{
	backup_info_add_extra (info, type, data, strlen (data) + 1);
}

/**************************************************************/
/* Count the sectors of various items not scanned during init */
int
backup_info_count_misc (struct backup_info *info)
{
// Boot sector
	info->nsectors++;
// Volume header
	info->nsectors++;
// Checksum
	info->nsectors++;

	return 0;
}

/*************************************/
/* Initializes the backup structure. */
struct backup_info *
init_backup_v3 (char *device, char *device2, int flags)
{
 	struct backup_info *info;

 	if (!device)
 		return 0;
 
 	info = calloc (sizeof (*info), 1);
 
 	if (!info)
 	{
 		return 0;
 	}

	info->format = bfV3;

	info->crc = ~0;
	info->state_machine = &backup_v3;

	info->mfs = mfs_init (device, device2, (O_RDONLY | MFS_ERROROK));
 
 	info->back_flags = flags;

	// MFS is little endian
	if (mfsLSB == 1)
	{
		info->back_flags |= BF_MFSLSB;
	}
	
	// TiVo partitions are little endian
	if (partLSB == 1)
	{
		info->back_flags |= BF_PARTLSB;
	}

	if (info->mfs && mfs_is_64bit (info->mfs))
	{
		info->back_flags |= BF_64;
	}

	// This appears to be an arbitrary number to pickup misc tyStream files (loopset, demo, etc.), which doesn't work very well after a unit is in service for a while,
	// then has it's loopsets updated. This job is now handled by backup_set_resource_check, so don't bother with a minimum fsid here...
	//info->thresh = 2000;

	if (!tivo_partition_swabbed (device))
		info->back_flags |= BF_NOBSWAP;

	info->hda = strdup (device);
	if (!info->hda)
	{
		info->err_msg = "Memory exhausted";
	}

	if (!mfs_has_error (info->mfs))
	{
		info->back_flags &= ~BF_TRUNCATED;
	}
 
	return info;
}

/***************************************************************************/
/* State handlers - return val -1 = error, 0 = more data needed, 1 = go to */
/* next state. */

/***********************************/
/* Generic handler for header data */
enum backup_state_ret
backup_write_header (struct backup_info *info, void *data, unsigned size, unsigned *consumed, void *src, unsigned total, unsigned datasize);
/* Defined in backup.c */

/**************************/
/* Scan MFS for v3 backup */
/* state_val1 = --unused-- */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = --unused-- */
enum backup_state_ret
backup_state_scan_mfs_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	int loop;
	char buf[1024];

	if ((add_partitions_to_backup_info (info, info->hda)) != 0) {
		return bsError;
	}

	// Loop through the partitions and save the name/type in extra info
	for (loop = 0; loop < info->nparts; loop++)
	{
		char *ptype = tivo_partition_type(info->hda, info->parts[loop].partno);
		char *pname = tivo_partition_name(info->hda, info->parts[loop].partno);

		sprintf (buf, "pname%d", info->parts[loop].partno);
		backup_info_add_extra_string (info, buf, pname);

		sprintf (buf, "ptype%d", info->parts[loop].partno);
		backup_info_add_extra_string (info, buf, ptype);
	}

	if (backup_scan_inodes (info) == ~0)
	{
		free (info->parts);
		return bsError;
	}

	if (add_mfs_partitions_to_backup_info (info) != 0) {
		free (info->parts);
		free (info->inodes);
		return bsError;
	}

	if (backup_info_scan_zone_maps (info) != 0)
	{
		free (info->parts);
		free (info->inodes);
		free (info->mfs);
		return bsError;
	}

	if (backup_info_count_misc (info) != 0)
	{
		free (info->parts);
		free (info->inodes);
		free (info->zonemaps);
		free (info->mfs);
		return bsError;
	}

	info->nsectors += (
		info->nparts * sizeof (struct backup_partition) +
		info->nmfs * sizeof (struct backup_partition) +
		info->nzones * sizeof (struct zone_map_info) +
		info->extrainfosize +
		sizeof (struct backup_head_v3) +
		511) / 512;

	return bsNextState;
}

/*******************/
/* Begin v3 backup */
/* state_val1 = --unused-- */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = next offset to use in block */
enum backup_state_ret
backup_state_begin_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	struct backup_head_v3 *head = data;

	if (size == 0)
	{
		info->err_msg = "Internal error: Backup buffer full";
		return bsError;
	}

	head->magic = TB3_MAGIC;
	head->flags = info->back_flags;
	head->nsectors = info->nsectors;
	head->nparts = info->nparts;
	head->nzones = info->nzones;
	head->mfspairs = info->nmfs;
	head->appsectors = info->appsectors;
	head->mediasectors = info->mediasectors;
	head->appinodes = info->appinodes;
	head->mediainodes = info->mediainodes;
	head->ilogtype = info->ilogtype;
	head->ninodes = info->ninodes;
	head->nextra = info->nextrainfo;
	head->extrasize = info->extrainfosize;
	head->size = sizeof (*head);

	info->shared_val1 = (sizeof (*head) + 7) & (512 - 8);
	*consumed = 0;

	return bsNextState;
}

/*********************************/
/* Add partition info to backup. */
/* state_val1 = index of last copied partition */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = next offset to use in block */
enum backup_state_ret
backup_state_info_partitions (struct backup_info *info, void *data, unsigned size, unsigned *consumed);
/* Defined in backup.c */

/*************************************/
/* Add MFS partition info to backup. */
/* state_val1 = index of last copied MFS partition */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = next offset to use in block */
enum backup_state_ret
backup_state_info_mfs_partitions (struct backup_info *info, void *data, unsigned size, unsigned *consumed);
/* Defined in backup.c */

/********************************/
/* Add zone map info to backup. */
/* state_val1 = offset of last copied zone map */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = next offset to use in block */
enum backup_state_ret
backup_state_info_zone_maps_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	return backup_write_header (info, data, size, consumed, info->zonemaps, info->nzones, sizeof (struct zone_map_info));
}

/********************************/
/* Add info_extra to backup.    */
/* state_val1 = offset in current info */
/* state_val2 = current info index */
/* state_ptr1 = --unused-- */
/* shared_val1 = next offset to use in block */
enum backup_state_ret
backup_state_info_extra_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	while (info->state_val2 < info->nextrainfo && *consumed < size)
	{
		int extrainfosize = offsetof (struct extrainfo, data);
		extrainfosize += (info->extrainfo[info->state_val2]->typelength + 3) & ~3;
		extrainfosize += (info->extrainfo[info->state_val2]->datalength + 3) & ~3;

		enum backup_state_ret ret = backup_write_header (info, data, size, consumed, info->extrainfo[info->state_val2], 1, extrainfosize);

		if (ret != bsNextState)
			return ret;

		info->state_val1 = 0;
		info->state_val2++;

	if (info->state_val2 < info->nextrainfo)
		return bsMoreData;
	}

	return bsNextState;
}

/********************************/
/* Finish off the backup header */
/* state_val1 = --unused-- */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = next offset to use in block */
enum backup_state_ret
backup_state_info_end (struct backup_info *info, void *data, unsigned size, unsigned *consumed);
/* Defined in backup.c */

/*************************/
/* Backup the boot block */
/* state_val1 = --unused-- */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = --unused-- */
enum backup_state_ret
backup_state_boot_block (struct backup_info *info, void *data, unsigned size, unsigned *consumed);
/* Defined in backup.c */

/***************************************/
/* Backup the raw (non-MFS) partitions */
/* state_val1 = current partition index */
/* state_val2 = offset within current partition */
/* state_ptr1 = --unused-- */
/* shared_val1 = --unused-- */
enum backup_state_ret
backup_state_partitions (struct backup_info *info, void *data, unsigned size, unsigned *consumed);
/* Defined in backup.c */

/****************************/
/* Backup the volume header */
/* state_val1 = --unused-- */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = --unused-- */
enum backup_state_ret
backup_state_volume_header_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	if (size == 0)
	{
		info->err_msg = "Internal error: Backup buffer full";
		return bsError;
	}

	memcpy (data, &info->mfs->vol_hdr, sizeof (info->mfs->vol_hdr));
	memset ((char *)data + sizeof (info->mfs->vol_hdr), 0, 512 - sizeof (info->mfs->vol_hdr));

	*consumed = 1;

	return bsNextState;
}

/*****************************/
/* Backup application inodes */
/* Write inode sector, followed by date for non tyStream inodes. */
/* state_val1 = current inode index */
/* state_val2 = offset of data in current inode */
/* state_ptr1 = current inode structure */
/* shared_val1 = --unused-- */
enum backup_state_ret
backup_state_inodes_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	mfs_inode *inode;

	if (size == 0)
	{
		info->err_msg = "Internal error: Backup buffer full";
		return bsError;
	}

	while (info->state_val1 < info->ninodes && size > 0)
	{
		uint64_t datasize;

		if (!info->state_ptr1)
		{
			mfs_inode *tmpinode;
/* Load the next inode to backup */
			uint64_t inode_size;

/* Fetch the next inode */
			inode = mfs_read_inode (info->mfs, info->inodes[info->state_val1]);

			if (!inode)
			{
				return bsError;
			}

			inode_size = offsetof (mfs_inode, datablocks);

/* Data in inode. */
			if (inode->type != tyStream && (inode->inode_flags & intswap32 (INODE_DATA) || inode->inode_flags & intswap32 (INODE_DATA2)))
			{
				inode_size += intswap32 (inode->size);
				if (inode_size > 512)
					inode_size = 512;
			}

/* Zeros compress easier, so might as well eliminate any unneeded data. */
			memcpy (data, inode, inode_size);
			if (inode_size < 512)
				memset ((char *)data + inode_size, 0, 512 - inode_size);

/* Clear out a few values from the backed up structure */
/* All these values will be re-assigned on restore */
			tmpinode = (mfs_inode *)data;
			tmpinode->inode = ~0;
			tmpinode->bootcycles = 0;
			tmpinode->bootsecs = 0;
			tmpinode->sig = 0;
			tmpinode->checksum = 0;
			if (mfsLSB)
				tmpinode->inode_flags &= intswap32 (INODE_DATA2);
			else
			tmpinode->inode_flags &= intswap32 (INODE_DATA);
			tmpinode->numblocks = 0;

			data = (char *)data + 512;
			--size;
			++*consumed;

			info->state_val2 = 0;
			info->state_ptr1 = inode;
		}
		else
		{
			inode = info->state_ptr1;
		}

		if (inode->type == tyStream)
		{
			if (info->back_flags & BF_STREAMTOT)
				datasize = intswap32 (inode->size);
			else
				datasize = intswap32 (inode->blockused);
			datasize *= intswap32 (inode->blocksize);
		}
		else
		{
			if (!(inode->inode_flags & intswap32 (INODE_DATA) || inode->inode_flags & intswap32 (INODE_DATA2)))
			{
				datasize = intswap32 (inode->size);
			}
			else
			{
				datasize = 0;
			}
		}

		while (info->state_val2 * 512 < datasize)
		{
			uint64_t tocopy = datasize - info->state_val2 * 512;
			if ((tocopy + 511) / 512 > size)
				tocopy = size * 512;

			if (!tocopy)
				return bsMoreData;

			if (mfs_read_inode_data_part (info->mfs, inode, data, info->state_val2, (tocopy + 511) / 512) < 0)
			{
				info->err_msg = "Error reading inode %d";
				info->err_arg1 = (int64_t)info->state_val1;
				free (inode);
				return bsError;
			}

/* Once again, zeros compress well, so zero out any garbage data. */
			if ((tocopy & 511) > 0)
			{
				memset ((char *)data + tocopy, 0, 512 - (tocopy & 511));
			}

/* Update the sizes */
			tocopy = (tocopy + 511) / 512;
			data = (char *)data + tocopy * 512;
			size -= tocopy;
			info->state_val2 += tocopy;
			*consumed += tocopy;
			if (inode->type != tyStream)
				info->shared_val1 += tocopy;
		}

/* If it exits this loop, it means this inode is done, move onto the next */
		free (inode);
		info->state_ptr1 = NULL;
		info->state_val1++;
		info->state_val2 = 0;
	}

/* If it exits this loop, it means all the inodes are done, */
/*  or it's out of data */

	if (info->state_val1 < info->ninodes)
		return bsMoreData;

	info->shared_val1 = 0;
	return bsNextState;
}

/*****************/
/* Finish backup */
/* For v3 backup, store CRC32 at end of the backup. */
/* state_val1 = --unused-- */
/* state_val2 = --unused-- */
/* state_ptr1 = --unused-- */
/* shared_val1 = --unused-- */
enum backup_state_ret
backup_state_complete_v3 (struct backup_info *info, void *data, unsigned size, unsigned *consumed)
{
	if (size == 0)
	{
		info->err_msg = "Internal error: Backup buffer full";
		return bsError;
	}

	memset (data, 0, 512);
	info->crc = compute_crc (data, 512 - sizeof (unsigned int), info->crc);
	*(unsigned int *)((char *)data + 512 - sizeof (unsigned int)) = ~info->crc;

	*consumed = 1;

#ifdef DEBUG
	if (info->nsectors != info->cursector + 1)
	{
		fprintf (stderr, "nsectors %d != cursector + 1 %d\n", info->nsectors, info->cursector);
	}
#endif

#if HAVE_SYNC
/* Make sure changes are committed to disk */
	sync ();
#endif

	return bsNextState;
}

backup_state_handler backup_v3 = {
	backup_state_scan_mfs_v3,				// bsScanMFS
	backup_state_begin_v3,					// bsBegin
	backup_state_info_partitions,			// bsInfoPartition
	NULL,									// bsInfoBlocks
	backup_state_info_mfs_partitions,		// bsInfoMFSPartitions
	backup_state_info_zone_maps_v3,			// bsInfoZoneMaps
	backup_state_info_extra_v3,				// bsInfoExtra
	backup_state_info_end,					// bsInfoEnd
	backup_state_boot_block,				// bsBootBlock
	backup_state_partitions,				// bsPartitions
	NULL,									// bsMFSInit
	NULL,									// bsBlocks
	backup_state_volume_header_v3,			// bsVolumeHeader
	NULL,									// bsTransactionLog
	NULL,									// bsUnkRegion
	NULL,									// bsMfsReinit
	backup_state_inodes_v3,					// bsInodes
	backup_state_complete_v3				// bsComplete
};
