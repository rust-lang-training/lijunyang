use rand::{self, Rng};

fn main() {
    let n: u32 = rand::thread_rng().gen_range(1..100000);
    println!("{}", is_prime(n)); // 输出 1 到 100 之间的随机数
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            println!("{}不是质数", n);
            return false;
        }

        i += 1;
    }
    println!("{}是质数", n);
    true
}
