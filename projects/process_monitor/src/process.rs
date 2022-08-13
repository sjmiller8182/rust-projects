use std::fmt;

use crate::linux_parser;
use crate::file_utils;
use crate::format;

const RAM_LABEL_FILTER: &str = "VmData";
const UID_LABEL_FILTER: &str = "Uid";

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Process {
    pid: u32,
    command: String,
    ram: u32,
    uid: String,
    user: String,
    acive_jiffies: u32,
    total_jiffies: u64,
    uptime: u64,
}

impl Process {

    fn format_pid_path(pid: u32, file_name: &str) -> String {
        format!("{}{}/{}", linux_parser::PROC_DIR, pid,file_name)
    }

    fn get_ram(pid: u32) -> u32 {
        let file_path = Process::format_pid_path(pid, linux_parser::STATUS_FILENAME);
        let status = file_utils::read_as_hashmap(&file_path, ":");
        let status = status.unwrap();
        let ram = status.get(RAM_LABEL_FILTER);
        let ram = ram.unwrap();
        let ram = ram.split_whitespace().next();
        let ram = ram.unwrap();
        let ram: u32 = match ram.parse() {
                Ok(num) => num,
                Err(_) => panic!("Failed convert {} to u32", ram),
        };
        ram
    }

    fn get_uid(pid: u32) -> String {
        let file_path = Process::format_pid_path(pid, linux_parser::STATUS_FILENAME);
        let status = file_utils::read_as_hashmap(&file_path, ":");
        let status = status.unwrap();
        let uid = status.get(UID_LABEL_FILTER);
        let uid = uid.unwrap();
        let uid:Vec<&str> = uid.split_whitespace().collect();
        let uid = uid.get(0);
        let uid = uid.unwrap();
        uid.to_string()

    }

    fn get_command(pid: u32) -> String {
        let file_path = Process::format_pid_path(pid, linux_parser::CMDLINE_FILENAME);
        let cmd = file_utils::read_file_to_string(&file_path).unwrap();
        // remove the trailing \u{0}
        let mut chars = cmd.chars();
        chars.next_back();
        let cmd: String = chars.collect();
        cmd
    }

    // https://stackoverflow.com/questions/16726779/how-do-i-get-the-total-cpu-usage-of-an-application-from-proc-pid-stat
    fn get_active_jiffies(pid: u32) -> u32 {
        let file_path = Process::format_pid_path(pid, linux_parser::STAT_FILENAME);       
        let stat = file_utils::read_file_to_string(&file_path).unwrap();
        let stat: Vec<&str> = stat.trim().split_whitespace().collect();

        let mut jiffies: u32 = 0;
        for i in 13..17 {
            let jiffie_cnt = stat[i].parse::<u32>().unwrap();
            jiffies += jiffie_cnt;
        }
        jiffies
    }

    fn get_user(pid_uid: &str) -> String {
        let lines = file_utils::iter_lines(linux_parser::PASSWD_PATH);
        let lines = match lines {
            Ok(l) => l,
            Err(_) => panic!("Failed to read file {}", linux_parser::PASSWD_PATH),
        };
        for line in lines {
            let line = line.unwrap();
            let line: Vec<&str> = line.trim().split(":").collect();
            let user = line.get(0).unwrap();
            let uid = line.get(2).unwrap();
            if pid_uid == *uid {
                return user.to_string();
            }
        }
        String::from("")
    }

    fn get_uptime(pid: u32, system_uptime: f64) -> u64 {
        let file_path = Process::format_pid_path(pid, linux_parser::STAT_FILENAME);
        let stat = file_utils::read_file_to_string(&file_path).unwrap();
        let stat: Vec<&str> = stat.trim().split_whitespace().collect();

        let start_time = stat.get(21).unwrap();
        let clk_per_sec = linux_parser::get_sc_clk_tck() as f64;
        let start_time = start_time.parse::<u32>().unwrap() as f64 / clk_per_sec;
        (system_uptime - start_time) as u64
    }

    pub fn new(pid: u32, total_jiffies: u64, system_uptime: f64) -> Process {
        let ram = Process::get_ram(pid);
        let command = Process::get_command(pid);
        let uid = Process::get_uid(pid);
        let user = Process::get_user(&uid);
        let acive_jiffies = Process::get_active_jiffies(pid);
        let uptime = Process::get_uptime(pid, system_uptime);
        // get uid
        // get user
        // get uptime
        Process { 
            pid, 
            command,
            ram, 
            uid, 
            user,
            acive_jiffies,
            total_jiffies,
            uptime,
        }
    }

    pub fn pid(&self) -> u32 { self.pid }

    pub fn ram(&self) -> u32 { self.ram }

    pub fn user(&self) -> String { self.user.clone() }

    pub fn uid(&self) -> String { self.user.clone() }

    pub fn cpu_utilization(&self) -> f64 {      
        let system_uptime = linux_parser::get_uptime();

        let file_path = Process::format_pid_path(self.pid, linux_parser::STAT_FILENAME);
        let stat = file_utils::read_file_to_string(&file_path).unwrap();
        let stat: Vec<&str> = stat.trim().split_whitespace().collect();

        let start_time = stat.get(21).unwrap();
        let clk_per_sec = linux_parser::get_sc_clk_tck() as f64;
        let start_time = start_time.parse::<u32>().unwrap() as f64 / clk_per_sec;
        let seconds = system_uptime - start_time;

        let total_time = self.acive_jiffies as f64 / clk_per_sec;

        100.0 * total_time / seconds
    }

    pub fn command(&self) -> String {
        if self.command.len() > 40 {
            // cut down long commands to 40 characters
            let trunc_cmd: String = self.command.chars().take(37).collect();
            return format!("{}...", trunc_cmd)
        }
        self.command.clone()
    }

    pub fn head_str() -> String {
        format!(
            "{:<8} {:<8} {:<8} {:<10} {:>10} {:<40}",
            "PID",
            "USER",
            "CPU[%]",
            "RAM[KB]",
            "UPTIME",
            "COMMAND",
        )
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{:<8} {:<8} {:<8.2} {:<10} {:>10} {:<40}",
            self.pid(), 
            self.user(), 
            self.cpu_utilization(), 
            self.ram(), 
            format::format_seconds(self.uptime), 
            self.command()
        )
    }
}