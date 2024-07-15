use std::{sync::mpsc, time::Duration};

pub fn test() {
    let (sender, receiver) = mpsc::channel::<String>();

    let _log_thread = std::thread::spawn(move || {
        while let Ok(msg) = receiver.recv() {
            println!("{:?}, {}", std::thread::current().id(), msg);
        }
    });

    let mut handles = vec![];

    for _ in 0..5 {
        let sender = sender.clone();
        let handle = std::thread::spawn(move || {
            let msg = format!("Hello from thread {:?}", std::thread::current().id());
            sender.send(msg).unwrap();
            std::thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

// 如何实现多个生产者 + 多个消费者
// Mutex
