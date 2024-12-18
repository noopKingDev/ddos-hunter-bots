use std::{error::Error, net::{SocketAddr, TcpStream}, str::FromStr, thread::{self, JoinHandle}, time::Duration};


pub struct Host {
    pub ip: String,
    pub port : u32
}

// struct CheckPort {
//     thread: 
// }

pub fn check_ports(ip: &String, ports: &[u32]) -> Result<Vec<Host>, Box<dyn Error>> {
    
    let mut simultaneous_check = Vec::new();

    let ports_thread = ports.clone();
    
    for &port in ports_thread {
        
        let ip_clone = ip.clone();
        
        let thread= thread::spawn(move || {
            match try_connection_with_port(&ip_clone, port) {
                Ok(r) => {
                    return Some(r)
                },
                Err(_)   => {
                    None
                }
            }
        });

        simultaneous_check.push(thread);
    }

        let mut live_ports = Vec::new();

    for thread in simultaneous_check {
        let res = thread.join();
        match res {
            Ok(host) => {
             if let Some(h) = host {
                live_ports.push(h)
             }
            },  // Sucesso
            Err(_) => {}, // Erro ao fazer join
        
        };
        // live_ports.push(res)
    }
    // println!("{:?}",live_ports);
    Ok(live_ports)
}

fn try_connection_with_port(ip: &String, port: u32) -> Result<Host, Box<dyn Error>> {
    let host = format!("{}:{}",ip,port);
    let add_host = host.parse::<SocketAddr>()?;
    let timeout = Duration::from_secs(2);
    {
        TcpStream::connect_timeout(&add_host, timeout)?;
    }
    

    let host_live = Host {
        ip: ip.clone(),
        port : port.clone()
    };
    Ok(host_live)
}