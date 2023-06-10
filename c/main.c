#include "header.h"
#include <assert.h>
#include <stdio.h>

int main() {
  assert(abs(-10) == 10);
  assert(add(10, 10) == 20);
  const char* s = "12";
  assert(strlen(s) == 2);
  print("hello\n");
}
