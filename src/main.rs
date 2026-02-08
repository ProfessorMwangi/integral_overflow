fn main() {
    let x: i8 = 127;
    // This overflows
    // println!("{}",x + 1);
    // Rust forces you to declare your intent....Therefore it has this 4 Overflow API's

    // Cryptography and Hashing
    let y = x.wrapping_add(1);
    println!("Wrapping Add 1 to x ({1:08b}) is = {0}({0:b})", y,x);

    // Financial Systems
    let y1 = x.checked_add(1);
    println!("Checked Add 1 to x is {:?}",y1);

    // Signal Processing
    let (y2, overflowed)= x.overflowing_add(1);
    println!("Overflowed Add 1 to x is {}.Which returns ({0},{1})", y2, overflowed);

    // Low level/ diagnostics
    let y3 = x.saturating_add(1);
    println!("Saturating Add 1 to x is {}",y3)


}
