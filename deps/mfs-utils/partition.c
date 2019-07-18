/*
  media-filesystem library, partition routines
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"
#include "log.h"

#define MAX_PARTITIONS 20
#define PARTITION_MAGIC	0x504d

struct tivo_partition {
	u16	signature;	/* expected to be PARTITION_MAGIC */
	u16	res1;
	u32	map_count;	/* # blocks in partition map */
	u32	start_block;	/* absolute starting block # of partition */
	u32	block_count;	/* number of blocks in partition */
	char	name[32];	/* partition name */
	char	type[32];	/* string type description */
	u32	data_start;	/* rel block # of first data block */
	u32	data_count;	/* number of data blocks */
	u32	status;		/* partition status bits */
	u32	boot_start;
	u32	boot_size;
	u32	boot_load;
	u32	boot_load2;
	u32	boot_entry;
	u32	boot_entry2;
	u32	boot_cksum;
	char	processor[16];	/* identifies ISA of boot */
	/* there is more stuff after this that we don't need */
};

struct pmap {
	u32 start;
	u32 length;
} pmaps[MAX_PARTITIONS];

static int num_partitions;
static int use_ptable = 0;

/* parse the tivo partition table */
void partition_parse()
{
	char buf[SECTOR_SIZE];
	struct tivo_partition *tp;
	int i, count, dev_no; /*JPB*/

	tp = (struct tivo_partition *)buf;

	//	read_sectors(fd, tp, 1, 1);
	//	count = ntohl(tp->map_count);

	/*JPB Handle multiple disks*/
	for (dev_no = 0; dev_no < num_devs(); ++dev_no) {
		u32 offset = dev_start_sector(dev_no);
		mfs_read_sectors(tp, offset+1, 1);
		count = ntohl(tp->map_count);
		//		fprintf( stderr, "dev: %d    offset: %d  count: %d\n", 
		//			 dev_no, offset, count );

		for (i=0;i<count;i++) {
			mfs_read_sectors(tp, offset+i+1, 1);
			if (ntohs(tp->signature) != PARTITION_MAGIC) {
				fprintf(stderr, 
				       "wrong magic %x in partition %d\n",
				       ntohs(tp->signature), i);
				exit(1);
			}
			if (strcmp(tp->type, "MFS") == 0) {
				pmaps[num_partitions].start =
				    ntohl(tp->start_block)+offset;
				pmaps[num_partitions].length =
				    ntohl(tp->block_count);
				num_partitions++;
			}

		}
		//		fprintf(stderr, "dev: %d  Found %d MFS partitions count: %d\n", 
		//			dev_no, num_partitions,count);

	}
	//	fprintf(stderr, "Found %d MFS partitions\n", num_partitions);
	use_ptable = 1;
}

/* map a mfs sector number to a absolute sector number */
u32 partition_remap(u32 sec)
{
	int i;
	u32 start=0;
	u32 len;

	if (!use_ptable) return sec;

	for (i=0; i<num_partitions; i++) {
		len = (pmaps[i].length & ~(MFS_BLOCK_ROUND-1));
		if (sec < start + len) {
			// fprintf(stderr, "remapped %d to %d\n",
			//       sec, pmaps[i].start + (sec - start));
			return pmaps[i].start + (sec - start);
		}
		start += len;
	}
	if (i == num_partitions) {
		fprintf(stderr,"Failed to partition map sector %d\n", sec);
		exit(1);
	}
	return sec;
}

u32 partition_total_size(void)
{
	u32 total=0;
	int i;

	if (!use_ptable) return 0;

	for (i=0; i<num_partitions; i++) {
		total += (pmaps[i].length & ~(MFS_BLOCK_ROUND-1));
	}

	return total;
}

void clear_use_ptable()
{
	use_ptable = 0;
}
