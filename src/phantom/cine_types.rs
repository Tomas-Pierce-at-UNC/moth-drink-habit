
#[repr(C)]
pub struct CineFileHeader {
    _data :[u8;44],
}

#[repr(C)]
pub struct BitmapInfoHeader {
    _data :[u8;40],
}

#[repr(C)]
pub struct Setup {
    _data :[u8;7240],
}


