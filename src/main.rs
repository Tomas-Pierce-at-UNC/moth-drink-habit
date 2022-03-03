
extern crate libc;

use std::env;
use std::fs;

mod phantom;

fn main() {
    let name = env::args().nth(1).unwrap();
    if name.ends_with(".cine") {
        let mut cine = fs::File::open(name).unwrap();
        let s_head = phantom::cine_header_3(&mut cine).unwrap();
        let b_head = phantom::cine_header_2(&mut cine).unwrap();
        let framerate = phantom::get_fps(s_head);
        let bit_depth = phantom::get_bit_depth(b_head);
        let size = phantom::get_size_of_images(b_head);
        println!("image size: {} bytes", size);
        println!("fps: {}", framerate);
        println!("bit depth : {}", bit_depth);
    } else {
        println!("not a cine");
    }
}
