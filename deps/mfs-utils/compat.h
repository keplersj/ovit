/**
 * Compatability Defines
 */
#ifndef _COMPAT_H
#define _COMPAT_H


#ifdef NEED_ALLOC_H
#include <alloca.h>
#endif

#ifdef NEED_STRNDUP
#undef strndup
#define strndup(s,n) \
	({								\
		const char *__old = (s);				\
		size_t ns = (n);					\
		char *__nw;						\
		size_t __len=strlen(__old);				\
		__len = __len>(ns)?(ns):__len;				\
		__nw = (char *) malloc(__len+1);			\
		(__nw ? (__nw[__len]=0,(char *)strncpy(__nw,__old,__len)):0); \
	})
#endif

#ifdef NEED_STRNDUPA
#undef strndupa
#define strndupa(s,n) \
	({					                               \
		const char *__old = (s);					\
		size_t ns = (n);                                               \
		char *__nw;			                               \
		size_t __len=strlen(s);                          	       \
		__len = __len>(ns)?(ns):__len;				       \
		__nw = (char *) alloca(__len+1);			       \
		(__nw ? (__nw[__len]=0, (char *)strncpy(__nw,__old,__len)):0); \
	})
#endif

#ifdef NEED_STRDUPA
#undef strdupa
#define strdupa(s)                                              \
       ({                                                       \
	       const char *__old = (s);				\
	       (char *)strcpy(alloca(strlen(__old)+1),__old);	\
       })
#endif

#endif
