// Import chrono::Local for date and time functionality
use chrono::Local;
use std::{fs::OpenOptions, io::Write};

// Allow dead code, unused variables, and private interfaces
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(private_interfaces)]

// Import sysinfo::System for system information
use sysinfo::System;

// Define a struct to hold system information
#[derive(Debug)]
#[allow(dead_code)]
pub struct SystemInfo {
    process_name: String,
    global_cpu_usage: f32,
    total_memory: u64,
    free_memory: u64,
    process_status: String,
    virtual_memory: u64,
    time: String,
}

// Implement SystemInfo struct
impl SystemInfo {
    // Search for process information by name
    pub fn search(process: &String) -> SystemInfo {
        // Get process name
        let processname = process;
        let now = Local::now();
        let custom_format = now.format("%A, %B %d, %Y at %I:%M:%S %p").to_string();
        // Print search message
        //println!("Searching for PID: {}", processname);
        // Create a new system instance
        let mut sys = System::new_all();
        // Refresh system information
        sys.refresh_all();
        // Iterate over all processes
        for (pid, process) in sys.processes() {
            // Get process name
            let process_name = process.name().to_string_lossy().to_string();
            // Get process ID
            let _pid = pid.as_u32();
            // Get current time
            
            // Format time string
            

            // Check if process name matches
            if &process_name == processname {
                // Create a new SystemInfo instance
                let info = SystemInfo {
                    process_name,
                    global_cpu_usage: sys.global_cpu_usage(),
                    total_memory: sys.total_memory(),
                    free_memory: sys.free_memory(),
                    process_status: process.status().to_string(),
                    virtual_memory: process.virtual_memory(),
                    time: custom_format,
                };
                // Return SystemInfo instance
                return info;
            }
        }
        // Print no matching process found message
        //println!("No matching process found.");
        // Return None
        return SystemInfo {
            process_name: (processname).to_string(),
            global_cpu_usage: sys.global_cpu_usage(),
            total_memory: sys.total_memory(),
            free_memory: sys.free_memory(),
            process_status: String::from("Not Running"),
            virtual_memory: 0,
            time: custom_format
        }
        }
           
    
    fn tostring(&self) -> String {
        format!("Process Name: {}\nGlobal CPU Usage: {:.2}%\nTotal Memory: {}\nFree Memory: {}\nProcess Status: {}\nVirtual Memory: {}\nTime: {}\n",
                self.process_name, self.global_cpu_usage, self.total_memory, self.free_memory, self.process_status, self.virtual_memory, self.time)
    }

    pub fn write_file(data:&SystemInfo)->String{
        let mut file = OpenOptions::new().append(true).create(true).open("Log.txt").expect("failed");
        file.write_all(data.tostring().as_bytes()).expect("fail");
        file.write_all("\n".as_bytes()).expect("failed");
        let s =String::from("Done");
        return s;
        }
}


