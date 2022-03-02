
#include "cineheaders.h"
#include "cineutil.h"

#include <stdio.h>

int main() {
    printf("CINEFILEHEADER: %ld\n", sizeof(CINEFILEHEADER));
    printf("BITMAPINFOHEADER: %ld\n", sizeof(BITMAPINFOHEADER));
    printf("SETUP: %ld\n", sizeof(SETUP));
    return 0;
}


