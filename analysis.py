
import glob
from skimage import io as skio
import numpy as np

def get_data():
    frames = glob.glob("frames/*")
    frames.sort()
    images = []
    for frame_name in frames:
        frame = skio.imread(frame_name)
        images.append(frame)

    return np.array(images)

if __name__ == '__main__':
    from matplotlib import pyplot
