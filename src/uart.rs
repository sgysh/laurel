/* for PL011 */

use core;

/* base address for lm3s811evb */
const UART0_BASE: u32 = 0x4000c000;

const UARTDR: u32 = UART0_BASE + 0x0;
const UARTFR: u32 = UART0_BASE + 0x18;

const UARTFR_TXFF: u32 = 1 << 5;
const UARTFR_RXFE: u32 = 1 << 4;

pub unsafe fn is_writeable() -> bool {
    core::ptr::read_volatile(UARTFR as *const u32) & UARTFR_TXFF == 0
}

pub unsafe fn is_readable() -> bool {
    core::ptr::read_volatile(UARTFR as *const u32) & UARTFR_RXFE == 0
}

pub unsafe fn write(val: u32) {
    core::ptr::write_volatile(UARTDR as *mut u32, val);
}

pub unsafe fn read() -> u32 {
    core::ptr::read_volatile(UARTDR as *const u32)
}
