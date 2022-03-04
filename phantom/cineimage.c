
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

uint8_t* get_image_bytes(int fd, int64_t offset) {
    off_t offset_pos = lseek(fd, offset, SEEK_SET);
    if(offset_pos == -1) return NULL;

    uint32_t annote_size = 0;

    ssize_t bits1 = read(fd, (void*) &annote_size, 4);

    if(bits1 != 4) return NULL;

    off_t offset_size = lseek(fd, offset + annote_size - 4, SEEK_SET);

    if(offset_size == -1) return NULL;

    uint32_t image_size = 0;

    ssize_t bits2 = read(fd, (void*) &image_size, 4);

    if(bits2 != 4) return NULL;

    off_t offset_pixels = lseek(fd, offset + annote_size, SEEK_SET);

    if(offset_pixels == -1) return NULL;

    uint8_t *arena = malloc(sizeof(uint8_t) * image_size);

    if(arena == NULL) return NULL;

    ssize_t bits3 = read(fd, (void*) arena, image_size);

    if(bits3 < image_size) {
        free(arena);
        return NULL;
    }

    return arena;
}
