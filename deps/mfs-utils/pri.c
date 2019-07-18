#ifdef TIVO

#include <stdio.h>
#include <stdarg.h>
#include <sys/types.h>
#define _STRUCT_TIMESPEC
#include <sched.h>
#include <unistd.h>
#include "log.h"

#ifdef TIVO_S1
//#include <asm/unistd.h>
/*
_syscall3(int, sched_setscheduler, pid_t, pid, int, policy, struct sched_param, *param);

extern int sched_setscheduler(pid_t pid, int policy, struct sched_param *param);
*/

#endif /* TIVO_S1 */

int fixPriority( int pri )
{
	struct sched_param	param;

	param.sched_priority = (pri<0) ? 0 : (pri>99 ? 99 : pri);
	if(sched_setscheduler(getpid(), pri?SCHED_FIFO:SCHED_OTHER, &param))
	{
		fprintf(stderr, "Could not set the priority...\n");
		perror("nicepri");
	}
#ifdef PRI_VERBOSE
	printf("Priority set...\n");
#endif

	return 0;
}

#else /* TIVO */

int fixPriority(int pri)
{
	return(0);
}

#endif /* TIVO */
