use num_cpus;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time;
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn single_thread(end: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();

    for n in 0..=end {
        if is_prime(n) {
            primes.push(n);
        }
    }
    primes
}

fn multi_thread_shared(end: u32) -> Arc<Mutex<Vec<u32>>> {
    let primes: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    let cpu_len = num_cpus::get() as u32 - 1u32;
    let step = end / cpu_len as u32;
    let mut handles = vec![];
    for i in 0..cpu_len {
        let temp = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            let mut container: Vec<u32> = vec![];
            let start = i * step;
            let end = if i == cpu_len - 1u32 {
                end
            } else {
                start + step
            };

            if i == cpu_len - 1u32 {
                for n in start..=end {
                    if is_prime(n) {
                        container.push(n);
                    }
                }
            } else {
                for n in start..end {
                    if is_prime(n) {
                        container.push(n);
                    }
                }
            }
            temp.lock().unwrap().extend(container);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    primes
}

fn multi_thread_mpsc(end: u32) -> Vec<u32> {
    let (sender, receiver) = mpsc::channel();

    let cpu_len = num_cpus::get() as u32 - 1u32;
    let step = end / cpu_len as u32;
    for i in 0..cpu_len {
        let _sender = sender.clone();
        thread::spawn(move || {
            let start = i * step;
            let end = if i == cpu_len - 1u32 {
                end
            } else {
                start + step
            };
            let mut container: Vec<u32> = vec![];
            if i == cpu_len - 1u32 {
                for n in start..=end {
                    if is_prime(n) {
                        container.push(n);
                    }
                }
            } else {
                for n in start..end {
                    if is_prime(n) {
                        container.push(n);
                    }
                }
            }
            _sender.send(container).unwrap();
        });
    }

    let mut primes = vec![];
    drop(sender);
    for res in receiver {
        primes.extend(res);
    }

    primes
}
pub fn test() {
    // assert_eq!(is_prime(2), true);
    // assert_eq!(is_prime(937), true);
    // assert_eq!(is_prime(971), true);
    // assert_eq!(is_prime(115), false);
    const N: u32 = 1_000_000;

    let now = time::Instant::now();
    let single_thread_primes = single_thread(N);
    let elapsed = now.elapsed();
    println!("single_thread_primes time: {:?}", elapsed);
    // println!("{}", single_thread_primes.len());

    let now = time::Instant::now();
    let multi_thread_primes = multi_thread_shared(N);
    let elapsed = now.elapsed();
    println!("multi_thread_primes time: {:?}", elapsed);
    // println!("{}", multi_thread_primes.lock().unwrap().len());

    let now = time::Instant::now();
    let multi_thread_mpsc_primes = multi_thread_mpsc(N);
    let elapsed = now.elapsed();
    println!("multi_thread_mpsc_primes time: {:?}", elapsed);
    // println!("{}", multi_thread_mpsc_primes.len());

    let res_len = multi_thread_primes.lock().unwrap().len();
    assert_eq!(single_thread_primes.len(), res_len);
    assert_eq!(res_len, multi_thread_mpsc_primes.len())
}

// 找出1百万以内的质数
// 多线程 + 线程间共享可变数据的方式
//多线程 + mpsc模式
// 以上两种模式任选其一或两者都做
