use std::{thread, time};
extern crate webbrowser;


const TARGET_URL:&'static str = "http://www.youtube.com/watch?v=dQw4w9WgXcQ";
const BREAK_TIME_IN_SEC:u64 = 10;
fn main() {
    for x in 0..2 {
        println!("{}", x);
        thread::sleep(time::Duration::from_secs(BREAK_TIME_IN_SEC));
        if webbrowser::open(TARGET_URL).is_ok() {
        	println!("take a break now.");
        }
    }

}
