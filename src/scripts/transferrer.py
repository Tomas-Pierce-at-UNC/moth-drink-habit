
import os
import glob
import numpy
from skimage import io as skio
from matplotlib import pyplot

if not os.path.isdir("./frames"):
    os.mkdir("frames")

count = len(glob.glob("./frames/*"))

# list im_buf is defined by calling Rust code

frame = numpy.array(im_buf, dtype=numpy.uint8)
arranged = numpy.reshape(frame, (600,800))
flipped = numpy.flip(arranged,0)

#skio.imshow(flipped)
#pyplot.show()

skio.imsave(f"./frames/frame{count:05}.tiff",flipped)

