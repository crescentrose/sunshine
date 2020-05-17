mod sunshine;
extern crate structopt;

use structopt::StructOpt;
use sunshine::{Opt, calculate};

fn main() {
    let opt = Opt::from_args();

    match calculate(opt) {
        Ok(()) => (),
        Err(error) => eprintln!("{}", error)
    }
}

