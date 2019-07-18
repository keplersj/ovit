#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <assert.h>
#include <sys/types.h>
#include <sys/wait.h>

//
//
// From the MFS_stream code. We are now using this as it seems to work better.
//
#include "mfs.h"

//
// usage - Print out the usage as seperate lines. Save the deprecation warnings.
//
void usage(const char*progname)
{
	fprintf(stderr, "\n\
usage: %s [options]\n\
Version: mfs-utils %s\n\
options:\n\
   -d <path> DeleteShowing tivosh script to use instead of the internal script\n\
   -h  help\n\
   -i  inetd mode. Use this in inetd.conf.\n\
   -l  Send messages to syslog\n\
   -L  Send all messages to syslog\n\
   -n  No Priority Change (Stay at higher priority...)\n\
   -r <ms>  Rate control (throttle)\n\
              -'ve  : no delay (default)\n\
              0     : sched_yield() between chunks\n\
              +'ve  : # of ms to delay between chunks\n\
   -s <path> NowShowing tivosh script to use instead of the internal code.\n\
             Also used for DeleteScript, unless overriden with -d\n\
", progname, BUILD_DATE);
	credits();
	exit(1);
}

#include "log.h"

int verbose = 0;

int	SetupClientSocket(char *Servername, int ServerPort);
int waitio (const int socket, const int secs);


// DEFINES
//
#define	CMD_SIZE		1024


//
// Globals
//
static int	sd;							// Original socket descriptor

char *script_NowShowing = 0;
char *script_DeleteShowing = 0;

//
// onintr - Our custom interrupt handler, shut down as cleanly as possible.
//
void onintr(int signum)							// These routine will be called on a ^C
{
	close(sd);						// Close the socket.
	exit(0);
}


//
// write_error - A handler for the SigPipe interrupt.
//
void write_error(int signum)
{
	printf("Attempted to write to a broken socket\n");
}


//
// myAtoi - A custom AtoI function... Saw some wierd behaviour with the standard one.
//
int myAtoi(char *s)
{
	long r = 0;

	while (*s && *s >= '0' && *s <= '9')
	{
		r = 10 * r + *s++ - '0';
	}

	return(r);
}




static void 
run_script(const char *script, int ns) 
{
	FILE *f = popen(script, "r");
	char buf[256];

	if (f)
	{
		while(fgets(buf, 255, f))
			if(strncmp(buf, "TmkLogger:", 10))
				send(ns, buf, strlen(buf), 0);
		pclose(f);
	}
	else
	{
		snprintf(buf, sizeof(buf), "<ERROR>:<Error: Could not find script file \"%s\" ....>\r\n", script );
		send(ns, buf, strlen(buf), 0);
	}
}


static void 
internal_DeleteShowing( char *cmd, int fd) 
{
	const char *script = "\
source $tcl_library/tv/log.tcl\n\
source $tcl_library/tv/mfslib.tcl\n\
\n\
\n\
\n\
proc DeleteShow {chan recfsid} {\n\
  global db\n\
\n\
  if {[string index $recfsid 0] == \"/\"} {\n\
    set recfsid [string range $recfsid 1 end]\n\
  }\n\
\n\
  set index [string first \"/\" $recfsid]\n\
  if { $index != -1 } {\n\
    set index [expr ($index - 1)]\n\
    set recfsid [string range $recfsid 0 $index]\n\
  }\n\
\n\
\n\
  set success [DeleteNowShowingRec $recfsid]\n\
\n\
  return $success\n\
}\n\
\n\
proc DeleteNowShowingRec { recfsid } {\n\
   global db\n\
\n\
   set canceldate [expr [clock seconds] / 86400]\n\
   set canceltime [expr [clock seconds] % 86400]\n\
\n\
   RetryTransaction {\n\
      set rec [db $db openid $recfsid]\n\
      set state [dbobj $rec get State]\n\
      if { $state != 4 } {\n\
         return 0\n\
      } else {\n\
         dbobj $rec set CancelReason 12\n\
         dbobj $rec set DeletionDate $canceldate\n\
         dbobj $rec set DeletionTime $canceltime\n\
         set errorstring [dbobj $rec get ErrorString]\n\
         set elength [string length $errorstring]\n\
         if { $elength > 0 } {\n\
            set errorstring [string trim $errorstring \"\\{\\}\"]\n\
            dbobj $rec set ErrorString \"$errorstring Deleted by user\"\n\
         } else {\n\
            dbobj $rec set ErrorString \"Deleted by user\"\n\
         }\n\
         dbobj $rec set State 5\n\
      }\n\
   }\n\
   return 1\n\
}\n\
\n\
set arg1 [lindex $argv 1]\n\
set arg2 [lindex $argv 2]\n\
\n\
global db\n\
set dbPoolSize [expr 100 * 1024]\n\
set db [dbopen $dbPoolSize]\n\
\n\
set chan stdout\n\
\n\
if { $arg1 == \"DELETE\" } {\n\
\n\
\n\
  set ok 1\n\
  foreach record [split $arg2 \" ,\"] {\n\
    set ret [DeleteShow $chan $record]\n\
\n\
    if { $ret == 0 } {\n\
      set ok 0\n\
    }\n\
  }\n\
\n\
  if { $ok == 1 } {\n\
    puts -nonewline $chan \"Delete Successful!\\r\\n\"\n\
  }  else {\n\
    puts -nonewline $chan \"Delete Incomplete...\\r\\n\"\n\
  }\n\
\n\
}\n\
catch { flush $chan }\n\
close $chan\n\
exit\n";
	int rc = 0, pid;
	int tofd[2];
	// Break cmd string at first blank
	char *arg = rindex(cmd,' ');
	if (arg) *arg++ = 0;

	/** Fork a process to run it the script in */
	rc = pipe(tofd);
	if (rc == -1)
		perror("pipe:");
	pid = fork();
	if (pid == 0) {		/* child */
		char *tivosh="/tvbin/tivosh";
		char *argv[] = { tivosh, strdup("-"), cmd, arg, 0 };
		extern char **environ;

		// pipe stdin; stdout to socket.
		close(0);
		close(1);
		close( tofd[1] );
		dup2( tofd[0], 0 );
		dup2( fd, 1 );
		close( tofd[0] );
		close(fd);
		execve( tivosh, argv, environ );
		perror("execve");
		_exit(1);	/* should never get here */
	} else if (pid >0) {	/* parent */
		int status;
		int rc;
		close( tofd[0] );
		write( tofd[1], script, strlen(script) );
		close(tofd[1]);
		rc = waitpid(pid, &status, 0 ); 
		if (rc == -1)
			perror("waitpid");
		else if (WIFEXITED(status)) {
				rc = WEXITSTATUS(status);
		} else
			rc = -1;
	} else {		/* error */
		perror("fork");
		rc = 1;
	}
	if (rc != 0) {
		const char *msg = 
			"<ERROR>:<Error: DeleteShowing script failed\n";
		send(fd, msg, strlen(msg), 0);
	}
}

//
// main - Where it all begins...
//
int
main(int argc, char *argv[])
{
	char			buf[CMD_SIZE];			// Command we read from the socket.
	int			ns = -1;			// New socket descriptor
	unsigned int		len;
	
	struct sockaddr		client_sa;			// Data structure for the address of the client entity
	struct sockaddr_in	sa_in;				// Structure for address of the server entity.

	int			doPri = 1;			// By default change the priority.
	int			reuseAddr;
	int                     c, fd, inetd = 0;
	int                    delayms=-1;
	const char *           progname = argv[0];

	while ((c = getopt(argc, argv, "nd:s:r:lLih?")) != -1)
	{
		switch (c)
		{
		        case 'n':	doPri=0;	 break;	// Don't change priority.
 		        case 'd':       script_DeleteShowing=optarg;  break;
 		        case 's':       script_NowShowing=optarg;  break;
			case 'l':	setup_syslog(1); break;
			case 'L':	setup_syslog(2); break;
			case 'i':	inetd=1;	 break;
		        case 'r':       delayms=strtoll(optarg,NULL,0);  break;
			default:	usage(progname); exit(1);
		}
	}
	
	argc -= optind; argv += optind;

	if (argc != 0)
	{
		usage(progname);
	}

	if (inetd) {
		// If started by inetd, stdin+stdout+stderr are all connected
		// to the socket. Here we keep just stdin (fd=0).
		int logfd;
		logfd= open("/dev/console", O_WRONLY|O_CREAT|O_TRUNC, 0644);
		if (logfd==-1) exit(2);
		dup2(logfd,1);
		close(logfd);
		dup2(1,2);
		sd = dup(0);
		close(0);
	}

	/* Check NowSHowing script */
	if (script_NowShowing) {
	  fd = open(script_NowShowing, O_RDONLY, 0);
	  if (fd < 0) {
		  char msg[132];
		  snprintf( msg, sizeof(msg), "Couldn't open %s", script_NowShowing );
		  perror(msg);
		  exit(1);
	  }
	  close(fd);
	}

	// Should really check that execute is allowed
	if (script_DeleteShowing) {
		fd = open(script_DeleteShowing, O_RDONLY, 0);
		if (fd < 0) {
			char msg[132];
			snprintf( msg, sizeof(msg), "Couldn't open %s", script_DeleteShowing );
			perror(msg);
		} else 
			close(fd);
	}

#ifdef TIVO
	//
	// Change our priority at the very start to keep up from hacking the tivo software too bad.
	//	
	if (doPri)
	{
		printf("Doing the Lowest PriorityFix...\n");
		fixPriority(1);				// Force ourselves to a low priority to keep the Tivo from skipping.

	}
	else
	{
		printf("NOT DOING the Lowest PriorityFix...\n");
	}
#endif

	mfs_init();						// Turn on the mfs code so we can read these as we go.

	if (!inetd) {
	  sd = socket(AF_INET,SOCK_STREAM,0);			// Get a socket descriptor and make sure it is good.
	  if (sd<0)
	  {
		perror("Can't open a socket\n");
		exit(0);
	  }

	  bzero(&sa_in, sizeof(sa_in));				// Bind the socket to an address.
	  sa_in.sin_addr.s_addr	= INADDR_ANY;			// And legal address in the server interface list.
	  sa_in.sin_port	= htons(0xded);			// on port 3565
	  sa_in.sin_family	= AF_INET;			// An inet family socket.

	  reuseAddr = 1;					// Be able to reopen an already running address.
	  setsockopt(sd, SOL_SOCKET, SO_REUSEADDR, (void *)&reuseAddr, sizeof(int));

	  if (bind(sd, (struct sockaddr *)&sa_in, sizeof(sa_in)))
	  {
		perror("Bad binding in the server");
		close(sd);
		exit(0);
	  }
	}
	
	signal(SIGINT,onintr);					// Setup the ^C interrupts and other good ones to watch.
	signal(SIGKILL,onintr);					// All of this so that we shut down as cleanly as possible.
	signal(SIGTERM,onintr);
	signal(SIGPIPE,write_error);
	
	if (!inetd)
	  listen(sd, 10);					// Set the listen queue, 10 can wait while we process the current.
	
	for(;;)
	{
		int ret;
		int i;
		
		if (inetd) {
		  ns = sd;
		} else {
		  printf("Waiting for an incoming connection!\n");
		
		  ret = waitio (sd, 0);
		  if (ret != 1)	continue;			// Something went bad, so leave!
		
		  ns = accept(sd,&client_sa,&len);		// Wait here for a client to connect and make a request.
		}
		bzero(buf, CMD_SIZE);
		read(ns, &buf, CMD_SIZE);
		
		for (i=strlen(buf); i>=0; i--)			// Chop the return.
		{
			if (buf[i] == '\0')		buf[i] = '\0';
			else if (buf[i] == '\r')	buf[i] = '\0';
			else if (buf[i] == '\n')	buf[i] = '\0';
			else				break;
		}

		logmsg("SERVER: We got a message! buf = '%s'\n", buf);
		//
		// CORE: Download a TyStream here...
		//
		if (!strncmp(buf, "TYSTREAM", strlen("TYSTREAM")))	// Do the work here!
		{
			char	seps[]   = "/,";
			char	*token;
			int	FSIDs[100];
			int	pos = 0;
			int	fsid;

			token = strtok( &(buf[strlen("TYSTREAM")+1]), seps );
			while(token != NULL)			// While there are tokens in "string"
			{
				printf("-> '%s'\n", token);

				FSIDs[pos++] = myAtoi(token);

				fsid = mfs_resolve(token);
				if (export_file(fsid,ns, 0, 0, delayms, 256, 0, 0) <0)
					break; // Stop as soon as we have a problem.
				token = strtok(NULL, seps);
			}

//			printf("pos = %d\n", pos);
//			printf("reget = %d\n", reget);
#ifdef TIVO
//			if (pos)	ForgeMain(ns, pos, FSIDs);				// If we found one, then do it.

/*
			for (i=0; i < pos; i++)
			{
				int fsid = mfs_resolve(FSIDs[i]);
				export_file(ns, fsid);
			}
*/
#endif
		}
		else if (!strncmp(buf, "LISTSECTORS", strlen("LISTSECTORS")))	
		{
			char	seps[]   = "/,";
			char	*token;
			int	FSIDs[100];
			int	pos = 0;
			int	fsid;

			token = strtok( &(buf[strlen("LISTSECTORS")+1]), seps );
			logmsg("token: %s", token);

			while(token != NULL) // While there are still fsids to process..
			{
				int i,j;
				char buf[128];
				run_desc runs[1024];
				int count;
				printf("-> '%s'\n", token);

				FSIDs[pos++] = myAtoi(token);

				fsid = mfs_resolve(token);
				j=snprintf(buf,sizeof(buf),"FSID:%d\n",fsid);
				if(j<sizeof(buf))
				  send(ns,buf,strlen(buf),0);
				count = list_sectors_for_file(fsid, runs, 
							      sizeof(runs)/sizeof(runs[0]));
				if (count <0)
					break; // Stop as soon as we have a problem.
				for(i=0; i<count; i++) {
					/* dump to buffer, then send buffer to client */
					j=snprintf(buf,sizeof(buf),
						   "DRV:%d PART:%d START:%d COUNT:%d\n",
						   runs[i].drive, runs[i].partition,
						   (int)runs[i].start, runs[i].count);
					if(j<sizeof(buf))
					{
						send(ns,buf,strlen(buf),0);
					}
					
				}
				token = strtok(NULL, seps);
			}

		}
		//
		// CORE: Download a NowShowing list here...
		//
		else if (!strncmp(buf, "SHOWING", strlen("SHOWING")))	// Do the work here!
		{
			if (!script_NowShowing)
				generate_NowShowing( ns );
			else
				run_script(script_NowShowing,ns);
		}
		else if (!strncmp(buf, "DELETE", strlen("DELETE")))	// Do the work here!
		{
			if (!script_NowShowing)
				internal_DeleteShowing(buf,ns);
			else {
				char buf2[256];
				snprintf(buf2, sizeof(buf2), "%s %s", script_DeleteShowing, buf);
				fprintf (stderr, "running cmd \"%s\"\n", buf2 );
				run_script(buf2,ns);
			}
		}
		//
		// CORE: Download a TyStream in the new way here... The new way gets us going at full packet sizes.
		//
		else if (!strncmp(buf, "TYSTRM2", strlen("TYSTRM2")))		// Do the work here!
		{
			char	seps[]   = "/,";
			char	*token;
			int	i;
			int	fsid;
			int	ns2;
			char	junk1[256];
			char	addr[256];
			char	port[256];

			sscanf(buf, "%s %s %s", junk1, addr, port);
			// printf("Addr = '%s' && port = %d\n", addr, myAtoi(port));

			ns2 = SetupClientSocket(addr, myAtoi(port));
			if (ns2 == -1)
			{
				printf("ERROR: Could not connect to client to send data!\n");
			}
			else
			{
				i = strlen("TYSTRM2")+1 + strlen(addr)+1 + strlen(port)+1;
				token = strtok( &(buf[i]), seps );
				while(token != NULL)			// While there are tokens in "string"
				{
					printf("-> '%s'\n", token);

					fsid = mfs_resolve(token);
					if (export_file(fsid,ns2, 0, 0, delayms, 256, 0, 0) <0)
						break; // Stop as soon as we have a problem.
					token = strtok(NULL, seps);
				}

				close(ns2);
			}
		}
/*
		else if (!strncmp(buf, "TEST", strlen("TEST")))
		{
			char	bigBuf[10 * 4096];
			char	junk1[256];
			char	addr[256];
			char	port[256];
			int	ns2;
			int	i;
			int	fsid;

			for (i=0; i<10 * 4096; i++)
			{
				if (i <     4096)	bigBuf[i] = '1';
				if (i < 2 * 4096)	bigBuf[i] = '2';
				if (i < 3 * 4096)	bigBuf[i] = '3';
				if (i < 4 * 4096)	bigBuf[i] = '4';
				if (i < 5 * 4096)	bigBuf[i] = '5';
				if (i < 6 * 4096)	bigBuf[i] = '6';
				if (i < 7 * 4096)	bigBuf[i] = '7';
				if (i < 8 * 4096)	bigBuf[i] = '8';
				if (i < 9 * 4096)	bigBuf[i] = '9';
				if (i < 10* 4096)	bigBuf[i] = '0';
			}

			sscanf(buf, "%s %s %s", junk1, addr, port);
			printf("Addr = '%s' && port = %d\n", addr, myAtoi(port));

			ns2 = SetupClientSocket(addr, myAtoi(port));
			if (ns2 > -1)
			{
				printf("Socket connected: %d\n", ns2);

				printf("writing: %d\n", 10 * 4096);
//				ret = write(ns2, bigBuf, 10 * 4096);

				// 596245/596248/596251
				fsid = mfs_resolve("596245");
				export_file(ns2, fsid);	

				fsid = mfs_resolve("596248");
				export_file(ns2, fsid);	

				fsid = mfs_resolve("596251");
				export_file(ns2, fsid);	

				printf("And done!\n");

				close(ns2);
			}
		}
*/
		else
		{
			printf("Bogus command... '%s'\n", buf);
		}
		
		close(ns);
		if (inetd) break;
	}
	exit(0);
}


//
// waitio - A non-busy wait for the socket.
//
int waitio (const int socket, const int secs)
{
	static fd_set	readset;
	struct timeval	TimeDelay;
	int		rslt;

	FD_ZERO (&readset);
	FD_SET (socket, &readset);
	
	if (secs)
	{
		TimeDelay.tv_sec  = secs;
		TimeDelay.tv_usec = 0;
		
		rslt = select(socket+1, &readset, NULL, NULL, &TimeDelay);
	}
	else
	{
		rslt = select(socket+1, &readset, NULL, NULL, NULL);
	}

	if (rslt <= 0)	return(rslt);				// We either timed out, or had some other error so leave...
	else if (FD_ISSET(socket, &readset))			// We read something from a socket, just not the right one...
	{
		return(1);					// It worked...
	}
	else
	{
		return(0);					// We read something from a socket, just not the right one...
	}
}


//
// SetupClientSocket - Create and fully connect a client TCP socket. It creats the socket,
//		connects it and returns the new socket.
//
int SetupClientSocket(char *Servername, int ServerPort)
{
	struct sockaddr_in	 sa;
	struct hostent		*hp;
	char			 hostname[256];
	int			 s;

	//
	// Create the socket handle.
	//
	s = socket(AF_INET, SOCK_STREAM, 0);
	if (s == -1)
	{
		printf("ERROR: Could not allocate the socket!\n");
		return(-1);
	}

	//
	// Now Create the server address and connect to it.
	//
	if (Servername)	strcpy(hostname, Servername);			// If we have a localname arg, then use it.
	else		gethostname(hostname, 255);			// Otherwise try to get it from the system.

	//
	// Start setting up the address for the socket.
	//
	memset(&sa, 0, sizeof(struct sockaddr_in));
	if (inet_addr(hostname) != -1)					// 1st try to just turn what we were passed into an address.
	{
		sa.sin_addr.s_addr = inet_addr(hostname);
		sa.sin_family = AF_INET;
	}
	else
	{
		//
		// Didn't work, so get the HostEnt by name and see if that worked.
		//
		if ((hp = gethostbyname(hostname)) == NULL)
		{
			printf("ERROR: Host %s not found...\n", hostname);
			return(-1);
		}

		sa.sin_addr.s_addr = *(unsigned long *)hp->h_addr;
		sa.sin_family = hp->h_addrtype;
	}
	sa.sin_port = htons((unsigned short)ServerPort);		// Use the server port arg!

	if (connect(s, (struct sockaddr *)&sa, sizeof(sa)) == -1)
	{
		printf("ERROR: Could not connect the socket!\n");
		return(-1);
	}

	return(s);
}

