extern crate structopt;

mod sunshine;

use std::process;
use structopt::StructOpt;
use sunshine::*;

fn main() {
    // Have to `use structopt::StructOpt`, otherwise the `from_args` method does not get imported
    let opt = Opt::from_args();

    // I still don't understand fully why we need to clone here, but we have to be polite to the
    // compiler, as the compiler is our friend.
    // I assume that we transfer ownership of the `opt` constant to the `calculate` function, and
    // when it ends it automatically frees the memory occupied by the options struct, so we could
    // either borrow it there or just clone the one thing we need here.
    let format = opt.format.clone();
    let simple = opt.simple;

    match calculate(opt) {
        Ok(result) => {
            // This feels ugly
            if simple {
                match result.time_of_day {
                    TimeOfDay::Day => println!("day"),
                    TimeOfDay::Night => println!("night"),
                }
            } else {
                println!("sunrise: {}", result.sunrise.format(&format[..]));
                println!("sunset: {}", result.sunset.format(&format[..]));
            }
            process::exit(0)
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}
