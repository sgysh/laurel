use alloc::vec_deque::VecDeque;
use sched;
use uart;

pub fn console_handler() {
    let mut buffer: VecDeque<u32> = VecDeque::with_capacity(10);
    push_back_data(&mut buffer, "> ");

    loop {
        unsafe {
            if uart::is_readable() {
                let val = uart::read();
                if val == '\r' as u32 {
                    push_back_data(&mut buffer, "\r\n> ");
                } else {
                    buffer.push_back(val);
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
