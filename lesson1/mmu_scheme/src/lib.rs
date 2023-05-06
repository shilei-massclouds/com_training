#![no_std]
#![feature(asm_const)]

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_ARRAY: [[u64; 512]; 3] = [[0; 512]; 3];

//
// Utility for mmu
//

const PAGE_SHIFT: usize = 12;
const PAGE_TABLE_ENTRIES: usize = 1 << (PAGE_SHIFT - 3);

const _PAGE_PRESENT : u64 = 1 << 0;     /* Valid */
const PAGE_TABLE: u64 = _PAGE_PRESENT;

pub const _PAGE_PFN_SHIFT: usize = 10;

#[cfg(feature = "sv39")]
const MMU_LEVELS: usize = 3;

#[cfg(feature = "sv48")]
const MMU_LEVELS: usize = 4;

macro_rules! LEVEL_SHIFT {
    ($level: expr) => {
        ((MMU_LEVELS - ($level)) * (PAGE_SHIFT - 3) + 3)
    }
}

fn vaddr_to_index(addr: usize, level: usize) -> usize {
    (addr >> LEVEL_SHIFT!(level)) & (PAGE_TABLE_ENTRIES - 1)
}

#[cfg(feature = "sv39")]
pub unsafe fn pre_mmu() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    let index = vaddr_to_index(0x8000_0000, 0);
    BOOT_PT_ARRAY[0][index] = (0x80000 << 10) | 0xef;

    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    let index = vaddr_to_index(0xffff_ffc0_8000_0000, 0);
    BOOT_PT_ARRAY[0][index] = (0x80000 << 10) | 0xef;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    let index = vaddr_to_index(0xffff_ffff_c000_0000, 0);
    BOOT_PT_ARRAY[0][index] = (0x80000 << 10) | 0xef;
}

fn make_node(paddr: u64, prot: u64) -> u64 {
    let pfn = paddr >> PAGE_SHIFT;
    return (pfn << _PAGE_PFN_SHIFT) | prot;
}

#[cfg(feature = "sv48")]
pub unsafe fn pre_mmu() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    let index = vaddr_to_index(0x8000_0000, 0);
    BOOT_PT_ARRAY[0][index] =
        make_node(BOOT_PT_ARRAY[1].as_ptr() as u64, PAGE_TABLE);
    let index = vaddr_to_index(0x8000_0000, 1);
    BOOT_PT_ARRAY[1][index] = (0x80000 << 10) | 0xef;

    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    let index = vaddr_to_index(0xffff_ffc0_8000_0000, 0);
    BOOT_PT_ARRAY[0][index] =
        make_node(BOOT_PT_ARRAY[2].as_ptr() as u64, PAGE_TABLE);
    let index = vaddr_to_index(0xffff_ffc0_8000_0000, 1);
    BOOT_PT_ARRAY[2][index] = (0x80000 << 10) | 0xef;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    let index = vaddr_to_index(0xffff_ffff_c000_0000, 0);
    BOOT_PT_ARRAY[0][index] =
        make_node(BOOT_PT_ARRAY[2].as_ptr() as u64, PAGE_TABLE);
    let index = vaddr_to_index(0xffff_ffff_c000_0000, 1);
    BOOT_PT_ARRAY[2][index] = (0x80000 << 10) | 0xef;
}

pub unsafe fn enable_mmu() {
    use riscv::register::satp;

    let mode = if cfg!(feature = "sv39") {
        satp::Mode::Sv39
    } else if cfg!(feature = "sv48") {
        satp::Mode::Sv48
    } else {
        panic!("bad feature. Only support sv39 or sv48!");
    };

    let page_table_root = BOOT_PT_ARRAY[0].as_ptr() as usize;
    satp::set(mode, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

pub unsafe fn post_mmu() {
    const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}
