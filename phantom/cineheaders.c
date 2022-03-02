
#include "cineheaders.h"

#include <stdlib.h>

#include <sys/types.h>
#include <unistd.h>

CINEFILEHEADER* get_cine_head(int fd) {
    off_t offset = lseek(fd, 0, SEEK_SET);
    if(offset == -1) {
        return NULL;
    }
    CINEFILEHEADER *cfh = malloc(sizeof(CINEFILEHEADER));
    if(cfh == NULL) {
        return NULL;
    }
    ssize_t bytes = read(fd, (void *) cfh, sizeof(CINEFILEHEADER));
    if(bytes < sizeof(CINEFILEHEADER)) {
        free(cfh);
        return NULL;
    }

    return cfh;
}

BITMAPINFOHEADER* get_bitmap_head(int fd) {
    CINEFILEHEADER *cfh = get_cine_head(fd);
    if(cfh == NULL) {
        return NULL;
    }
    off_t pos = (off_t) cfh->OffImageHeader;
    free(cfh);
    off_t offset = lseek(fd, pos, SEEK_SET);
    if(offset == -1) {
        return NULL;
    }
    BITMAPINFOHEADER *bih = malloc(sizeof(BITMAPINFOHEADER));
    if(bih == NULL) {
        return NULL;
    }
    ssize_t bytes = read(fd, (void *) bih, sizeof(BITMAPINFOHEADER));
    if(bytes < sizeof(BITMAPINFOHEADER)) {
        free(bih);
        return NULL;
    }
    return bih;
}

SETUP* get_setup_head(int fd) {
    CINEFILEHEADER *cfh = get_cine_head(fd);
    if(cfh == NULL) return NULL;

    off_t pos = (off_t) cfh->OffSetup;
    free(cfh);

    off_t offset = lseek(fd, pos, SEEK_SET);

    if(offset == -1) return NULL;

    SETUP *setup = malloc(sizeof(SETUP));

    if(setup == NULL) return NULL;

    ssize_t bytes = read(fd, (void *) setup, sizeof(SETUP));

    if(bytes < sizeof(SETUP)) {
        free(setup);
        return NULL;
    }

    return setup;

}
