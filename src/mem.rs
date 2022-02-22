use std::alloc;
use std::env;
use std::process;
use std::thread;
use std::time;

pub unsafe fn mem() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("usage: mem <value>");
        process::exit(1);
    }
    let layout = alloc::Layout::new::<u32>();
    let p = alloc::alloc(layout);
    assert!(!p.is_null());
    println!("({}) addr pointed to by p: {:?}", process::id(), p);
    *p = args.nth(1).unwrap().parse().unwrap();
    loop {
        thread::sleep(time::Duration::new(1, 0));
        *p = *p + 1;
        println!("({}) value of p: {}", process::id(), *p);
    }
}
