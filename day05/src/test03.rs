use async_std::{fs::File, io, io::prelude::*, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut content = String::new();

    file.read_to_string(&mut content).await?;
    Ok(content)
}

pub async fn test03_main() {
    // let content = read_file("./src/test02.rs").await.unwrap();
    // println!("{}", content);

    let reader_task = task::spawn(async {
        let result = read_file("./src/test02.rs").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e),
        }
    });
    println!("Started task!");
    task::block_on(reader_task);
    println!("Stopped task!");
}
// Task 是 async_std 的核心抽象之一
// task 有许多理想的属性
// 所有分配一次完成
// 所有任务都有backchannel， 通过 JoinHandle 将结果和错误回传到生成任务
// 带有用于调试的元数据
// 支持本地任务存储
