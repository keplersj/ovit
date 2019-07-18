#include <stdio.h>
#include <stdlib.h>
#include "mfs.h"

int main(int argc, char *argv[]) {
  mfs_init();
  fprintf( stderr, "tzoffset is: %d\n", tzoffset() );
  exit(0);
}
