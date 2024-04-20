#define WASM_IMPORT(module, name)                                              \
  __attribute__((import_module(#module))) __attribute__((import_name(#name)))  \
  name
#define WASM_EXPORT(name) __attribute__((export_name(#name))) name
#define sqrtf __builtin_sqrtf

#include <stdint.h>
extern uint32_t __heap_base;
