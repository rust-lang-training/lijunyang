use rand::Rng;
use std::sync::{Arc, Mutex};

pub fn test() {
    const N_THREADS: usize = 30;
    const N_POINTS: usize = 1_000_000_0;

    let mut handles = vec![];
    let res = Arc::new(Mutex::new(0));
    for _ in 0..N_THREADS {
        let c = res.clone();

        let handle = std::thread::spawn(move || {
            let mut n = 0;
            for _ in 0..N_POINTS {
                let x = rand::thread_rng().gen::<f64>();
                let y = rand::thread_rng().gen::<f64>();
                if x * x + y * y <= 1.0 {
                    n += 1;
                }
            }
            *c.lock().unwrap() += n;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *res.lock().unwrap();

    let pi = 4.0 * (result as f64) / (N_POINTS * N_THREADS) as f64;
    println!("pi: {}", pi);
}
