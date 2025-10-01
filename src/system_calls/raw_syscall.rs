//! src/system_calls/raw_syscall.rs
//use std::arch::asm;

pub fn index(show: bool) {
    if show {
        println!("--- Raw Syscall");

        // This example only works in macOS M-series chip
        let message = "Hello world from raw syscall macOS M-series chip";
        let message = String::from(message);
        syscall(message);
    }
}

#[inline(never)]
fn syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();

    println!("ptr value: {:?}", ptr);
    println!("len value: {:?}", len);

    /*unsafe {
        asm!(
        "mov x16, 4",
        "mov x0, 1",
        "svc 0",
        in("x1") ptr,
        in("x2") len,
        out("x16") _,
        out("x0") _,
        lateout("x1") _,
        lateout("x2") _
        );
    }*/
}
