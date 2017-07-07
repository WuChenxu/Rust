use std::env;

fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invlid number:{}", n);
    }
    
    n == 5
}

fn main(){
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap();
    let n: i32 = arg.parse().unwrap();
    println!("{}", 2*n);
    guess(11);
}
