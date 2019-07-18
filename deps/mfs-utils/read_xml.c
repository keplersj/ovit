#include <unistd.h>
#include <stdlib.h>

// Read XML from stdin into an inmemory buffer, null terminated.
// Caller must free the buffer
char *read_xml()
{
	char *buf;
	int n, p;
	int nalloc = 16*1024;

	/* allocate an arbitrarily big buffer, to prevent fragging mem */
	buf = malloc(nalloc);

	n = read(0, buf, nalloc);
	p = n;
	while (n > 0)
	{
		nalloc *= 2;
		buf = realloc(buf, p + nalloc);
		n = read(0, &buf[p], nalloc);
		p += n;
	}
	buf[p] = 0; // null terminate
	return buf;
}
