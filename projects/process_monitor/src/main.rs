mod linux_parser;
mod process;
mod file_utils;
mod system;
mod terminal;
mod format;
mod cpu;
mod memory;

use crate::system::System;

use std::{thread, time::Duration};

fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn main() {
    
    let mut sys = System::new();
    wait(100);
    
    loop {
        terminal::clear_screen();
        sys.refresh();
        //println!("{:#?}", sys);
        sys.print(10);

        wait(2000);
    }
}
