use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use tiny_http::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for steam in listener.incoming() {
        match steam {
            Ok(steam) => {
                pool.execute(||{
                    handle_http(steam)
                });
            }
            Err(_e) => {
                println!("CONNECT ERR HTTP")
            }
        }
    }
}

fn handle_http(mut steam: TcpStream) {
    let reader = BufReader::new(&mut steam);
    let http_method = reader.lines().next().unwrap().unwrap();

    let (status_line,file_name) = if http_method == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK","index.html")
    }else {
        ("HTTP/1.1 404 ERROR","404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();
    let response = format!("{status_line} \r\nContent-Length: {length} \r\n\r\n {content}");
    //返回客户端数据
    steam.write_all(response.as_bytes()).unwrap()

}
