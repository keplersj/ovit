/*
  media-filesystem library
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

#include "mfs.h"
#include "log.h"
#include <unistd.h>
#include <netinet/tcp.h>

#ifdef TIVO_S1
#include <asm/unistd.h>
#endif /* TIVO_S1 */

#ifdef TIVO_S2
#include <linux/tivo-tagio.h>
#include <linux/tivo-scramble.h>
#include <linux/ide-tivo.h>
#include <linux/hdreg.h>
#endif /* TIVO_S2 */

int scramble_present = 0;
u8 scramble_key[16];
int fs_inconsistent = 0;

void read_all(int fd, void *buf, int size)
{
	while (size) {
		int n = read(fd, buf, size);
		if (n <= 0) {
			// fprintf(stderr,"ERROR: eof in read_all\n");
			exit(1);
		}
		buf += n;
		size -= n;
	}
}

void write_all(int fd, void *buf, int size)
{
	while (size) {
		int n = write(fd, buf, size);
		if (n <= 0) {
			// fprintf(stderr,"ERROR: eof in write_all\n");
			exit(1);
		}
		buf += n;
		size -= n;
	}
}

/* open a socket to a tcp remote host with the specified port 
   based on code from Warren */
int open_socket_out(char *host, int port)
{
	int type = SOCK_STREAM;
	struct sockaddr_in sock_out;
	int res;
	struct hostent *hp;  
	int nodelay = 1;
	int window = 65535;

	res = socket(PF_INET, type, 0);
	if (res == -1) {
		return -1;
	}

	hp = gethostbyname(host);
	if (!hp) {
		fprintf(stderr,"unknown host: %s\n", host);
		return -1;
	}

	memcpy(&sock_out.sin_addr, hp->h_addr, hp->h_length);
	sock_out.sin_port = htons(port);
	sock_out.sin_family = PF_INET;

	if (connect(res,(struct sockaddr *)&sock_out,sizeof(sock_out))) {
		close(res);
		fprintf(stderr,"failed to connect to %s - %s\n", 
			host, strerror(errno));
		return -1;
	}

	setsockopt(res, SOL_SOCKET, SO_RCVBUF, &window, sizeof(int));
	setsockopt(res, IPPROTO_TCP, TCP_NODELAY, &nodelay, sizeof(int));

	return res;
}

void byte_swap(void *p, char *desc)
{
	int n, i;
	while (*desc) {
		switch (*desc) {
		case 'i': {
			u32 *v;
			n = strtol(desc+1, &desc, 10);
			v = p;
			for (i=0;i<n;i++) v[i] = ntohl(v[i]);
			p = (void *)(v+n);
			break;
		}

		case 's': {
			u16 *v;
			n = strtol(desc+1, &desc, 10);
			v = p;
			for (i=0;i<n;i++) v[i] = ntohs(v[i]);
			p = (void *)(v+n);
			break;
		}

		case 'b': {
			n = strtol(desc+1, &desc, 10);
			p += n;
			break;
		}
		}
		while (*desc == ' ') desc++;
	}
}


u64 ll_seek(int fd, u64 offset, int whence)
{
	u64 result;
	int ret = 0;
#ifdef TIVO_S1
	ret = syscall(__NR__llseek, fd, (u32)(offset>>32),
		(u32)(offset&0xffffffff), &result, whence);
#else /* TIVO_S1 */
	result = lseek(fd, offset, whence);
#endif /* TIVO_S1 */
	if(ret || (result == (off_t)-1))
	{
		fprintf(stderr,"llseek failed\n");
		exit(1);
	}
	return result;
}

u32 read_sectors(int fd, void *buf, u32 sector, u32 count)
{
#ifdef TIVO_S1
	struct FsIovec vec;
	struct FsIoRequest req;

	vec.pb = buf;
	vec.cb = count*SECTOR_SIZE;
	req.sector = sector;
	req.num_sectors = count;
	req.deadline = 0;
	
	return syscall(__NR_readsectors, fd, &vec, 1, &req, scramble_present);
#elif TIVO_S2
	tivotag tagtuple[TIVOTAGIO_NUMTAGS];
	struct FsIovec vec;
	struct io_scramble scram;

	int i = 0;

	vec.pb = buf;
	vec.cb = count*SECTOR_SIZE;

	tagtuple[i].tag = TIVOTAGIO_CMD;	/* CMD = READ */
	tagtuple[i].val = 0;
	tagtuple[i++].size = 0;
	tagtuple[i].tag = TIVOTAGIO_IOVEC;	/* list of buffers */
	tagtuple[i].val = (long)&vec;
	tagtuple[i++].size = sizeof(struct FsIovec);
	tagtuple[i].tag = TIVOTAGIO_SECTOR;	/* starting sector */
	tagtuple[i].val = sector;
	tagtuple[i++].size = 0;
	tagtuple[i].tag = TIVOTAGIO_NRSECTORS;	/* # of sectors */
	tagtuple[i].val = count;
	tagtuple[i++].size = 0;

	if(scramble_present)
	{
		scram.magic = IO_SCRAMBLE_MAGIC;
		scram.version = IO_SCRAMBLE_VERSION_TIVO1;
		scram.sectorsPerTransfer = count;
		scram.recordOffset = 0;
		memcpy(&scram.config.wyrd[0], scramble_key, 16);
		tagtuple[i].tag = TIVOTAGIO_SCRAMBLER;
		tagtuple[i].val = (long)&scram;
		tagtuple[i++].size = 0;
	}

	tagtuple[i].tag = TIVOTAGIO_END;	/* end tags */
	tagtuple[i].val = 0;
	tagtuple[i++].size = 0;

	return(ioctl(fd, HDIO_DRIVE_TIVO_IO, (void *)&tagtuple));
#else
	ll_seek(fd, ((u64)sector)<<SECTOR_SHIFT, SEEK_SET);
	return read(fd, buf, count * SECTOR_SIZE)>>SECTOR_SHIFT;
#endif
}

u32 write_sectors(int fd, void *buf, u32 sector, u32 count)
{
	if(fs_inconsistent)
	{
		fprintf(stderr, "Filesystem is inconsistent, cannot write\n");
		exit(1);
	}
	{
#ifdef TIVO_S1
	struct FsIovec vec;
	struct FsIoRequest req;

	vec.pb = buf;
	vec.cb = count*SECTOR_SIZE;
	req.sector = sector;
	req.num_sectors = count;
	req.deadline = 0;
	
	return syscall(__NR_writesectors, fd, &vec, 1, &req);
#elif TIVO_S2
	tivotag tagtuple[TIVOTAGIO_NUMTAGS];
	struct FsIovec vec;
	struct io_scramble scram;

	int i = 0;

	vec.pb = buf;
	vec.cb = count*SECTOR_SIZE;

	tagtuple[i].tag = TIVOTAGIO_CMD;	/* CMD = READ */
	tagtuple[i].val = 1;
	tagtuple[i++].size = 0;
	tagtuple[i].tag = TIVOTAGIO_IOVEC;	/* list of buffers */
	tagtuple[i].val = (long)&vec;
	tagtuple[i++].size = sizeof(struct FsIovec);
	tagtuple[i].tag = TIVOTAGIO_SECTOR;	/* starting sector */
	tagtuple[i].val = sector;
	tagtuple[i++].size = 0;
	tagtuple[i].tag = TIVOTAGIO_NRSECTORS;	/* # of sectors */
	tagtuple[i].val = count;
	tagtuple[i++].size = 0;
	
	if(scramble_present)
	{
		scram.magic = IO_SCRAMBLE_MAGIC;
		scram.version = IO_SCRAMBLE_VERSION_TIVO1;
		scram.sectorsPerTransfer = count;
		scram.recordOffset = 0;
		memcpy(&scram.config.wyrd[0], scramble_key, 16);
		tagtuple[i].tag = TIVOTAGIO_SCRAMBLER;
		tagtuple[i].val = (long)&scram;
		tagtuple[i++].size = 0;
	}

	tagtuple[i].tag = TIVOTAGIO_END;	/* end tags */
	tagtuple[i].val = 0;
	tagtuple[i++].size = 0;

	return(ioctl(fd, HDIO_DRIVE_TIVO_IO, (void *)&tagtuple));
#else
	ll_seek(fd, ((u64)sector)<<SECTOR_SHIFT, SEEK_SET);
	return write(fd, buf, count * SECTOR_SIZE)>>SECTOR_SHIFT;
#endif
	}
}


/****************************************************************************
Set a fd into nonblocking mode
****************************************************************************/
void set_nonblocking(int fd)
{
	int val;

	if((val = fcntl(fd, F_GETFL, 0)) == -1)
		return;
	if (!(val & O_NONBLOCK)) {
		val |= O_NONBLOCK;
		fcntl(fd, F_SETFL, val);
	}
}

int bitcount32(u32 x)
{
	int count;
	for (count=0; x; count++)
		x &= (x-1);
	return count;
}
