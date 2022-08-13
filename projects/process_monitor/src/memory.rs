use std::fmt;
use crate::format;
use crate::linux_parser;

#[derive(Debug, Clone, Copy)]
pub enum MemScale {
    AsKiloBytes = 1,
    AsMegaBytes = 1000,
    AsGibaBytes = 1000000,
}

impl MemScale {

    fn unit(&self) -> String {
        match *self {
            MemScale::AsKiloBytes => String::from("KB"),
            MemScale::AsMegaBytes => String::from("MB"),
            MemScale::AsGibaBytes => String::from("GB"),
        }
    }
}

#[derive(Debug)]
pub struct MemInfo {
    total_mem: u64,
    free_mem: u64,
    scaling: MemScale,
}

impl fmt::Display for MemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "- Total: {}, Free: {}  [{}]\n- Utilization: {}", 
            self.total_mem,
            self.free_mem,
            self.scaling.unit(),
            format::bar(50, self.utilization())
        )
    }
}

impl MemInfo {
    pub fn new() -> MemInfo {
        let scaling = MemScale::AsKiloBytes;
        let (free_mem, total_mem) = linux_parser::get_mem_utilization();
        let free_mem = free_mem / scaling as u64;
        let total_mem = total_mem / scaling as u64;

        MemInfo { total_mem, free_mem, scaling}
    }

    pub fn utilization(&self) -> f64 {
        1.0 - (self.free_mem as f64 / self.total_mem as f64)
    }

    pub fn refresh(&mut self) {
        let (free_mem, _) = linux_parser::get_mem_utilization();
        let free_mem = free_mem / self.scaling as u64;

        self.free_mem = free_mem
    }
}
