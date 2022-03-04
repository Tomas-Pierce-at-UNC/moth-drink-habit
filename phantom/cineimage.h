
#ifndef CINE_IMAGE
#define CINE_IMAGE

#include "cineheaders.h"
#include "cineutil.h"

#include <stdint.h>

uint32_t get_image_size(BITMAPINFOHEADER *bih);

uint8_t *get_image_bytes(int fd, int64_t offset);

#endif

