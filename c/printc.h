#include <stdio.h>
#include <stdarg.h>

inline extern int print(const char *format, ...) {
  va_list va;
  char buffer[1024];

  va_start(va, format);
  int j = vsnprintf(buffer, 1024, format, va);
  printrs(buffer);
  va_end(va);

  return j;
}

inline extern int println(const char *format, ...) {
  va_list va;
  char buffer[1024];

  va_start(va, format);
  int j = vsnprintf(buffer, 1024, format, va);
  printrsln(buffer);
  va_end(va);

  return j;
}
