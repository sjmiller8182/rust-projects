use std::fmt;

use crate::linux_parser;
use crate::format;

#[derive(Debug)]
pub struct Cpu {
    n_cores: u32,
    prev_jif: (u64, u64),
    current_jif: (u64, u64),
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "- Cores Cnt: {}\n- Utilization: {}", 
            self.get_cpu_count(),
            format::bar(50, self.utilization())
        )
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        let n_cores = linux_parser::get_cpu_cnt();
        let current_jif = linux_parser::get_cpu_utilization();

        Cpu {
            n_cores,
            prev_jif: (0, 0),
            current_jif
        }
    }

    pub fn utilization(&self) -> f64 {
        let change_active = self.current_jif.0 - self.prev_jif.0;
        let change_total = self.current_jif.1 - self.prev_jif.1;
        // don't divide by zero
        if change_total == 0 {
            return 0.0;
        }
        change_active as f64 / change_total as f64
    }

    pub fn refresh(&mut self) {
        self.prev_jif = self.current_jif;
        self.current_jif = linux_parser::get_cpu_utilization();
    }

    pub fn get_cpu_count(&self) -> u32 {
        self.n_cores
    }

    pub fn total_jif(&self) -> u64 {
        self.current_jif.0 + self.current_jif.1
    }
}
