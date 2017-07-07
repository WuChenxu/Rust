use std::mem;

struct Scard_IO_Request {
    proto: u8,
    pciLength: u32
}

type Pair<'a> = (i32, &'a str);
fn main() {
let array: [i32; 3] = [1,2,3];
let vector: Vec<i32> = vec![1,2,3];
let slice: &[i32] = &vector[..];

    println!("size_of(isize)={}", mem::size_of::<isize>());
    println!("size_of(i8)={}", mem::size_of::<i8>());
    println!("size_of(i16)={}", mem::size_of::<i16>());
    println!("size_of(char)={}", mem::size_of::<char>());
    println!("size_of(&str)={}", mem::size_of::<&str>());
    println!("size_of(struct Scard_IO_Request)={}", mem::size_of::<Scard_IO_Request>());
    println!("size_of(tuple Pari)={}", mem::size_of::<Pair>());
    println!("size_of(array[i32,3])={}", mem::size_of::<[i32; 3]>());
    println!("size_of(&array)={}", mem::size_of::<&[i32; 3]>());
    println!("size_of(&[&i8])={}", mem::size_of::<&[&i8]>());
    println!("size_of(Vec<i32>)={}", mem::size_of::<Vec<i32>>());
    println!("size_of(fn(i32)->i32)={}", mem::size_of::<fn(i32)->i32>());
}
