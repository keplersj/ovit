/*
  media-filesystem object schema code
  tridge@samba.org, January 2001
  released under the Gnu GPL v2
*/
#include <assert.h>
#include "preload_schema.h"
#include "log.h"

static const char **types = 0;
static int ntypes=0;
static attrs_t *attrs = 0;

static void schema_add(int type, int attr, char *name, int objtype)
{
	attrs_t *a;

	assert(type >= 0);
	if (attrs[type].n == 0) {
		attrs[type].n = 20;
		attrs[type].a = (attr_t *) malloc( 20*sizeof(const attr_t) );
		memset( attrs[type].a, 0, 20*sizeof(const attr_t) );
	}
	a = &attrs[type];
	while (attr >= a->n) {
		int osz = a->n;
		a->n *= 2;
		a->a = (attr_t *) realloc( a->a, a->n*sizeof(const attr_t) );
		memset( a->a+osz, 0, osz*sizeof(const attr_t) );
	}	
	
	a->a[attr].name = strdup(name);
	a->a[attr].objtype = objtype;
	return;
}

/* used to load a local schema.txt to make things faster */
static int preload_schema_file(char *fname)
{
	FILE *f = fopen(fname, "r");
	int itype, iattr, atype=0;
	char *type, *attr, *flag;

	char line[200];

	if (!f) return 0;

	while (fgets(line, sizeof(line), f)) {
		if (!isdigit(line[0])) continue;
		if (line[strlen(line)-1] == '\n') line[strlen(line)-1] = 0;
		itype = atoi(strtok(line,"\t "));
		type = strtok(NULL,"\t ");
		iattr = atoi(strtok(NULL,"\t "));
		attr = strtok(NULL,"\t ");
		flag = strtok(NULL,"\t ");

		if (types == 0) {
			attrs = (attrs_t *) malloc( 200*sizeof(const attrs_t) );
			memset( attrs, 0, 200*sizeof(const attrs_t) );
			types = (const char **) malloc( 200*sizeof(char *) );
			memset( types, 0, 200*sizeof(char *) );
			ntypes = 200;
		}
		while (itype >= ntypes) {
			int osz = ntypes;
			ntypes *= 2;
			types = (const char **) realloc ( types, ntypes*sizeof(char *) );
			attrs = (attrs_t *) realloc( attrs, ntypes*sizeof(const attrs_t));
			memset( attrs+osz, 0, osz*sizeof(const attrs_t) );
			memset( types+osz, 0, osz*sizeof(const char*) );
		}
		if (strcmp(flag,"string")==0) {
			atype = TYPE_STRING;
		} else if (strcmp(flag,"int")==0) {
			atype = TYPE_INT;
		} else if (strcmp(flag,"object")==0) {
			atype = TYPE_OBJECT;
		} else if (strcmp(flag,"file")==0) {
			atype = TYPE_FILE;
		}
		if (!types[itype]) types[itype] = strdup(type);
		schema_add(itype, iattr, attr, atype);
		fprintf(stderr, "preloaded %d/%d/%s/%d\n",itype, iattr, attr, atype);
	}

	fclose(f);
	return 1;
}

/* preload scheme from compiled in table */
static int preload_schema() {
	types = stype_names;
	ntypes = sizeof(stype_names)/sizeof(stype_names[0]);
	attrs = sattrs;
	return 1;
}

static void load_schema(void)
{
	static int loaded;
	char *filename; 
	
	if (loaded) return;

	loaded = 1;

	filename = getenv("TIVO_SCHEMA");
	if (filename && preload_schema_file(filename)) return;

	preload_schema();
}

/* lookup a string for a schema type */
const char *schema_type(int type)
{
	if (!types) load_schema();

	return (type>=ntypes) ? 0 : types[type];
}

/* lookup an attribute for a given type and attr value,
   auto-loading the schema if necessary */
const char *schema_attrib(int type, int attr)
{
	attrs_t a;
	(void) schema_type(type);
	if (type >= ntypes) {
//		fprintf(stderr,"Invalid type %d in schema_attrib\n", type);
		return "UNKNOWN";
	}
	a = attrs[type];
	if (attr >= a.n) {
//	  fprintf(stderr,"Invalid attr %d in schema_attrib for type: %s\n", attr, types[type] );
		return "UNKNOWN";
	}
	if (!a.a[attr].name) {
		return "UNKNOWN";
	}
	//	printf("schema_attrib(%d, %d) -> %s\n", type, attr, attrs[type][attr]);
	return a.a[attr].name;
}

/* dump the complete schema */
void dump_schema(FILE *f)
{
	int i, j;

	load_schema();

	for (i=1; i<ntypes; i++) {
		if (!types[i]) continue;
		for (j=1; j<attrs[i].n; j++) {
			if (attrs[i].a[j].name) {
				fprintf(f, "%d %s %d %s %s\n",
				       i, types[i], j, attrs[i].a[j].name,
				       object_typestr(attrs[i].a[j].objtype));
			}
		}
	}
}


