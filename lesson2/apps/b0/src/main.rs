#![no_std]
#![no_main]

use drv0 as _;
use drv1 as _;

use drv_common::CallEntry;

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B0\n");
    verify();
}

fn traverse_drivers() {
    extern "C" {
        fn initcalls_start() -> usize;
        fn initcalls_end() -> usize;
    }

    let mut range_start = unsafe { initcalls_start() };
    let range_end = unsafe { initcalls_end() };
    libos::println!("init calls range: 0x{:X} ~ 0x{:X}\n", range_start, range_end);

    while range_start < range_end {
        let entry = range_start as *const CallEntry;
        let drv = unsafe {((*entry).init_fn)()};
        libos::println!("Found driver '{}': compatible '{}'",
            drv.name, drv.compatible);

        range_start += core::mem::size_of::<CallEntry>();
    }
}

fn verify() {
    traverse_drivers();

    libos::println!("\nResult: Okay!");
}
