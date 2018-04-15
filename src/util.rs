use core;

pub unsafe fn irq_disable() {
    asm!("cpsid i");
}

pub unsafe fn irq_enable() {
    asm!("cpsie i");
}

pub unsafe fn write(addr: *mut u32, val: u32) {
    core::ptr::write_volatile(addr, val);
}

pub unsafe fn write_or(addr: *mut u32, val: u32) {
    let reg = core::ptr::read_volatile(addr);
    core::ptr::write_volatile(addr, reg | val);
}

pub unsafe fn write_modify(addr: *mut u32, val: u32, mask: u32) {
    let mut reg = core::ptr::read_volatile(addr);
    reg = reg & !mask;
    core::ptr::write_volatile(addr, reg | (val & mask));
}

pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("");
    }
}

#[allow(dead_code)]
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}
