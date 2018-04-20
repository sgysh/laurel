#![feature(used)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(naked_functions)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]

#![no_std]
#![no_main]

extern crate compiler_builtins;
extern crate alloc;

mod kernel;
mod sched;
mod util;
mod heap_allocator;
mod uart;
mod chunk;
mod console;

#[used]
static mut HEAP: [u8 ; 256] = [0xcc; 256];

#[global_allocator]
static mut HEAP_ALLOCATOR: heap_allocator::MyAllocator = heap_allocator::MyAllocator::empty();

#[no_mangle]
#[lang="panic_fmt"]
pub fn panic_fmt() -> ! {
    loop {}
}

#[lang="eh_personality"]
extern fn eh_personality () {}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}

#[link_section = ".reset_vector"]
#[used]
static RESET_VECTOR: extern "C" fn() = __start;

#[link_section = ".exceptions"]
#[used]
static EXCEPTIONS: [extern "C" fn(); 14] = [
    nmi_exception,            /* NMI */
    hardfault_exception,      /* Hard Fault */
    memfault_exception,       /* Memory Management Fault */
    busfault_exception,       /* Bus Fault */
    usagefault_exception,     /* Usage Fault*/
    unhandled_exception,      /* [Reserved] */
    unhandled_exception,      /* [Reserved] */
    unhandled_exception,      /* [Reserved] */
    unhandled_exception,      /* [Reserved] */
    svcall_exception,         /* SVCall */
    unhandled_exception,      /* Reserved for Debug */
    unhandled_exception,      /* [Reserved] */
    sched::pendsv_exception,  /* PendSV */
    sched::systick_exception  /* Systick */
];

extern "C" fn nmi_exception() {
    loop {}
}

extern "C" fn hardfault_exception() {
    loop {}
}

extern "C" fn memfault_exception() {
    loop {}
}

extern "C" fn busfault_exception() {
    loop {}
}

extern "C" fn usagefault_exception() {
    loop {}
}

extern "C" fn svcall_exception() {
    loop {}
}

extern "C" fn unhandled_exception() {
    loop {}
}

extern "C" fn unhandled_interrupt() {
    loop {}
}

#[link_section = ".interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [unhandled_interrupt; 240];

extern "C" fn __start() {
    unsafe {
        init_data();
        init_bss();

        HEAP_ALLOCATOR.init(HEAP.as_ptr().offset(0) as usize, HEAP.len());
    }

    kernel::start();
}

unsafe fn init_data() {
    extern "C" {
        static _data_lma_start: u32;

        static mut _edata: u32;
        static mut _sdata: u32;
    }

    let n = (&_edata as *const _ as usize - &_sdata as *const _ as usize) / core::mem::size_of::<u32>();

    core::intrinsics::copy_nonoverlapping(&_data_lma_start, &mut _sdata, n);
}

unsafe fn init_bss() {
    extern "C" {
        static mut _ebss: u32;
        static mut _sbss: u32;
    }

    let n = (&_ebss as *const _ as usize - &_sbss as *const _ as usize) / core::mem::size_of::<u32>();

    for x in 0..n {
        core::ptr::write_volatile(((&_sbss as *const u32).offset(x as isize)) as *mut _, 0);
    }
}

#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, len: usize) -> i32 {
    for i in 0.. {
        if i >= len {break;}

        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);

        let res = a as i32 - b as i32;
        if res != 0 {
            return res
        }
    }

    0
}
