use console;
use kernel;
use util;

#[used]
static mut JIFFIES: u32 = 0xffff_ff00;

#[used]
static mut KERNEL_STACK: [u32; 512] = [0xdeadbeef; 512];
#[used]
static mut CONSOLE_STACK: [u32; 1024] = [0xeeeeeeee; 1024];

struct Tcb {
    sp: *mut u32,
}

impl Tcb {
    pub const fn new(p: *mut u32) -> Tcb {
        Tcb { sp: p }
    }
}

static mut TCBS: [Tcb; 2] = [
    Tcb::new(0xaaaaaaaa as *mut u32),
    Tcb::new(0xbbbbbbbb as *mut u32),
];

static mut CURRENT_TCB_INDEX: usize = 0;

pub unsafe fn task_init() {
    KERNEL_STACK[KERNEL_STACK.len() - 1] = 0x01000000; /* xPSR  */
    KERNEL_STACK[KERNEL_STACK.len() - 2] =
        util::align_down(kernel::kernel_start as usize, 2) as u32; /* PC */
    KERNEL_STACK[KERNEL_STACK.len() - 3] = util::align_down(task_finished as usize, 2) as u32; /* LR */

    let sp = KERNEL_STACK
        .as_ptr()
        .offset(KERNEL_STACK.len() as isize - 8) as *mut u32;
    asm!(
        "
        msr psp, $0
        "
        :
        : "r" (sp)
        :
    );

    CONSOLE_STACK[CONSOLE_STACK.len() - 1] = 0x01000000; /* xPSR  */
    CONSOLE_STACK[CONSOLE_STACK.len() - 2] =
        util::align_down(console::console_handler as usize, 2) as u32; /* PC */
    CONSOLE_STACK[CONSOLE_STACK.len() - 3] = util::align_down(task_finished as usize, 2) as u32; /* LR */

    TCBS[1].sp = CONSOLE_STACK
        .as_ptr()
        .offset(CONSOLE_STACK.len() as isize - 16) as *mut u32;
}

pub extern "C" fn systick_exception() {
    unsafe {
        JIFFIES = JIFFIES.wrapping_add(1);
        if JIFFIES % 10 == 0 {
            task_switch();
        }
    }
}

#[naked]
pub extern "C" fn pendsv_exception() {
    unsafe {
        let mut sp;

        asm!(
            "
            mrs r0, psp
            stmdb r0!, {r4-r11}
            str r0, [$0]
            "
            : "=r" (sp)
            :
            :
        );

        TCBS[CURRENT_TCB_INDEX].sp = sp;
        CURRENT_TCB_INDEX = (CURRENT_TCB_INDEX + 1) % 2;
        sp = TCBS[CURRENT_TCB_INDEX].sp;
        let r_sp: *mut u32 = (TCBS[CURRENT_TCB_INDEX].sp as i32 + 0x20) as *mut u32;

        asm!(
            "
            ldmia $0!, {r4-r11}
            msr psp, $1
            "
            :
            : "r" (sp), "r" (r_sp)
            :
        );

        asm!(
            "
            mov lr, #0xfffffffd
            bx lr
            "
        );
    }
}

pub unsafe fn task_switch() {
    /*
     * ICSR(Interrupt Control and State Register): 0xe000ed04
     * bit 28: PENDSVSET
     *   PendSV set-pending bit.
     *   Write:
     *     0 = no effect
     *     1 = changes PendSV exception state to pending.
     *   Read:
     *     0 = PendSV exception is not pending
     *     1 = PendSV exception is pending.
     *   Writing 1 to this bit is the only way to set the PendSV exception state to pending.
     */
    util::write_or(0xe000ed04 as *mut u32, 1 << 28);
}

fn task_finished() {
    loop {
        unsafe {
            task_switch();
        }
    }
}
