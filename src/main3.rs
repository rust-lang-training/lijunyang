fn main() {
    // println!("Hello, world!");
    for i in 1..=50 {
        println!("i = {} {}==={}", i, fabnacci(i), fabnacci2(i as u64));
    }
}
fn fabnacci2(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    };
    return fabnacci2(n - 1).wrapping_add(fabnacci2(n - 2));
}
fn fabnacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    };
    let mut first = 1;
    let mut second = 1;
    let mut temp = 0;

    for _ in 3..=n {
        temp = (first as u32).saturating_add(second);
        first = second;
        second = temp;
    }
    second
}
