
#include "cineheaders.h"
#include "cineutil.h"

#include <stdlib.h>
#include <stdint.h>
#include <sys/types.h>
#include <unistd.h>

int32_t get_image_count(CINEFILEHEADER *cfh) {
    return cfh->ImageCount;
}

int64_t* get_image_offsets(int fd, CINEFILEHEADER *cfh) {
    if(cfh == NULL) return NULL;
    
    size_t im_count = get_image_count(cfh);
    size_t byte_count = im_count * sizeof(int64_t);

    off_t pos = (off_t)(cfh->OffImageOffsets);

    off_t offset = lseek(fd, pos, SEEK_SET);
    
    if(offset == -1) return NULL;

    int64_t *offsets = calloc(im_count, sizeof(int64_t));

    if(offsets == NULL) return NULL;

    ssize_t bytes = read(fd, (void *) offsets, byte_count);

    if(bytes < byte_count) {
        free(offsets);
        return NULL;
    }

    return offsets;
    
}
