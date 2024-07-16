fn slow_fn() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}
pub fn test_main() {
    let now = std::time::Instant::now();
    slow_fn();
    slow_fn();
    let a = now.elapsed();
    println!("{}", a.as_millis());
}
