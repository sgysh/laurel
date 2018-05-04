use alloc::string::String;
use alloc::vec_deque::VecDeque;
use builtin;
use core;
use sched;
use uart;

pub fn console_handler() {
    let mut buffer: VecDeque<u32> = VecDeque::with_capacity(10);
    let mut command: String = String::with_capacity(10);
    push_back_data(&mut buffer, "> ");

    loop {
        unsafe {
            if uart::is_readable() {
                let val = uart::read();
                if val == '\r' as u32 {
                    command = builtin::run_command(command);
                    push_back_data(&mut buffer, "\r\n> ");
                } else {
                    buffer.push_back(val);
                    command.push(core::char::from_u32(val).unwrap());
                }
            } else if uart::is_writeable() && !buffer.is_empty() {
                uart::write(buffer.pop_front().unwrap());
            } else {
                sched::task_switch();
            }
        }
    }
}

fn push_back_data(buffer: &mut VecDeque<u32>, data: &str) {
    for c in data.chars() {
        buffer.push_back(c as u32);
    }
}

pub fn write_console(data: &str) {
    for c in data.chars() {
        loop {
            unsafe {
                if uart::is_writeable() {
                    uart::write(c as u32);
                    break;
                } else {
                    sched::task_switch();
                }
            }
        }
    }
}
