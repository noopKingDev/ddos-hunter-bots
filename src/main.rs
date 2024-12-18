mod services;
mod exploits;
mod utils;

use std::{fs, thread::{self, sleep}, time::Duration};

use services::hunter_shell::hunter_shells;
use utils::{check_ports::check_ports, utils::generation_ips};

static THREADS :u32 = 500;

fn main() {
    let input = String::from("--rand-ip");

    match input {
      i if i.contains("--rand-ip") => {
        rand_ip_check();
      },
      i if i.contains("--list") => {
        check_list_ip("./ips.txt");
      }
      _ => {}
    };
    
}

fn check_list_ip(path: &str) {

  let ips_file = fs::read_to_string(&path).unwrap();
  let lines_files: Vec<String> = ips_file.lines().map(|line| line.to_string()).collect();
  
  let lines_count_file = lines_files.len() as u32;

  if lines_count_file < 1 {
    println!("file is empyt !");
    return
  }


    let separator  = if  lines_count_file  > THREADS {
      lines_count_file  / THREADS
    } else {
      1
    };
    
    
    for i in 0..separator {
      
      let current_range = lines_count_file / separator * i;
      let next_range = lines_count_file / separator * (i + 1);

      let mut threads = Vec::new();


      for current_line in current_range..next_range {
        

          let l_file = lines_files.clone();

          let thread = thread::spawn(move || {
            
              let ports = [23,21,2121,22];
              let current_index = &l_file[current_line as usize];

              let ip = String::from(current_index);
              let open_ports = check_ports(&ip, &ports).unwrap();
              
              for host in open_ports {
                hunter_shells(host.ip, host.port);
          }
        });

        threads.push(thread)
      };
      for thread in threads {
        thread.join().unwrap();
      }
}
}
fn rand_ip_check() {
  let test_quantity: u32 = 100000;
    
    let separator  = if  test_quantity > THREADS {
      test_quantity / THREADS
    } else {
      1
    };
    
    
    for i in 0..separator {
      let current_range = test_quantity / separator * i;
      let next_range = test_quantity / separator * (i + 1);
      
      let mut threads = Vec::new();

      
      for _ in current_range..next_range {
        
          let thread = thread::spawn(|| {
            
              let ports = [23,21,2121,22];
          
              let rand_ip = generation_ips();
              // println!("checking ip : {rand_ip}");
              let open_ports = check_ports(&rand_ip, &ports).unwrap();
              
              for host in open_ports {
                hunter_shells(host.ip, host.port);
                // println!("{}:{}", host.ip, host.port);
          }
        });

        threads.push(thread)
      };
      for thread in threads {
        thread.join().unwrap();
      }
}
}