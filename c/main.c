#include "header.h"
#include <stdio.h>
#include <assert.h>

int main() {
  assert(abs(-10) == 10);
  assert(add(10, 10) == 20);
  const char* s = "12";
  assert(strlen(s) == 2);
  test_vec();
  print("hello");
  print_async("hello async");
  uint64_t num = rand_u64();

  unsigned long long arr[] = {1,2,3,4,5};
  struct CArray shuffled = {};
  shuffle(arr, 5, &shuffled);

  for (int i = 0; i < shuffled.size; i++) {
    printf("%llu\n", shuffled.ptr[i]);
  }
}
