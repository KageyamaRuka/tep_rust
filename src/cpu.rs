use std::env;
use std::process;
use std::thread;
use std::time;

pub fn cpu() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("usage: cpu <string>\n");
        process::exit(1);
    }
    let arg = args.nth(1).unwrap();
    loop {
        thread::sleep(time::Duration::new(1, 0));
        println!("{arg}");
    }
}
