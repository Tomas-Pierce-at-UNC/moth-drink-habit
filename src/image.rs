
use std::io;
use std::fs;
use std::io::{Read,Seek};

pub fn read_image(cine :&mut fs::File, offset :u64, image_size :usize) -> io::Result<Vec<u8>> {
    cine.seek(io::SeekFrom::Start(offset))?;
    let mut annote_size_bytes :[u8;4] = [0,0,0,0];
    cine.read_exact(&mut annote_size_bytes)?;
    let annote_size = u32::from_le_bytes(annote_size_bytes);
    cine.seek(io::SeekFrom::Start(offset + (annote_size as u64)))?;
    let mut data = vec![0u8;image_size];
    cine.read_exact(&mut data)?;
    Ok(data)
}
