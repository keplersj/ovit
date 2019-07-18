#ifndef MFS_LOG_H
#define MFS_LOG_H

#include <string.h>
#include <stdio.h>
#include <syslog.h>
#include <stdarg.h>
#include <errno.h>

// use_statlog=0: write to stdout/stderr as normal
// use_statlog=1: stderr->syslog, drop stdout
// use_statlog=2: stdout+stderr->syslog
extern int use_syslog;
extern void setup_syslog(int level);

#define logmsg(args...)    (use_syslog ? (syslog(LOG_INFO,args),1) : printf(args))
#define printf(args...)    (use_syslog ? (use_syslog>=2 ? (syslog(LOG_INFO,args),1) : 1) : printf(args))
#define fprintf(s,args...) (use_syslog ? (fileno(s)==2 ?                  (syslog(LOG_ERR, args),1) :        \
                                          fileno(s)==1 ? (use_syslog>=2 ? (syslog(LOG_INFO,args),1) : 1)     \
                                                       : fprintf((s),args))                                  \
                                       :                 fprintf((s),args))
#define vprintf(f,a)       (use_syslog ? (use_syslog>=2 ? (vsyslog(LOG_INFO,(f),(a)),1) : 1) : vprintf(f,a))
#define vfprintf(s,f,a)    (use_syslog ? (fileno(s)==2 ?                  (vsyslog(LOG_ERR, (f),(a)),1) :   \
                                          fileno(s)==1 ? (use_syslog>=2 ? (vsyslog(LOG_INFO,(f),(a)),1) : 1) \
                                                       : vfprintf((s),(f),(a)))                              \
                                       :                 vfprintf((s),(f),(a)))
#define perror(s)          (use_syslog ? (syslog(LOG_ERR,"%s: %s",(s),strerror(errno)),1) : perror(s))

#endif
