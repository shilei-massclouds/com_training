#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(naked_functions)]

mod lang_items;
mod trap;
pub mod stdio;

use riscv::register::satp;
use riscv::register::stvec;
use riscv::register::sstatus;

const TASK_STACK_SIZE: usize = 0x40000;
const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV39: [u64; 512] = [0; 512];

unsafe fn pre_mmu() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x102] = (0x80000 << 10) | 0xef;
}

unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_SV39.as_ptr() as usize;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

unsafe fn post_mmu() {
}

pub(crate) fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// Writes Supervisor Trap Vector Base Address Register (`stvec`).
#[inline]
pub fn set_trap_vector_base(stvec: usize) {
    unsafe { stvec::write(stvec, stvec::TrapMode::Direct) }
}

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    extern "C" {
        fn main(cpu_id: usize, dtb: usize);
    }

    clear_bss();
    set_trap_vector_base(trap_vector_base as usize);
    main(cpu_id, dtb);
    terminate();
}

/// The earliest entry point for the primary CPU.
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start() -> ! {
    // PC = 0x8020_0000
    // a0 = hartid
    // a1 = dtb_ptr
    core::arch::asm!("
        // 1. save hartid & dtb_ptr
        mv      s0, a0
        mv      s1, a1

        // 2. setup boot stack
        la      sp, {boot_stack}
        li      t0, {boot_stack_size}
        add     sp, sp, t0

        // 3. setup boot page table
        call    {pre_mmu}
        // 4. enable paging
        call    {enable_mmu}
        // 5. post process paging
        call    {post_mmu}

        li      s2, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, s2

        // 6. restore hartid & dtb_ptr
        mv      a0, s0
        mv      a1, s1

        // 7. enter rust world
        la      a2, {rust_entry}
        add     a2, a2, s2
        jalr    a2                      // call rust_entry(hartid, dtb)

        // 8. Unreachable!!!
        j       .",
        boot_stack = sym BOOT_STACK,
        boot_stack_size = const TASK_STACK_SIZE,
        pre_mmu = sym pre_mmu,
        enable_mmu = sym enable_mmu,
        post_mmu = sym post_mmu,
        phys_virt_offset = const PHYS_VIRT_OFFSET,
        rust_entry = sym rust_entry,
        options(noreturn),
    )
}

extern "C" {
    fn sbss();
    fn ebss();
    fn trap_vector_base();
}

/// Makes the current CPU to ignore interrupts.
#[inline]
pub fn disable_irqs() {
    unsafe { sstatus::clear_sie() }
}

#[inline]
pub fn halt() {
    disable_irqs();
    unsafe { riscv::asm::wfi() } // should never return
}

/// Shutdown the whole system, including all CPUs.
pub fn terminate() -> ! {
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    loop {
        halt();
    }
}

pub fn init() {
}
