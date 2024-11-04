fn main(){
    // Integers: 
    // Signed Integers: i8, i16, i32, i64, i128
    // Unsigned Integers: u8, u16, u32, u64, u128
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Example
    let e: i32 = 214748364;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // Boolean values: true, false
    let is_snowing: bool = true;
    println!("Is It snowing ? {}", is_snowing);

    // Character Type - char
    let letter: char = 'a';

    println!("First letter of the alphabet is: {}", letter);


}