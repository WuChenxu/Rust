use std::fmt;

struct I32(i32);

impl fmt::LowerExp for I32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.0 as f32;
        write!(f, "{:e}", val)
    }
}
fn main() {
    //1. basic usage
    print!("{}.print is output to console w/o newline.", 1);
    print!("Continue...");
    print!("\n");
    println!("{}.println is string to console w/ newline.", 2);
    let str = format!("{}.format is output to a string.", 3);
    assert_eq!(str, "3.format is output to a string.");
    println!("{}", str);

    //2. with argument
    println!("{}, {}", "Hello", "world");
    println!("{0}, this is {1}. {1}, this is {0}.", "Hellen", "Tom");
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brwon fox", verb="jumps over");

    //3. format
    println!("dec:{:04}, bin:{0:b}, oct:{0:o}, hex:{0:x} {0:X}", 42);
    println!("dec:{:04}, bin:{0:#b}, oct:{0:#o}, hex:{0:#x} {0:#0X}", 42);
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    let pi = 3.1415926;
    //keep 2 digits fractional part
    println!("Pi is {:.2}", pi);
    //with sign
    println!("Pi is {:+.*}", 2, pi);
    println!("{:+.2}", -1);
    println!("{:+.2}", -1.0);
    //without fractional part
    println!("{:+.0}", 2.71828);
    //pad in left with 0, width is 2
    println!("{:0>2}", 5);
    println!("{:02}", 5);
    //pad x , middle alignment
    println!("{:x^10}", 5);
    //pad in right with x, width 4
    println!("{:x<4}", 5);
    //exp
    //println!("{:e}", 1000);//error
    println!("T{:e}", I32(1000));
    println!("e{:2e}", 10000.0);
    println!("E{:2E}", 10000.0);
    println!("{:10}", 13);
    println!("{:010}", 13);
    println!("{:<10}", 13);
    println!("{:^10}", 13);
    //the argv[1] defines the first format length, argv[2] defines the second format length
    println!("Pi is roughly {2:.*}, {3:.*}",3,4, pi, 1.23456789);
    
    //4.print braces,escape { and } with another { and } 
    println!("{{ {email} }}", email="1");

}

