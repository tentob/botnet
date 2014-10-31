extern crate getopts;
use getopts::{optopt, optflag, getopts, OptGroup};
use std::os;

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let opts = [
        optflag("h","help","prints this message")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string() ) }
    };

    if matches.opt_present("h") {
        println!("Usage: {} [options]", program);
        println!("-h\t--help\t\tShows this message");
        return;
    }
}
