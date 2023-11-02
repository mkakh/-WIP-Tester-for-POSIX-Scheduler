#include "util.h"
// async-signal-safe
void debug_print(const char *msg) { write(STDOUT_FILENO, msg, strlen(msg)); }

// not async-signal-safe
void debug_printf(const char *format, ...) {
  char buf[256];
  va_list args;
  va_start(args, format);
  vsnprintf(buf, sizeof(buf), format, args);
  debug_print(buf);
  va_end(args);
}

void debug_print_mapping(const mapping &m) {
  int i = 1;
  for (const auto &[key, value] : m) {
    debug_printf("%d: %d -> %d\n", i++, key, value);
  }
}
