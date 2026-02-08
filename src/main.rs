fn main() {
    let x: i8 = 127;
    // This overflows
    // println!("{}",x + 1);
    // Rust forces you to declare your intent....Therefore
    let y = x.wrapping_add(1);
    println!("Wrapping Add 1 to x is = {}", y);
}
