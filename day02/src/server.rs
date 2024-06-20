use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    // 创建一个TcpListener, 绑定到本地127.0.0.1:8080
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
    println!("server listening on 127.0.0.1:8080");
    // 等待客户端的连接
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            handle_client(stream);
        });
    }
    return Ok(());
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0u8; 1024];
    // 读取客户端的发送的数据
    match stream.read(&mut buffer) {
        Ok(size) => {
            let msg = String::from_utf8_lossy(&buffer[..size]);
            println!("received from client: {}", msg);

            let response = "hello SCDN form server";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("read failed from client: {}", e);
        }
    }
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
