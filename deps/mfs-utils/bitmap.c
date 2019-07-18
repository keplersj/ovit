#include "mfs.h"
#include "log.h"

/****************************************************************************
allocate a bitmap of the specified size
****************************************************************************/
struct bitmap *bitmap_allocate(int n)
{
	struct bitmap *bm;

	bm = (struct bitmap *)malloc(sizeof(*bm));

	if (!bm) return NULL;
	
	bm->n = n;
	bm->b = (u32 *)malloc(sizeof(bm->b[0])*(n+31)/32);
	if (!bm->b) {
		free(bm);
		return NULL;
	}

	memset(bm->b, 0, sizeof(bm->b[0])*(n+31)/32);

	return bm;
}

/****************************************************************************
set a bit in a bitmap
****************************************************************************/
void bitmap_set(struct bitmap *bm, unsigned i, unsigned n)
{
	if (i+n > bm->n) {
		fprintf(stderr, "Invalid bitmap set bits=%d i=%d n=%d\n",
		       bm->n, i, n);
		exit(1);
	}
	while (n--) {
		bm->b[i/32] |= (1<<((31-i)%32));
		i++;
	}
}


/****************************************************************************
query a bit in a bitmap
****************************************************************************/
int bitmap_query(struct bitmap *bm, unsigned i)
{
	if (bm->b[i/32] & (1<<((31-i)%32))) {
		return 1;
	}
	return 0;
}
