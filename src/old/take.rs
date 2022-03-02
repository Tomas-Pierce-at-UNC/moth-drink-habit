
use std::io::{
    Read,
    Seek,
};

use std::io::SeekFrom;
use std::io;

use std::mem;

use std::fs;

use crate::head;

pub trait TakeAt: Copy + Clone {

    type Source : Read + Seek;

    fn take_at(src :&mut Self::Source, position :SeekFrom) -> io::Result<Self> {
        src.seek(position)?;
        let size = mem::size_of::<Self>();
        let mut buffer = vec![0u8;size];
        src.read_exact(&mut buffer[..])?;
        let ptr = buffer.as_ptr();
        let sptr :*const Self = ptr as *const Self;
        let item :Self = unsafe { *sptr };
        Ok(item)
    }

}

pub trait TakeFixed: TakeAt {

    const POSITION :u64;

    fn take_fixed(src :&mut Self::Source) -> io::Result<Self> {
        Self::take_at(src, SeekFrom::Start(Self::POSITION))
    }

}

impl TakeAt for head::CineHeader {
    type Source = fs::File;
}

impl TakeAt for head::BitmapHeader {
    type Source = fs::File;
}

impl TakeAt for head::SetupHeader {
    type Source = fs::File;
}

impl TakeFixed for head::CineHeader {
    const POSITION :u64 = 0x0;
}

impl TakeFixed for head::BitmapHeader {
    const POSITION :u64 = 0x2C;
}

impl TakeFixed for head::SetupHeader {
    const POSITION :u64 = 0x54;
}
