/* play a video given a blkmap file

   tridge@samba.org, December 2000

   released under the Gnu General Public License version 2 or later
*/

#include "mfs.h"

//
// usage - Print out the usage as seperate lines. Save the deprecation warnings.
//
void usage(void)
{
	printf("vserver_mfs [options]\n");
	printf("Version: 1.3-20050428\n");
	printf("options:\n");
	printf("    -l  Send messages to syslog\n");
	printf("    -L  Send all messages to syslog\n");
	printf("    -i  inetd mode. Use this in inetd.conf.\n");
	printf("    -h  help\n");
	credits();
	exit(1);
}

#include "log.h"

#define CHUNK_SIZE	(256*1024)
#define CHUNK_BLOCKS	(CHUNK_SIZE >> SECTOR_SHIFT)

char buf[CHUNK_SIZE];

static void lock_byte(int fd, int b)
{
	struct flock lock;
	lock.l_type = F_WRLCK;
	lock.l_whence = SEEK_SET;
	lock.l_start = b;
	lock.l_len = 1;
	lock.l_pid = getpid();

	if (fcntl(fd,F_SETLKW,&lock) != 0) {
		fprintf(stderr,"ERROR: Failed to get lock at %d\n", b);
		exit(0);
	}
}

static void unlock_byte(int fd, int b)
{
	struct flock lock;
	lock.l_type = F_WRLCK;
	lock.l_whence = SEEK_SET;
	lock.l_start = b;
	lock.l_len = 1;
	lock.l_pid = getpid();

	if (fcntl(fd,F_UNLCK,&lock) != 0) {
		fprintf(stderr,"ERROR: Failed to unlock at %d\n", b);
		exit(0);
	}
}

#ifndef LOCK_READ
#define LOCK_READ 0
#endif

#ifndef LOCK_WRITE
#define LOCK_WRITE 1
#endif

static int vserver(int fd)
{
	unsigned blk, n, nblk, blks;
	struct vserver_cmd cmd;
	int window = 32768;
	char fname[40];
	int lock_fd;
	run_desc retval;

	void lock(int type, int v) {
		if (v) {
			lock_byte(lock_fd, type);
		} else {
			unlock_byte(lock_fd, type);
		}
	}

	sprintf(fname,"/tmp/vserver.%d", getpid());
	lock_fd = open(fname, O_RDWR|O_CREAT, 0600);
	unlink(fname);
	if (lock_fd == -1) {
		perror(fname);
		exit(0);
	}

	setsockopt(fd, SOL_SOCKET, SO_SNDBUF, &window, sizeof(int));

	fork();

	for (;;) {
		lock(LOCK_READ, 1);
		read_all(fd, &cmd, sizeof(cmd));

		switch(cmd.command) {
		case MFS_CMD_READ:
			lock(LOCK_WRITE, 1);
			lock(LOCK_READ, 0);
			blk = ntohl(cmd.param1);
			nblk = ntohl(cmd.param2);
			for (n = nblk; n > 0; n -= blks) {
				blks = n;
				// printf("read request for 0x%x/%d\n", blk, blks);
				if (blks > CHUNK_BLOCKS) blks = CHUNK_BLOCKS;
				mfs_read_sectors(buf, blk, blks);
				write_all(fd, buf, blks << SECTOR_SHIFT);
				blk += blks;
			}
			break;

		case MFS_CMD_LIST_SECTORS:
			lock(LOCK_WRITE, 1);
			lock(LOCK_READ, 0);
			blk = htonl(cmd.param1);
			nblk = htonl(cmd.param2);
			retval = mfs_list_sectors( blk, nblk);
			retval.drive = ntohl(retval.drive);
			retval.partition = ntohl(retval.partition);
			retval.start = ntohl(retval.start);
			retval.count = ntohl(retval.count );
			write_all(fd, &retval, sizeof(retval) );
			break;
		
		case MFS_CMD_WRITE:
			blk = ntohl(cmd.param1);
			nblk = ntohl(cmd.param2);
			for (n = nblk; n > 0; n -= blks) {
				blks = n;
				// printf("write request for 0x%x/%d\n", blk, blks);
				if (blks > CHUNK_BLOCKS) blks = CHUNK_BLOCKS;
				read_all(fd, buf, blks << SECTOR_SHIFT);
				// printf("\nwriting to sector %d (%d blks)\n",
				//       blk, blks);
				// dump_sectors(buf, blks);
				mfs_write_sectors(buf, blk, blks);
				blk += blks;
			}
			break;

		case MFS_CMD_ZERO:
			lock(LOCK_READ, 0);
			blk = ntohl(cmd.param1);
			nblk = ntohl(cmd.param2);
			memset(buf, 0, sizeof(buf));
			for (n = nblk; n > 0; n -= blks) {
				blks = n;
				if (blks > CHUNK_BLOCKS) blks = CHUNK_BLOCKS;
				mfs_write_sectors(buf, blk, blks);
				blk += blks;
			}
			break;

		default:
			printf("Unknown command %d!\n", cmd.command);
			exit(1);
		}
		lock(LOCK_READ, 0);
		lock(LOCK_WRITE, 0);
	}
	_exit(0);
}

/****************************************************************************
open a socket of the specified type, port and address for incoming data
****************************************************************************/
static int open_socket_in(int type, int port, unsigned socket_addr)
{
	struct sockaddr_in sock;
	int res;
	int one=1;

	bzero((char *)&sock,sizeof(sock));

	sock.sin_port = htons(port);
	sock.sin_family = AF_INET;
	sock.sin_addr.s_addr = socket_addr;
	res = socket(AF_INET, type, 0);
	if (res == -1) { 
		fprintf(stderr, "socket failed\n"); return -1; 
	}
	setsockopt(res,SOL_SOCKET,SO_REUSEADDR,(char *)&one,sizeof(one));
	if (bind(res, (struct sockaddr * ) &sock,sizeof(sock)) < 0) { 
		return(-1); 
	}

	return res;
}

 int main(int argc, char *argv[])
{
	int sock;
	int c, inetd = 0;

	while ((c = getopt(argc, argv, "lLih?")) != -1)
	{
		switch (c)
		{
			case 'l':	setup_syslog(1); break;
			case 'L':	setup_syslog(2); break;
			case 'i':	inetd=1;	break;
			default:	usage();	exit(1);
		}
	}
	
	argc -= optind; argv += optind;

	if (argc != 0)
	{
		usage();
	}

	if (inetd) {
		// If started by inetd, stdin+stdout+stderr are all connected
		// to the socket. Here we keep just stdin (fd=0).
		int logfd;
		logfd= open("/dev/null", O_WRONLY|O_CREAT|O_TRUNC, 0644);
		if (logfd==-1) exit(2);
		dup2(logfd,1);
		close(logfd);
		dup2(1,2);
		sock = dup(0);
		close(0);

		signal(SIGCHLD, SIG_IGN);
		mfs_init();
		logmsg("starting vserver\n");
		vserver(sock);
		return 0;
	}

	sock = open_socket_in(SOCK_STREAM, VSERVER_PORT, INADDR_ANY);

	if (listen(sock, 5) == -1) {
		fprintf(stderr,"listen failed\n");
		exit(1);
	}

	signal(SIGCHLD, SIG_IGN);

	mfs_init();

	logmsg("waiting for connections on port %d\n", VSERVER_PORT);

	while (1) {
		struct sockaddr addr;
		unsigned in_addrlen = sizeof(addr);
		int fd;

		fd = accept(sock,&addr,&in_addrlen);

		if (fd != -1) {
			if (fork() == 0) {
				close(sock);
				vserver(fd);
			}
			close(fd);
		}
	}
	return 0;
}
