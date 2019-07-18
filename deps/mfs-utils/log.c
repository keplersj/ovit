#include "log.h"
int use_syslog = 0;

void setup_syslog (int level)
{
  if (!use_syslog)
    openlog(NULL, LOG_PID, LOG_USER);
  use_syslog= level;
  return;
}
