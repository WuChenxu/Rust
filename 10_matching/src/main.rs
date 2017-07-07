
fn main() {
    let x = 1;
    let tup = (false, 41);
    match x {
        1 => println!("this is 1"),
          _ => println!("this is others"),
    }

    match tup {
        (true, 20...26) => println!("True and range"),
            (true, _)       => println!("True and inclusive"),
            (_, 40...49)    => println!("Any and range"),
            (_, _)          => println!("default"),
    }
    Fizz_Buzz_Test();
    println!("");
    Fizz_Buzz_Test2();

}

fn Fizz_Buzz_Test2() {
    for i in 1..101 {
        match (i%3, i%5) {
            (0, 0) => print!("FizzBuzz"),
                (0, _) => print!("Fizz"),
                (_, 0) => print!("Buzz"),
                (_, _) => print!("{}",i),
        }
    }
}

fn Fizz_Buzz_Test()
{
    for i in 1..101 {
        match i {
            i if  ((i % 3 == 0) & (i % 5 == 0)) => print!("FizzBuzz"),
              i if  (i % 3 == 0)                 => print!("Fizz"),
              i if  (i % 5 == 0)                 => print!("Buzz"),
              _                            => print!("{}", i),
        }
    }

}
