use std::env;
use std::path::Path;
use std::fs;

fn main() {
    println!("current dir: {}", env::current_dir().unwrap().display());
    fs::rename("a.txt", "b.txt");
    
}
