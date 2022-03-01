
use std::fs;
use std::io;
use std::io::{Read,Seek};

use crate::head;

use crate::take::TakeFixed;

pub fn get_image_offsets(cine :&mut fs::File, head :&head::CineHeader) -> io::Result<Vec<u64>> {
    let count = head.get_image_count();
    let place = head.get_images_array_offset();
    let mut offsets :Vec<u64> = Vec::with_capacity(count as usize);
    let mut arena :[u8;8] = [0u8;8];
    cine.seek(io::SeekFrom::Start(place))?;
    for _i in 0..count {
        cine.read_exact(&mut arena)?;
        /* is int64_t in documentation */
        let this = i64::from_le_bytes(arena);
        /* rust uses u64 for file offsets */
        let position = this as u64;
        offsets.push(position);
        /* move to the next integer */
        cine.seek(io::SeekFrom::Current(8i64))?;
    }
    Ok(offsets)
}

pub fn read_image_data(cine :&mut fs::File, offset :u64, size :usize) -> io::Result<Vec<u8>> {
    cine.seek(io::SeekFrom::Start(offset))?;
    let mut arena :[u8;4] = [0u8;4];
    cine.read_exact(&mut arena)?;
    let annote_size = u32::from_le_bytes(arena) as u64;
    cine.seek(io::SeekFrom::Start(offset + annote_size))?;
    let mut bytes = vec![0u8;size];
    cine.read_exact(&mut bytes[..])?;
    Ok(bytes)
}

pub struct FrameStepper {
    name :String,
    file :fs::File,
    chead :head::CineHeader,
    bhead :head::BitmapHeader,
    offsets :Vec<u64>,
    index :usize,
}


impl FrameStepper {
    pub fn new(name :&String) -> io::Result<FrameStepper> {
        let namecopy = name.clone();
        let mut file :fs::File = fs::File::open(name)?;
        let bhead = head::BitmapHeader::take_fixed(&mut file)?;
        let chead = head::CineHeader::take_fixed(&mut file)?;
        let offsets = get_image_offsets(&mut file, &chead)?;
        let f = FrameStepper {
            name : namecopy,
            file : file,
            chead : chead,
            bhead : bhead,
            offsets : offsets,
            index : 0,
        };
        Ok(f)
    }

    pub fn next(&mut self) -> io::Result<Vec<u8>> {
        if self.index >= self.offsets.len() - 1 {
            Err(io::Error::new(io::ErrorKind::Other, "exceeded bounds"))
        } else {
            let data = read_image_data(&mut self.file, self.offsets[self.index], self.bhead.get_size_in_bytes())?;
            self.index += 1;
            Ok(data)
        }
    }

    pub fn has_next(&mut self) -> bool {
        self.index < self.offsets.len() - 1
    }

    pub fn bytes_count(&self) -> u64 {
        self.bhead.get_size_in_bytes() as u64
    }
}
