#![no_std]
#![no_main]

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: A0");
    libos::println!("Hello, ArceOS!");
}
