/*
 * Trivial implementation of a prime sieve
 * Based on pseudocode from https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
 */
#include "wasm.h"
#include <stdint.h>

#define WASM_EXPORT(name) __attribute__((export_name(#name))) name

int WASM_EXPORT(prime)(uint8_t *a, int n) {
  if (n < 2) {
    return 0;
  }

  for (int i = 2; i <= sqrtf(n); i++) {
    if (a[i]) {
      continue;
    }
    for (int j = i * i, c = 0; j < n; j = i * i + c * i, c++) {
      a[j] = 1;
    }
  }

  for (int i = n - 1; i >= 2; i--) {
    if (!a[i]) {
      return i;
    }
  }

  // Unreachable unless prime numbers stop existing
  return 0;
}
