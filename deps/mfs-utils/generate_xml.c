/*
  Media-filesystem export utility
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/

//
// generate_xml
//
// Author: Jon Biggar
// Modifiations licenced under the Gnu GPL v2
//
// based on tridge's mfs_export.
//
// based on mfs_tarstream by:
// Authors: tivodvlpr & gps - with suggestions from #tivo
// License: GNU General Public License v2
//
// $Id $
//
#include <assert.h>
#include <stdarg.h>

#include "mfs.h"
#include "attribute.h"


static int	parts[1024] = {-1};
static int	parts_len = 0;
static int	xml_len = 0;
static int	xml_alloc_size=65536;
static char	*xml_buf = 0;
static const int max_xml_size = 1024*1024;

static void xml_printf(const char *fmt, ...) {
	va_list ap;
	int len;
	va_start(ap,fmt);
	if (xml_buf == 0) {
		xml_len = 0;
		xml_alloc_size = 16*1024;
		xml_buf = malloc(xml_alloc_size);
		if (xml_buf == 0) return;
	}
	do {
		// Try to format using existing buffer spzce.
		len = vsnprintf( xml_buf+xml_len,xml_alloc_size-xml_len, fmt, 
				 ap );
		if (len < 0 || (xml_len+len)>=xml_alloc_size) {
			// failure: double allocation and try again
			xml_alloc_size *= 2;
			xml_buf = realloc( xml_buf, xml_alloc_size );
		} else 
			xml_len += len;
		// limit growth size so we don't loop until all memory is consumed.
	} while (len < 0 && xml_buf != 0 && xml_alloc_size<max_xml_size);

	va_end(ap);
}

struct TivoObjectAttrDesc {
	const char *		name;
	int			type;
	int			mandatory;
    
	/* only used for attrs of TYPE_OBJECT */
	int			is_subobject;
	const char *		object_name;
	const char *		object_xml_id;
	int			object_len;
	const struct TivoObjectAttrDesc	*object_attrs;
};

static const struct TivoObjectAttrDesc	Part[] = {
	{ "Begin", TYPE_INT },
	{ "CommercialSkipOffset", TYPE_INT },
	{ "End", TYPE_INT },
	{ "File", TYPE_FILE },
};

static const struct TivoObjectAttrDesc	ApgProgram[] = {
	{ "Category", TYPE_INT },
};

static const struct TivoObjectAttrDesc	Series[] = {
	{ "Episodic", TYPE_INT },
	{ "Genre", TYPE_INT },
	{ "ServerId", TYPE_STRING },
	{ "ServerVersion", TYPE_INT },
	{ "ThumbData", TYPE_INT },
	{ "Title", TYPE_STRING },
	{ "TmsId", TYPE_STRING },
};

static const struct TivoObjectAttrDesc	Program[] = {
	{ "Actor", TYPE_STRING },
	{ "Advisory", TYPE_INT },
	{ "ApgProgram", TYPE_OBJECT, 0, 1, "ApgProgram", "ApgProgram",
	  sizeof(ApgProgram)/sizeof(struct TivoObjectAttrDesc), ApgProgram },
	{ "ColorCode", TYPE_INT, },
	{ "DescLanguage", TYPE_STRING },
	{ "Description", TYPE_STRING },
	{ "Director", TYPE_STRING },
	{ "EpisodeNum", TYPE_INT, }, 
	{ "EpisodeTitle", TYPE_STRING },
	{ "Genre", TYPE_INT },
	{ "IsEpisode", TYPE_INT },
	{ "MovieRunTime", TYPE_INT },
	{ "MovieYear", TYPE_INT },
	{ "MpaaRating", TYPE_INT },
	{ "OriginalAirDate", TYPE_INT },
	{ "RootServerId", TYPE_STRING },
	{ "Series", TYPE_OBJECT, 0, 0, "Series", "Series",
	  sizeof(Series)/sizeof(struct TivoObjectAttrDesc), Series },
	{ "ServerId", TYPE_STRING },
	{ "ServerVersion", TYPE_INT },
	{ "ShowType", TYPE_INT },
	{ "SourceType", TYPE_INT },
	{ "Title", TYPE_STRING },
	{ "TmsId", TYPE_STRING },
	{ "Writer", TYPE_STRING },
};

static const struct TivoObjectAttrDesc	Station[] = {
	{ "Affiliation", TYPE_STRING },
	{ "AffiliationIndex", TYPE_INT },
	{ "CallSign", TYPE_STRING, 1 },
	{ "City", TYPE_STRING },
	{ "Country", TYPE_STRING },
	{ "DmaNum", TYPE_INT },
	{ "Name", TYPE_STRING },
	{ "ServerId", TYPE_STRING },
	{ "ServerVersion", TYPE_INT },
	{ "TmsId", TYPE_STRING },
	{ "ZipCode", TYPE_STRING },
};

static const struct TivoObjectAttrDesc	Showing[] = {
	{ "Bits", TYPE_INT, },
	{ "Date", TYPE_INT },
	{ "Duration", TYPE_INT },
	{ "Program", TYPE_OBJECT, 0, 0, "Program", "Program",
	  sizeof(Program)/sizeof(struct TivoObjectAttrDesc), Program },
	{ "Reason", TYPE_INT },
	{ "Station", TYPE_OBJECT, 0, 0, "Station", "Station",
	  sizeof(Station)/sizeof(struct TivoObjectAttrDesc), Station },
	{ "Time", TYPE_INT },
	{ "TvRating", TYPE_INT },
};

static const struct TivoObjectAttrDesc	Recording[] = {
	{ "BitRate", TYPE_INT },
	{ "Part", TYPE_OBJECT, 1, 1, "RecordingPart", "Part",
	  sizeof(Part)/sizeof(struct TivoObjectAttrDesc), Part },
	{ "RecordQuality", TYPE_INT },
	{ "SelectionType", TYPE_INT },
	{ "Showing", TYPE_OBJECT, 0, 1, "Showing", "Showing",
	  sizeof(Showing)/sizeof(struct TivoObjectAttrDesc), Showing },
	{ "StartDate", TYPE_INT },
	{ "StartTime", TYPE_INT },
	{ "StopDate", TYPE_INT },
	{ "StopTime", TYPE_INT },
	{ "DeletionDate", TYPE_INT },
	{ "ExpirationDate", TYPE_INT },
	{ "ExpirationTime", TYPE_INT },
	{ "StreamFileSize", TYPE_INT },
	{ "SubPriority", TYPE_INT },
	{ "UsedBy", TYPE_INT },
};

static const int RecordingLen = sizeof(Recording)/sizeof(struct TivoObjectAttrDesc);

static void 
generate_object_xml(int fsid, int subobjid,
		    int depth, const char *name,
		    int ad_len, const struct TivoObjectAttrDesc *attr_descs)
{
	int		i,j=0;
	mfs_attribute_t	attr;

	for (i = 0; i < ad_len; ++i) {
		if (get_attribute_fsid(fsid, name, subobjid, attr_descs[i].name,
				  &attr)) {
			if (attr.type != attr_descs[i].type) {
				fprintf(stderr, "Bad %s:%s\n", name,
					attr_descs[i].name);
				exit(1);
			}
			switch (attr_descs[i].type) {
			case TYPE_INT:
				for (j = 0; j < attr.n; ++j) {
					xml_printf("%*s<%s>%d</%s>\n", depth+2, "",
						   attr_descs[i].name,
						   attr.u.integer[j], attr_descs[i].name);
				}
				break;
			case TYPE_FILE:
				xml_printf("%*s<%s>%d</%s>\n", depth+2, "",
					   attr_descs[i].name,
					   attr.u.integer[0], attr_descs[i].name);
				parts[parts_len++] = attr.u.integer[0];
				break;
			case TYPE_OBJECT:
				for (j = 0; j < attr.n; ++j) {
					if (attr_descs[i].is_subobject) {
						xml_printf("%*s<SubObject type=\"%s\" id=\"%s\">\n",
							   depth+1, "", attr_descs[i].object_name,
							   attr_descs[i].object_xml_id);
						generate_object_xml(fsid, attr.u.object[j].subobj,
								    depth+1,
								    attr_descs[i].object_name,
								    attr_descs[i].object_len,
								    attr_descs[i].object_attrs);
						xml_printf("%*s</SubObject>\n", depth+1, "");
					} else {
						xml_printf("%*s<Object type=\"%s\" id=\"%s\">\n",
							   depth+1, "", attr_descs[i].object_name,
							   attr_descs[i].object_xml_id);
						generate_object_xml(attr.u.object[j].fsid,
								    attr.u.object[j].subobj,
								    depth+1,
								    attr_descs[i].object_name,
								    attr_descs[i].object_len,
								    attr_descs[i].object_attrs);
						xml_printf("%*s</Object>\n", depth+1, "");
					}
				}
				break;
			case TYPE_STRING:
				for (j = 0; j<attr.n;++j) {
					xml_printf("%*s<%s>%s</%s>\n", depth+2, "",
						   attr_descs[i].name,
						   attr.u.string[j], attr_descs[i].name);
				}
				break;
			}
			attr_release(&attr);
		} else if (attr_descs[i].mandatory) {
			fprintf(stderr, "Can't get %s:%s\n", name, attr_descs[i].name);
			exit(1);
		}
	}
}

// caller must free result
char *generate_xml(char *path)
{
	int	fsid = mfs_resolve(path);
	char *tivo_version = get_tivo_version();

	if (!fsid ||
	    mfs_fsid_type(fsid) != MFS_TYPE_OBJ) {
		fprintf(stderr, "Bad recording id: %s  type: %d\n", path, 
			mfs_fsid_type(fsid) );
		return 0;
	}
	
	xml_printf("<?xml version=\"1.1\" tivoversion=\"%s\"?>\n",
		   tivo_version);

	xml_printf("<Object type=\"Recording\" id=\"_top\">\n");

	generate_object_xml(fsid, 0xffffffff, 0, "Recording",
			    RecordingLen, Recording);

	xml_printf("</Object>\n");
	if (tivo_version != 0) 
		free(tivo_version);
	parts[parts_len] = -1;
	return xml_buf;
}

int *get_parts() 
{
	return parts;
}

