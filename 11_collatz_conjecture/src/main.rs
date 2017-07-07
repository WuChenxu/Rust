use std::env;

fn collatz_conjecture(number: u64) -> u64 {
    print!("num:{}    ", number);
    let mut n = number;
    let mut c = 0;
    loop {
//        println!("{}", n);
        if n == 1 {
            break;
        } else if n % 2 != 0 {
            n = 3 *n +1;
        } else {
            n = n/2;
        }
        
        c = c + 1;
    }
        println!("{}", c);
    c
}
    
fn collatz_conjecture2(N: u64)  -> u64{
    println!("{}", N);
    if N == 1 { return 1;}
    match N % 2 {
        0 => { collatz_conjecture2(N/2)}
        _ => { collatz_conjecture2(N*3+1)}
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: please provide a number as argument.");
        return;
    }   
    
    let i =  args[1].parse::<u64>().ok().expect("Please input an int");;
    //collatz_conjecture2(5476377146882523136);
    //collatz_conjecture2(i);
    for x in 1..1000000 {
        if i == collatz_conjecture(x) {
            println!("the lowest number req is {}", x);
            break;
        }
    }    

}

