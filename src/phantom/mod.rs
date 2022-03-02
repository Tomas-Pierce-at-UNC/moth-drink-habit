
mod cine_types;
mod null_error;

use null_error::NullError;

pub use cine_types::{
    CineFileHeader,
    BitmapInfoHeader,
    Setup
};

extern {
    fn get_cine_head(fd :libc::c_int) -> *const CineFileHeader;
    fn get_bitmap_head(fd :libc::c_int) -> *const BitmapInfoHeader;
    fn get_setup_head(fd :libc::c_int) -> *const Setup;

    fn get_image_count(cfh :*const CineFileHeader) -> i32;
    fn get_image_offsets(fd :libc::c_int, cfh :*const CineFileHeader) -> *const i64;
}

use std::os::unix::io::AsRawFd;
use std::fs;

use std::ptr;

pub fn cine_header_1(file :&mut fs::File) -> Result<*const CineFileHeader, NullError> {
    let fd = file.as_raw_fd();
    let chead = unsafe {
        get_cine_head(fd)
    };
    if chead.is_null() {
        Err(NullError)
    } else {
        Ok(chead)
    }
}

pub fn cine_header_2(file :&mut fs::File) -> Result<*const BitmapInfoHeader, NullError> {
    let fd = file.as_raw_fd();
    let bhead = unsafe {
        get_bitmap_head(fd)
    };
    if bhead.is_null() {
        Err(NullError)
    } else {
        Ok(bhead)
    }
}

pub fn cine_header_3(file :&mut fs::File) -> Result<*const Setup, NullError> {
    let fd = file.as_raw_fd();
    let setup = unsafe {
        get_setup_head(fd)
    };
    if setup.is_null() {
        Err(NullError)
    } else {
        Ok(setup)
    }
}

pub fn count_images(cfh :*const CineFileHeader) -> i32 {
    unsafe {
        get_image_count(cfh)
    }
}

fn image_offsets(file :&mut fs::File) -> Result<*const i64, NullError> {
    let fd = file.as_raw_fd();
    let cfh = cine_header_1(file)?;
    let offsets = unsafe {
        get_image_offsets(fd, cfh)
    };
    if offsets.is_null() {
        Err(NullError)
    } else {
        Ok(offsets)
    }
}

fn pointer_to_vector(offsets :*const i64, count :i32) -> Vec<u64> {
    let mut arena = Vec::new();
    for i in 0..count {
        let here = unsafe {
            offsets.offset(i as isize)
        };
        let value :i64 = unsafe {
            *here
        };
        let place = value as u64;
        arena.push(place);
    }
    arena
}

pub fn locate_images(file :&mut fs::File) -> Result<Vec<u64>, NullError> {
    let offsets = image_offsets(file)?;
    let cfh = cine_header_1(file)?;
    let count = count_images(cfh);
    let vector = pointer_to_vector(offsets, count);
    Ok(vector)
}
