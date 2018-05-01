use alloc::boxed::Box;
use sched;
use util;

pub fn start() {
    unsafe {
        util::irq_disable();

        sched::task_init();

        clock_init();

        util::irq_enable();

        sched::task_switch();
    }

    loop {}
}

pub fn kernel_start() {
    let mut x: u32 = 0xffff_ff00;
    let a = Box::new(10);
    let b = Box::new(11);
    x = x + *a;
    x = x + *b;
    loop {
        if x % 1000 == 0 {
            unsafe {
                sched::task_switch();
            }
        }
        x = x.wrapping_add(1);
    }
}

unsafe fn clock_init() {
    /* SysTick Control and Status Register (STCTRL): 0xe000e010 */
    util::write_or(0xe000e010 as *mut u32, 0b111);

    /*
     * SysTick Reload Value Register (STRELOAD): 0xe000e014
     * 10/((1 * 1000 ms)/(12 * 1000 * 1000 Hz)) = 120000
     *                                          = 0b1_1101_0100_1100_0000
     * => 10ms per a tick
     */
    util::write(0xe000e014 as *mut u32, 0b1_1101_0100_1100_0000 - 1);

    /*
     * Run-Mode Clock Configuration (RCC): 0x400fe060
     * use Internal oscillator
     */
    util::write_modify(0x400fe060 as *mut u32, 0b01_0000, 0b11_0000);
}
