
#include "cineheaders.h"
#include "cineutil.h"
#include "cineimage.h"

#include <stdlib.h>
#include <stdint.h>
#include <sys/types.h>
#include <unistd.h>

uint32_t get_image_size(BITMAPINFOHEADER *bih) {
    return bih->biSizeImage;
}


