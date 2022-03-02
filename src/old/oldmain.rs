
use std::env;
use std::io;

mod head;
mod take;
mod cine;



fn main() {
    let args = env::args();
    for arg in args {
        if arg.ends_with(".cine") {
            match run(&arg) {
                Ok(_) => {println!("processed {} successfully", arg);},
                Err(e) => {println!("processing {} failed with error: {}", arg, e);},
            };
        }
    }
}

fn run(cine_name :&String) -> io::Result<()> {
    let mut stepper = cine::FrameStepper::new(cine_name)?;
    while stepper.has_next() {
        let data = stepper.next()?;
    }
    Ok(())
}
