MYARCH := $(shell uname -ms | tr ' ' -)
DEBUG=0
STATIC=0
USE_TRIDGE_MFS_SO=0

CFLAGS = -Wall -I. -I/sw/include -I/sw/include/gnugetopt -I../include -D_GNU_SOURCE
CCLDFLAGS =

IS_TIVO := $(shell grep -qsE 'Teleworld|TiVo' /proc/cpuinfo && echo 1 || echo 0)
ifneq ($(IS_TIVO),0)
MYARCH = $(shell uname -m)
endif
ARCH := $(MYARCH)
ifeq ($(ARCH),ppc)
# ARCH is ppc
PREFIX=powerpc-TiVo-linux-
CFLAGS += -DTIVO -DTIVO_S1
EXTRABINS = contrib/s1_unscramble
else
# ARCH is mips or native (assuming kernel/glibc largefile support)
CFLAGS += -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE 
ifeq ($(ARCH),mips)
# ARCH is mips
PREFIX=mips-TiVo-linux-
CFLAGS += -mips2 -DTIVO -DTIVO_S2 
#CCLDFLAGS += -static
else
# host ARCH, not cross compiling
EXTRABINS = vplayer vsplit
ifeq ($(ARCH),Darwin-Power-Macintosh)
CFLAGS += -DNEED_STRNDUP -DNEED_STRNDUPA -DNEED_STRDUPA
else
ifeq ($(ARCH),Darwin-i386\)
CFLAGS += -DNEED_STRNDUP -DNEED_STRNDUPA -DNEED_STRDUPA
else
CFLAGS += -DNEED_ALLOCA_H
ifeq ($(findstring CYGWIN,$(ARCH)),CYGWIN)
CFLAGS += -DNEED_STRNDUPA -DNEED_STRDUPA
endif
endif
endif
endif
endif

ifneq ($(IS_TIVO),0)
PREFIX = 
endif

CC = $(PREFIX)gcc
AR = $(PREFIX)ar

ifeq ($(DEBUG),1)
CFLAGS += -O0 -ggdb
#LIBS += -lefence -lpthread
else
CFLAGS += -O3
CCLDFLAGS += -Wl,-s
endif

ifeq ($(STATIC),1)
CCLDFLAGS += -static
TRIDGE_MFS_LIB=$(OBJDIR)/libtridgemfs.a
else
ifeq ($(USE_TRIDGE_MFS_SO),1)
TRIDGE_MFS_LIB=$(BINDIR)/libtridgemfs.so.1.0
CCLDFLAGS += -Wl,-rpath,/usr/local/lib
else
TRIDGE_MFS_LIB=$(OBJDIR)/libtridgemfs.a
endif
endif

# Add a define for the build date for usage message strings.
CFLAGS += -DBUILD_DATE=\"`date +%Y/%m/%d`\"

AUTO_PROTO_SRC = mfs.c object.c util.c bitmap.c io.c partition.c \
	crc.c pri.c export.c schema.c query.c tzoffset.c tar.c \
	credits.c read_xml.c generate_xml.c generate_NowShowing.c attribute.c log.c
COMMON = $(AUTO_PROTO_SRC) ty_audio.c

BINS = \
 mfs_info mfs_ls mfs_streams mfs_dumpobj mfs_dumpschema mfs_tzoffset \
 mfs_import mfs_uberexport mfs_burstcmds                \
 mfs_export mfs_stream mfs_tarstream mfs_tmfstream      \
 tserver vserver NowShowing ciphercheck    \
 vplay                                                  \
 mfs_dump mfs_poke                                      \
 mfs_bitmap mfs_purge mfs_getslice mfs_findzero

OBJDIR = obj.$(ARCH)
BINDIR = bin.$(ARCH)

.PHONY : all clean binaries mkdirs tags

all: proto.h mkdirs binaries

clean:
	rm -rf obj.* bin.* proto.h preload_schema.h *~

binaries: $(BINS:%=$(BINDIR)/%)  $(EXTRABINS:%=$(BINDIR)/%) 

mkdirs:
	mkdir -p $(OBJDIR) $(BINDIR) $(OBJDIR)/contrib $(BINDIR)/contrib

tags:
	etags *.[ch]

mfs.h: proto.h

proto.h: $(AUTO_PROTO_SRC)
	cat $(AUTO_PROTO_SRC) | awk -f mkproto.awk > proto.h

.PRECIOUS : $(OBJDIR)/%.o


$(OBJDIR)/%.o : %.c mfs.h log.h
	$(CC) $(CFLAGS) -c $< -o $@

$(BINDIR)/% : $(OBJDIR)/%.o $(TRIDGE_MFS_LIB)
	$(CC) $(CCLDFLAGS) -o $@ $^ $(LIBS)

$(BINDIR)/% : %.sh
	cp $^ $(subst .sh,,$@)

$(BINDIR)/vplayer : $(OBJDIR)/vplayer.o $(TRIDGE_MFS_LIB)
	$(CC) $(CCLDFLAGS) -o $@ $^ $(LIBS) -lncurses

$(BINDIR)/libtridgemfs.so.1.0: $(COMMON:%.c=$(OBJDIR)/%.o) $(SCHEMA:%.c=$(OBJDIR)/%.o)
	$(CC) -shared $(CCLDFLAGS) -Wl,-soname,libtridgemfs.so.1 -o $@ $^

$(OBJDIR)/libtridgemfs.a: $(COMMON:%.c=$(OBJDIR)/%.o) $(SCHEMA:%.c=$(OBJDIR)/%.o)
	$(AR) -rc  $@ $^ ; $(PREFIX)ranlib $@


schema.c: preload_schema.h

preload_schema.h: schema-merged-9.3.txt
	perl make-preload-schema.pl <$< >$@
