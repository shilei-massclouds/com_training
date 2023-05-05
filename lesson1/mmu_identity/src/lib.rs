#![no_std]

use riscv::register::satp;

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV39: [u64; 512] = [0; 512];

pub unsafe fn pre_mmu() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x102] = (0x80000 << 10) | 0xef;
}

pub unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_SV39.as_ptr() as usize;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

pub unsafe fn post_mmu() {
}