#include "header.h"
#include <stdio.h>
#include <assert.h>
#include <stdarg.h>
#include "printc.h"
#include <time.h>

int main() {
  // assert(abs(-10) == 10);
  // assert(add(10, 10) == 20);
  // const char* s = "12";
  // assert(strlen(s) == 2);
  // test_vec();
  // println("hello");
  // print_async("hello async");
  // uint64_t num = rand_u64();

  // unsigned long long arr[] = {1,2,3,4,5};
  // struct CArray shuffled = new_carray();
  // shuffle(arr, 5, &shuffled);

  // for (int i = 0; i < shuffled.size; i++) {
  //   print("%llu ", shuffled.ptr[i]);
  // }


  // int map[101] = {};
  // for (int i = 0; i < 1e6; i++) {
  //   unsigned long long rand_num = rand_range(0, 100);
  //   map[rand_num]++;
  // }

  // for (int i = 0; i < 101; i++) {
  //   print("%d: %d\n", i, map[i]);
  // }


  for (int i = 0; i < 1e1; i++) {
    unsigned long long* vec = malloc(sizeof(unsigned long long) * 10);
    for (int i = 0; i < 10; i++) {
      cvec_push(vec, i);
      int x = cvec_get(vec, i);
      printf("inserted: %d\n", x);
    }
    for (int i = 0; i < 10; i++) {
      int x = cvec_pop(vec);
      printf("popped: %d\n", x);
    }
  }

}
