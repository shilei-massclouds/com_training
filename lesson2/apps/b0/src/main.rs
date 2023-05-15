#![no_std]
#![no_main]

pub struct Driver<'a> {
    name: &'a str,
    compatible: &'a str,
}

impl Driver<'_> {
    fn new<'a>(name: &'a str, compatible: &'a str) -> Driver<'a> {
        libos::println!("Register driver {}", name);
        Driver {
            name,
            compatible,
        }
    }
}

type InitFn = fn() -> Driver<'static>;

pub struct CallEntry {
    id: usize,
    init_fn: InitFn,
}

#[used]
#[link_section = ".init_calls"]
pub static drv0_entry: CallEntry = CallEntry {
    id: 0x1234,
    init_fn: drv0_init_fn,
};

fn drv0_init_fn() -> Driver<'static> {
    Driver::new("drv0", "type0")
}

#[used]
#[link_section = ".init_calls"]
pub static drv1_entry: CallEntry = CallEntry {
    id: 0x4321,
    init_fn: drv1_init_fn,
};

fn drv1_init_fn() -> Driver<'static> {
    Driver::new("drv1", "type1")
}

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
    libos::println!("init calls range: 0x{:X} ~ 0x{:X}", range_start, range_end);

    while range_start < range_end {
        let entry = range_start as *const CallEntry;
        libos::println!("Init call id {:x}", unsafe {(*entry).id});
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
