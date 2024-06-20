use std::io::{Read, Write};
fn main() {
    let server_address = "127.0.0.1:8080";
    // 连接指定的服务器地址， 返回一个ke读写的 TcpStream
    let mut stream = std::net::TcpStream::connect(server_address).expect("Could not connect");
    let msg = b"hello CSDN from client";
    // 将所有数据完整的发送出去
    stream.write_all(msg).expect("Could not write to server");

    let mut response = Vec::new();
    // 读取数据知道流关闭或发生错误 将读取到的数据追加到 response变量中
    stream
        .read_to_end(&mut response)
        .expect("Could not read response");
    // 将接收到的数据转换为 utf-8 字符串
    println!("{}", String::from_utf8_lossy(&response));
}
