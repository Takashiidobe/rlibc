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
typedef uint8_t CChar;

CInt abs(CInt i);

uint32_t add(uint32_t a, uint32_t b);

bool isupper(CInt c);

/**
 * # Safety
 * This should be safe
 */
uintptr_t strlen(const CChar *s);
