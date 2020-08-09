use std::net::{SocketAddr, TcpStream, TcpListener};
// use std::net::{SocketAddrV4, Ipv4Addr}
use std::io::prelude::*; //引入来获取读写流所需的特定 trait

fn main() {
    let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 7878)),//取本地地址和端口，以下是不同方式取址
    //SocketAddr::from(([127, 0, 0, 1], 8000)),
    //let loopback = Ipv4Addr::new(127, 0, 0, 1); let socket = SocketAddrV4::new(loopback, 0); TcpListener::bind(socket)?;
    ];

    let listener = TcpListener::bind(&addrs[0]).unwrap(); //绑定到一个端口
    for stream in listener.incoming(){
        match stream {// 错误处理（模式匹配）
            Err(e) => println!("Eorr : {}", e),
            Ok(stream) => {handle_connection(stream)}
        }
        //let stream = stream.unwrap();
       // handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    //创建了一个 512 字节的缓冲区，它足以存放基本请求的数据
    let mut buffer = [0;512];
    //将缓冲区传递给 stream.read ，它会从 TcpStream 中读取字节并放入缓冲区中
    stream.read(&mut buffer).unwrap();
    //将缓冲区中的字节转换为字符串并打印出来。String::from_utf8_lossy 函数获取一个 &[u8] 并产生一个 String
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); 
}