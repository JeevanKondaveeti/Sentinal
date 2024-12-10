#[allow(non_snake_case)]
mod Search;
use std::io;
use std::thread;
use std::time::Duration;
use sysinfo::System;
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let processes = sys.processes();
    for (pid,process) in processes{
        println!("pid: {:?} , process: {:?}", pid,process.name().to_string_lossy().to_string());
    }
    println!("Enter pid of the appliction:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let input_pid:u32 = match input.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Enter valid Pid");
            return;
        }
    };
    let mut process_name =String::new();
    for (pid,process) in processes{
        if input_pid == pid.as_u32(){
            process_name = process.name().to_string_lossy().to_string();
        }
    }
    loop{
       
        let p = Search::SystemInfo::search(&process_name);
        //println!("{:?}",p);
        let data_write = p;
        
        let store = Search::SystemInfo::write_file(&data_write);
        println!("file write on 'Log.txt' :{}",store);
        thread::sleep(Duration::from_secs(60));
    }
}
