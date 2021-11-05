use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

// 处理客户端流信息
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    // 将流写入缓存
    stream.read(&mut buf).unwrap();
    // 将bytes转换为string
    let request = String::from_utf8_lossy(&buf[..]);
    let request_line = request.lines().next().unwrap();
    // 打印请求
    println!("Request: {}", request_line);
    let get = b"GET / HTTP/1.1\r\n";

    // 逻辑处理
    let (status_line, content) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello Rust!\n")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "NOT FOUND\n")
    };
    // 构造响应
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // 构造Tcp套接字，设置Tcp监听地址
    let listener = TcpListener::bind("0.0.0.0:8080");
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    match listener {
        Ok(lis) => {
            // 获取连接的迭代器
            for stream in lis.incoming() {
                // 获取请求流数据
                match stream {
                    Ok(stream) => {
                        // handle_client(stream);
                        let handle = thread::spawn(move || {
                            handle_client(stream);
                        });
                        thread_vec.push(handle);
                    }
                    Err(err) => {
                        println!("fail! {:?}", err)
                    }
                }
                
            }
            for handle in thread_vec {
                handle.join().unwrap();
            }
        
        }
        Err(err) => {
            println!("Error {}", err);
        }
    }
}
