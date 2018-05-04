use alloc::string::ToString;
use console;
use sched;
use util;

pub fn run() {
    console::write_console("\r\n");
    console::write_console("PID CMD");

    unsafe {
        util::irq_disable();

        for i in 0..sched::get_tcb_len() {
            let tcb = sched::get_tcb(i);
            console::write_console("\r\n");
            console::write_console("  ");
            console::write_console(&*(i.to_string()));
            console::write_console(" ");
            console::write_console(tcb.name);
        }

        util::irq_enable();
    }
}
