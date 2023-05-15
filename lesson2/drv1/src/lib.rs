#![no_std]

pub struct Driver<'a> {
    name: &'a str,
    compatible: &'a str,
}

impl Driver<'_> {
    fn info<'a>(name: &'a str, compatible: &'a str) -> Driver<'a> {
        Driver {
            name,
            compatible,
        }
    }
}

type InitFn = fn() -> Driver<'static>;

pub struct CallEntry {
    init_fn: InitFn,
}

#[used]
#[link_section = ".init_calls"]
pub static DRV1_ENTRY: CallEntry = CallEntry {
    init_fn: drv1_init_fn,
};

fn drv1_init_fn() -> Driver<'static> {
    Driver::info("uart", "ns16550a")
}