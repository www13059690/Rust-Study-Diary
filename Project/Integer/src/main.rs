fn main() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b); // 19

    let b = a.checked_add(20);
    println!("{:?}", b);

    let (b, c) = a.overflowing_add(20);
    println!("{}, {}", b, c);

    let b = a.saturating_add(20);
    println!("{}", b);
}
