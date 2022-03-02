
#ifndef CINE_UTIL
#define CINE_UTIL

#include "cineheaders.h"

#include <stdint.h>

int32_t get_image_count(CINEFILEHEADER *cfh);

int64_t* get_image_offsets(int fd, CINEFILEHEADER *cfh);

#endif
