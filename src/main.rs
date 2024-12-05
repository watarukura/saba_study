#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use alloc::rc::Rc;
use core::cell::RefCell;
use noli::*;
use saba_core::browser::Browser;
use saba_core::http::HttpResponse;
use ui_wasabi::app::WasabiUI;

fn main() -> u64 {
    let browser = Browser::new();

    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));
    match ui.borrow_mut().start() {
        Ok(_) => {}
        Err(e) => {
            println!("browser fails to start {:?}", e);
            return 1;
        }
    }

    0
}

entry_point!(main);
