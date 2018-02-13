fn borrow(r: &u8) -> u8 {
    *r
}

fn increment(r: &mut u8) {
    *r = *r + 1;
}

fn helper(slot: Box<i32>) {
    println!(" the number is: {}", slot);
}


fn main() {
    let slot = Box::new(3);
    helper(slot);
   // helper(slot);
    let x = &10;
    let y = x;
    let mut a: u8 = 100;
    let b = a.clone();
    println!("{}   a{}", *x, a);
    println!("borrow(x): {}", borrow(x));
    increment(&mut a);
    println!("{}   a{}", *x, a);
    
    let mut t = 9;
    {
        let y = &t;
        //t = 11; //error here
    }
    t = 12;
    
    let mut ref1: &u8;
    {
        let val: &u8 = &10;
        //ref1 = &val;
    }//error: temp value drop here
   // println!("{}",  **ref1); //error: can not be de-reference

    let mut vec: Vec<i32> = Vec::new();
    println!("empty vec:{:?}, len={}", vec, vec.len()); 
    let mut vec = vec![0, 1, 2];
    vec.push(3);
    println!("{:?}", vec);
    vec.insert(2, 10);//intert 10 at positioin 2
    println!("{:?}", vec);
    let last = vec.pop();
    println!("{:?}", vec);
    for i in vec.iter() {
        println!("member: {}", i);
    }
    let element1 = vec.remove(1);
    println!("{:?}", vec);

    //let p = vec![1, 2, 3];
    //let q = inc(p);
    let mut p : Vec<u32>= vec![1, 2, 3];
    inc_mut(&mut p);
    for i in p.iter() {
        print!("{} ", i);
    } 
    
    println!("");

}


fn inc(vec: Vec<u32>) -> Vec<u32> {
    vec.into_iter().map(|x| x+1).collect()
}

fn inc_mut(vec: &mut Vec<u32>) {
    for i in vec.iter_mut() {
       *i += 1; 
    }
}
