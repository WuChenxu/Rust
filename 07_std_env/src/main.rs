use std::env;

/*
std::env
Inspection and manipulation of the process's environment.

This module contains functions to inspect various aspects such as environment variables, process arguments, the current directory, and various other important directories.
*/
fn main() {
    println!("env::consts:==========");
    println!("ARCH:{:>10}", env::consts::ARCH);
    println!("DLL_EXTENSION:{:>10}", env::consts::DLL_EXTENSION);
    println!("DLL_PREFIX:{:>10}", env::consts::DLL_PREFIX);
    println!("DLL_SUFFIX:{:>10}", env::consts::DLL_SUFFIX);
    println!("EXE_EXTENSION:{:>10}", env::consts::EXE_EXTENSION);
    println!("FAMILY:{:>10}", env::consts::FAMILY);
    println!("OS:{:>10}", env::consts::OS);
    println!("\nenv::functions:======");
    println!("args:{:?}", env::args().collect::<Vec<String>>());
    println!("args_os:{:?}", env::args_os().collect::<Vec<_>>());
    println!("current_dir:{:?}", env::current_dir().unwrap());
    println!("current_exe:{:?}", env::current_exe().unwrap());
    println!("home_dir:{:?}", env::home_dir().unwrap());
    println!("join_paths:{:?}", env::join_paths().collect::<Vec<_>>());

}
