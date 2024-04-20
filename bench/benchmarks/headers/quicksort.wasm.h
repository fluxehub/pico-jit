unsigned char quicksort_wasm[] = {
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x1c, 0x05, 0x60,
  0x03, 0x7f, 0x7f, 0x7f, 0x00, 0x60, 0x04, 0x7f, 0x7f, 0x7f, 0x7f, 0x00,
  0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x60, 0x01, 0x7f, 0x01, 0x7f, 0x60,
  0x00, 0x00, 0x02, 0x0d, 0x01, 0x03, 0x65, 0x6e, 0x76, 0x05, 0x61, 0x62,
  0x6f, 0x72, 0x74, 0x00, 0x01, 0x03, 0x06, 0x05, 0x00, 0x02, 0x00, 0x03,
  0x04, 0x05, 0x04, 0x01, 0x01, 0x02, 0x02, 0x06, 0x06, 0x01, 0x7f, 0x01,
  0x41, 0x00, 0x0b, 0x07, 0x11, 0x02, 0x04, 0x73, 0x6f, 0x72, 0x74, 0x00,
  0x04, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x02, 0x00, 0x08, 0x01,
  0x05, 0x0c, 0x01, 0x0a, 0x0a, 0x95, 0x04, 0x05, 0x2e, 0x00, 0x20, 0x01,
  0x20, 0x00, 0x41, 0x14, 0x6b, 0x28, 0x02, 0x10, 0x41, 0x02, 0x76, 0x4f,
  0x04, 0x40, 0x41, 0x90, 0x0a, 0x41, 0xd0, 0x08, 0x41, 0xdd, 0x00, 0x41,
  0x29, 0x10, 0x00, 0x00, 0x0b, 0x20, 0x00, 0x20, 0x01, 0x41, 0x02, 0x74,
  0x6a, 0x20, 0x02, 0x36, 0x02, 0x00, 0x0b, 0x2c, 0x00, 0x20, 0x01, 0x20,
  0x00, 0x41, 0x14, 0x6b, 0x28, 0x02, 0x10, 0x41, 0x02, 0x76, 0x4f, 0x04,
  0x40, 0x41, 0x90, 0x0a, 0x41, 0xd0, 0x08, 0x41, 0xce, 0x00, 0x41, 0x29,
  0x10, 0x00, 0x00, 0x0b, 0x20, 0x00, 0x20, 0x01, 0x41, 0x02, 0x74, 0x6a,
  0x28, 0x02, 0x00, 0x0b, 0x8f, 0x01, 0x01, 0x04, 0x7f, 0x20, 0x01, 0x20,
  0x02, 0x48, 0x20, 0x01, 0x20, 0x02, 0x72, 0x41, 0x00, 0x4e, 0x71, 0x04,
  0x40, 0x20, 0x00, 0x20, 0x01, 0x10, 0x02, 0x21, 0x05, 0x20, 0x01, 0x41,
  0x01, 0x6b, 0x21, 0x04, 0x20, 0x02, 0x41, 0x01, 0x6a, 0x21, 0x03, 0x03,
  0x40, 0x03, 0x40, 0x20, 0x00, 0x20, 0x04, 0x41, 0x01, 0x6a, 0x22, 0x04,
  0x10, 0x02, 0x20, 0x05, 0x48, 0x0d, 0x00, 0x0b, 0x03, 0x40, 0x20, 0x00,
  0x20, 0x03, 0x41, 0x01, 0x6b, 0x22, 0x03, 0x10, 0x02, 0x20, 0x05, 0x4a,
  0x0d, 0x00, 0x0b, 0x20, 0x03, 0x20, 0x04, 0x4c, 0x04, 0x40, 0x01, 0x05,
  0x20, 0x00, 0x20, 0x04, 0x10, 0x02, 0x21, 0x06, 0x20, 0x00, 0x20, 0x04,
  0x20, 0x00, 0x20, 0x03, 0x10, 0x02, 0x10, 0x01, 0x20, 0x00, 0x20, 0x03,
  0x20, 0x06, 0x10, 0x01, 0x0c, 0x01, 0x0b, 0x0b, 0x20, 0x00, 0x20, 0x01,
  0x20, 0x03, 0x10, 0x03, 0x20, 0x00, 0x20, 0x03, 0x41, 0x01, 0x6a, 0x20,
  0x02, 0x10, 0x03, 0x0b, 0x0b, 0x9d, 0x02, 0x01, 0x08, 0x7f, 0x20, 0x00,
  0x41, 0xff, 0xff, 0xff, 0xff, 0x00, 0x4b, 0x04, 0x40, 0x41, 0xa0, 0x08,
  0x41, 0xd0, 0x08, 0x41, 0x33, 0x41, 0x3c, 0x10, 0x00, 0x00, 0x0b, 0x20,
  0x00, 0x41, 0x02, 0x74, 0x22, 0x04, 0x41, 0xec, 0xff, 0xff, 0xff, 0x03,
  0x4b, 0x04, 0x40, 0x41, 0x90, 0x09, 0x41, 0xd0, 0x09, 0x41, 0xd6, 0x00,
  0x41, 0x1e, 0x10, 0x00, 0x00, 0x0b, 0x20, 0x04, 0x41, 0x10, 0x6a, 0x22,
  0x03, 0x41, 0xfc, 0xff, 0xff, 0xff, 0x03, 0x4b, 0x04, 0x40, 0x41, 0x90,
  0x09, 0x41, 0xd0, 0x09, 0x41, 0x21, 0x41, 0x1d, 0x10, 0x00, 0x00, 0x0b,
  0x23, 0x00, 0x21, 0x02, 0x23, 0x00, 0x41, 0x04, 0x6a, 0x22, 0x06, 0x20,
  0x03, 0x41, 0x13, 0x6a, 0x41, 0x70, 0x71, 0x41, 0x04, 0x6b, 0x22, 0x03,
  0x6a, 0x22, 0x05, 0x3f, 0x00, 0x22, 0x07, 0x41, 0x10, 0x74, 0x41, 0x0f,
  0x6a, 0x41, 0x70, 0x71, 0x22, 0x08, 0x4b, 0x04, 0x40, 0x20, 0x07, 0x20,
  0x05, 0x20, 0x08, 0x6b, 0x41, 0xff, 0xff, 0x03, 0x6a, 0x41, 0x80, 0x80,
  0x7c, 0x71, 0x41, 0x10, 0x76, 0x22, 0x08, 0x20, 0x07, 0x20, 0x08, 0x4a,
  0x1b, 0x40, 0x00, 0x41, 0x00, 0x48, 0x04, 0x40, 0x20, 0x08, 0x40, 0x00,
  0x41, 0x00, 0x48, 0x04, 0x40, 0x00, 0x0b, 0x0b, 0x0b, 0x20, 0x05, 0x24,
  0x00, 0x20, 0x02, 0x20, 0x03, 0x36, 0x02, 0x00, 0x20, 0x06, 0x41, 0x04,
  0x6b, 0x22, 0x02, 0x41, 0x00, 0x36, 0x02, 0x04, 0x20, 0x02, 0x41, 0x00,
  0x36, 0x02, 0x08, 0x20, 0x02, 0x41, 0x04, 0x36, 0x02, 0x0c, 0x20, 0x02,
  0x20, 0x04, 0x36, 0x02, 0x10, 0x20, 0x06, 0x41, 0x10, 0x6a, 0x22, 0x02,
  0x41, 0x00, 0x20, 0x04, 0xfc, 0x0b, 0x00, 0x03, 0x40, 0x20, 0x00, 0x20,
  0x01, 0x4a, 0x04, 0x40, 0x20, 0x02, 0x20, 0x01, 0x20, 0x00, 0x20, 0x01,
  0x6b, 0x10, 0x01, 0x20, 0x01, 0x41, 0x01, 0x6a, 0x21, 0x01, 0x0c, 0x01,
  0x0b, 0x0b, 0x20, 0x02, 0x41, 0x00, 0x20, 0x00, 0x41, 0x01, 0x6b, 0x10,
  0x03, 0x20, 0x02, 0x0b, 0x07, 0x00, 0x41, 0xbc, 0x0a, 0x24, 0x00, 0x0b,
  0x0b, 0x91, 0x02, 0x0a, 0x00, 0x41, 0x8c, 0x08, 0x0b, 0x01, 0x2c, 0x00,
  0x41, 0x98, 0x08, 0x0b, 0x23, 0x02, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00,
  0x00, 0x49, 0x00, 0x6e, 0x00, 0x76, 0x00, 0x61, 0x00, 0x6c, 0x00, 0x69,
  0x00, 0x64, 0x00, 0x20, 0x00, 0x6c, 0x00, 0x65, 0x00, 0x6e, 0x00, 0x67,
  0x00, 0x74, 0x00, 0x68, 0x00, 0x41, 0xbc, 0x08, 0x0b, 0x01, 0x3c, 0x00,
  0x41, 0xc8, 0x08, 0x0b, 0x2d, 0x02, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00,
  0x00, 0x7e, 0x00, 0x6c, 0x00, 0x69, 0x00, 0x62, 0x00, 0x2f, 0x00, 0x73,
  0x00, 0x74, 0x00, 0x61, 0x00, 0x74, 0x00, 0x69, 0x00, 0x63, 0x00, 0x61,
  0x00, 0x72, 0x00, 0x72, 0x00, 0x61, 0x00, 0x79, 0x00, 0x2e, 0x00, 0x74,
  0x00, 0x73, 0x00, 0x41, 0xfc, 0x08, 0x0b, 0x01, 0x3c, 0x00, 0x41, 0x88,
  0x09, 0x0b, 0x2f, 0x02, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x41,
  0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x63, 0x00, 0x61, 0x00, 0x74,
  0x00, 0x69, 0x00, 0x6f, 0x00, 0x6e, 0x00, 0x20, 0x00, 0x74, 0x00, 0x6f,
  0x00, 0x6f, 0x00, 0x20, 0x00, 0x6c, 0x00, 0x61, 0x00, 0x72, 0x00, 0x67,
  0x00, 0x65, 0x00, 0x41, 0xbc, 0x09, 0x0b, 0x01, 0x3c, 0x00, 0x41, 0xc8,
  0x09, 0x0b, 0x25, 0x02, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x7e,
  0x00, 0x6c, 0x00, 0x69, 0x00, 0x62, 0x00, 0x2f, 0x00, 0x72, 0x00, 0x74,
  0x00, 0x2f, 0x00, 0x73, 0x00, 0x74, 0x00, 0x75, 0x00, 0x62, 0x00, 0x2e,
  0x00, 0x74, 0x00, 0x73, 0x00, 0x41, 0xfc, 0x09, 0x0b, 0x01, 0x3c, 0x00,
  0x41, 0x88, 0x0a, 0x0b, 0x2b, 0x02, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00,
  0x00, 0x49, 0x00, 0x6e, 0x00, 0x64, 0x00, 0x65, 0x00, 0x78, 0x00, 0x20,
  0x00, 0x6f, 0x00, 0x75, 0x00, 0x74, 0x00, 0x20, 0x00, 0x6f, 0x00, 0x66,
  0x00, 0x20, 0x00, 0x72, 0x00, 0x61, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x65
};
unsigned int quicksort_wasm_len = 912;
