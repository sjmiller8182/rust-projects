
use chrono::{DateTime, Utc};

use crate::linux_parser;
use crate::process::Process;
use crate::format;
use crate::cpu::Cpu;
use crate::memory::MemInfo;

#[derive(Debug)]
pub struct System {
    os: String,
    kernel: String,
    cpu: Cpu,
    memory: MemInfo,
    processes: Vec<Process>,
    total_processes: u32,
    running_processes: u32,
    uptime: f64,
}

fn get_time() -> String {
    let time = Utc::now();
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn header() -> String {
    format!(
        "System Information{}{}",
        " ".repeat(40),
        get_time(),
    )
}

impl System {
    pub fn new() -> System {
        let os = linux_parser::get_operating_system();
        let kernel = linux_parser::get_kernel();
        
        let cpu = Cpu::new();
        
        let memory = MemInfo::new();
        
        let uptime = linux_parser::get_uptime();

        let total_processes = linux_parser::get_processes(linux_parser::ProcessStates::Total);
        let running_processes = linux_parser::get_processes(linux_parser::ProcessStates::Running);

        let mut processes: Vec<Process> = Vec::new();
        let pids = linux_parser::get_pids();
        for pid in pids {
            processes.push(Process::new(pid, cpu.total_jif(), uptime));
        }
        processes.sort_by(|a, b| a.cpu_utilization().partial_cmp(&b.cpu_utilization()).unwrap());
        processes.reverse();

        System {
            os,
            kernel,
            cpu,
            memory,
            processes,
            total_processes,
            running_processes,
            uptime,
        }
    }


    pub fn refresh(&mut self) {
        self.cpu.refresh();

        self.memory.refresh();

        let uptime = linux_parser::get_uptime();

        let mut processes: Vec<Process> = Vec::new();
        let pids = linux_parser::get_pids();
        for pid in pids { processes.push(Process::new(pid, self.cpu.total_jif(), uptime)); }
        self.processes = processes;

        self.total_processes = linux_parser::get_processes(linux_parser::ProcessStates::Total);
        self.running_processes = linux_parser::get_processes(linux_parser::ProcessStates::Running);

        self.uptime = uptime;
    }

    pub fn print(&self, process_limit: u32) {
        println!("{}", header());
        println!("OS: {} Kernel: {}", self.os, self.kernel);
        println!("Uptime: {}", format::format_seconds(self.uptime as u64));
        println!("CPU");
        println!("{}", self.cpu);
        println!("Memory");
        println!("{}", self.memory);
        println!("Processes");
        println!("- Total: {}, Running: {}", self.total_processes, self.running_processes);
        println!("");
        println!("{}", Process::head_str());
        let mut process_cnt = 0;
        for process in &self.processes {
            //let process = process.to_string();
            println!("{}", process);
            process_cnt += 1;
            if process_cnt >= process_limit {
                break;
            }
        }
    }
}