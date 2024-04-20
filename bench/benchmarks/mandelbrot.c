/* The Computer Language Benchmarks Game
 * https://salsa.debian.org/benchmarksgame-team/benchmarksgame/

   contributed by Greg Buchholz
   adapted by Paul Clavier
*/

#include "wasm.h"

uint32_t WASM_EXPORT(mandelbrot)(int dim) {
  int w, h, bit_num = 0;
  char byte_acc = 0;
  int i, iter = 50;
  float x, y, limit = 2.0;
  float Zr, Zi, Cr, Ci, Tr, Ti;

  w = h = dim;

  uint32_t checksum = 0;

  for (y = 0; y < h; ++y) {
    for (x = 0; x < w; ++x) {
      Zr = Zi = Tr = Ti = 0.0;
      Cr = (2.0f * x / w - 1.5f);
      Ci = (2.0f * y / h - 1.0f);

      for (i = 0; i < iter && (Tr + Ti <= limit * limit); ++i) {
        Zi = 2.0f * Zr * Zi + Ci;
        Zr = Tr - Ti + Cr;
        Tr = Zr * Zr;
        Ti = Zi * Zi;
      }

      byte_acc <<= 1;
      if (Tr + Ti <= limit * limit)
        byte_acc |= 0x01;

      ++bit_num;

      if (bit_num == 8) {
        checksum += byte_acc;
        byte_acc = 0;
        bit_num = 0;
      } else if (x == w - 1) {
        byte_acc <<= (8 - w % 8);
        checksum -= byte_acc;
        byte_acc = 0;
        bit_num = 0;
      }
    }
  }

  return checksum;
}

// #include <stdio.h>

// int main() {
//   uint32_t checksum = mandelbrot(100);
//   printf("Checksum: %u\n", checksum);
//   return 0;
// }
