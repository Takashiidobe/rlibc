#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * `int`
 */
typedef int CInt;

/**
 * 8-bit Char
 */
typedef char CChar;

CInt abs(CInt i);

uint32_t add(uint32_t a, uint32_t b);

bool isupper(CInt c);

uintptr_t strlen(const CChar *s);

void print(const CChar *print_str);

void vec(void);
