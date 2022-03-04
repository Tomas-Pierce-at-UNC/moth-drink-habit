
extern crate libc;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyByteArray;

use std::env;
use std::fs;

mod phantom;
mod image;

fn main() {
    let script = include_str!("./scripts/transferrer.py");
    let name = env::args().nth(1).unwrap();
    if name.ends_with(".cine") {
        let mut cine = fs::File::open(name).unwrap();
        let b_head = phantom::cine_header_2(&mut cine).unwrap();
        let size = phantom::get_size_of_images(b_head);
        let offsets = phantom::locate_images(&mut cine).unwrap();
        for offset in offsets {
            let image = image::read_image(&mut cine, offset, size as usize).unwrap();
            //let b_array = PyByteArray::new(&image[..]);
            //let locals = PyDict::new();
            //locals.set_item("im_buf", b_array).unwrap();
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("im_buf", image).unwrap();
                //let numpy = PyModule::import(py, "numpy").unwrap();
                py.run(script,None, Some(locals)).unwrap();
            });
        }
        //let im0 = image::read_image(&mut cine, offsets[1], size as usize).unwrap();

        //let text = include_str!("scripts/example.py");
        /*
        Python::with_gil(|py| {
            py.run(text,None,None).unwrap();
        });
        */
        
        //println!("{:?}",im0);
    } else {
        println!("not a cine");
    }
}
