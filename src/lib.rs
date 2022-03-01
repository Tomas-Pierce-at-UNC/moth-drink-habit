
pub mod take;
pub mod head;
pub mod cine;

use std::ffi;
use std::ptr;
use std::mem;

/* I know perfectly well I should be shot for this.
 * I'm doing it anyway because I need to be able to
 * get the results from Python 3, and this seemed
 * like the easiest way to do it.
 */


#[no_mangle]
pub unsafe extern "C" fn get_image_i(name :*const libc::c_char, index :u64, size_out :*mut u64) -> * const u8 {
    let fname = ffi::CStr::from_ptr(name);
    let filename = fname.to_str().unwrap().to_owned();
    let mut stepper = cine::FrameStepper::new(&filename).unwrap();
    *size_out = stepper.bytes_count();
    let mut thing = vec![0u8;1];
    for _i in 0..index {
        if stepper.has_next() {
            thing = stepper.next().unwrap();
        } else {
            return ptr::null();
        }
    }
    thing.as_ptr()
}
