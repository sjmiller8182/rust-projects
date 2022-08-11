
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

use nix::unistd;

use crate::file_utils;

// files pathes
pub const PROC_DIR: &str = "/proc/";
pub const VERSION_FILENAME: &str = "version";
pub const MEM_FILENAME: &str = "meminfo";
pub const UPTIME_FILENAME: &str = "uptime";
pub const STAT_FILENAME: &str = "stat";
pub const STATUS_FILENAME: &str = "status";
pub const CMDLINE_FILENAME: &str = "cmdline";
pub const PASSWD_PATH: &str = "/etc/passwd";
pub const OS_PATH: &str = "/etc/os-release";


pub enum ProcessStates {
    Total,
    Running,
    Blocked,
}
impl ProcessStates {
    fn label(&self) -> &str {
        match self {
            Self::Total => "processes",
            Self::Blocked => "procs_blocked",
            Self::Running => "procs_running",
        }
    }
}

pub fn get_sc_clk_tck() -> i64 {
    match unistd::sysconf(unistd::SysconfVar::CLK_TCK) {
        Ok(op) => op.expect("Failed to read CLK_TCK"),
        Err(_) => 1,
    }
}

pub fn read_lines<P>(filename: P) -> io::Result::<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn is_numeric(str: &str) -> bool {
    let is_numeric: Vec<bool> = str.chars().map(|c| c.is_numeric()).collect();
    !is_numeric.contains(&false)
}


pub fn get_operating_system() -> String {
    let os_key = "PRETTY_NAME";
    let os_release = file_utils::read_as_hashmap(OS_PATH, "=");
    let os_release = os_release.unwrap();
    let os_name = os_release.get(os_key);
    let os_name = os_name.unwrap();
    os_name.replace("\"", "")
}

pub fn get_kernel() -> String {
    let file_path = format!("{}{}", PROC_DIR, VERSION_FILENAME);
    let version_info = match fs::read_to_string(file_path) {
        Ok(str) => str,
        Err(_) => "Failed to read version file".to_string(),
    };
    let version_info = version_info.split(" ");
    let version_info: Vec<&str> = version_info.collect();
    version_info[2].to_string()
}

pub fn get_pids() -> Vec<u32> {
    let mut pids: Vec<u32> = vec![];
    let paths = fs::read_dir(PROC_DIR).unwrap();
    for path in paths{
        let pid = path.unwrap().file_name().to_str().unwrap().to_string();
        if is_numeric(&pid) {
            let pid = pid.parse::<u32>().unwrap();
            pids.push(pid);
        }
    }
    return pids
}

pub fn get_mem_utilization() -> (u64, u64) {
    let file_path = format!("{}{}", PROC_DIR, MEM_FILENAME);

    let lines = file_utils::read_n_lines(&file_path, 2);
    let lines = lines.unwrap();

    let mem_total:Vec<&str> = lines.get(0).unwrap().split_whitespace().collect();
    let mem_free:Vec<&str> = lines.get(1).unwrap().split_whitespace().collect();

    let mem_total = mem_total[1].parse::<u64>().unwrap();
    let mem_free = mem_free[1].parse::<u64>().unwrap();

    (mem_free, mem_total)

}

pub fn get_uptime() -> f64 {
    let file_path = format!("{}{}", PROC_DIR, UPTIME_FILENAME);
    let line = fs::read_to_string(file_path).expect("Unable to read uptime file");
    let uptime: Vec<&str> = line.split_whitespace().collect();

    uptime[0].parse::<f64>().unwrap()
}

pub fn get_cpu_cnt() -> u32 {
    let file_path = format!("{}{}", PROC_DIR, STAT_FILENAME);
    let lines = file_utils::iter_lines(&file_path);
    let lines = match lines {
        Ok(l) => l,
        Err(_) => panic!("Failed to read file {}", file_path),
    };

    let mut cpu_cnt = 0;
    for line in lines {
        let line = line.unwrap();
        if line.contains("cpu") {
            cpu_cnt += 1;
        }
    }
    match cpu_cnt {
        0 => 0,
        1 => 1,
        _ => cpu_cnt - 1,
    }
}

pub fn get_cpu_utilization() -> (u64, u64) {
    // see https://www.linuxhowtos.org/System/procstat.htm
    let mut cpu_utilization: Vec<u64> = vec![];

    let file_path = format!("{}{}", PROC_DIR, STAT_FILENAME);
    let ut_line = file_utils::read_n_lines(&file_path, 1);
    let ut_line = ut_line.unwrap();
    let ut_line = ut_line[0].to_string();
    let utilization = ut_line.split_whitespace();
    for str in utilization {
        if is_numeric(str) {
            let jiffy: u64 = match str.parse() {
                Ok(num) => num,
                Err(_) => panic!("Invalid jiffy string")
            };
            cpu_utilization.push(jiffy);
        }
    }
    // (return active jiffies, idle jiffies)
    (
        cpu_utilization[0] + cpu_utilization[1] + cpu_utilization[2],
        cpu_utilization[3] + cpu_utilization[4]
    )
}

/*
pub fn get_processes(process_type: ProcessStates) -> u32 {
    let file_path = format!("{}{}", PROC_DIR, STAT_FILENAME);
    let value = get_value(&process_type.label(), &file_path);
    let value: u32 = match value.parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed conver {} to u32", value),
    };
    value
}
*/

pub fn get_processes(process_type: ProcessStates) -> u32 {
    let file_path = format!("{}{}", PROC_DIR, STAT_FILENAME);
    let stat = file_utils::read_as_hashmap(&file_path, " ");
    let stat = stat.unwrap();
    let value = stat.get(process_type.label());
    let value = value.unwrap();
    match value.parse() {
        Ok(num) => num,
        Err(e) => panic!("Failed to convert {} to u32: {}", value, e),
    }

}

