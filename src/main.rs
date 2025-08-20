#![no_std]
#![no_main]

extern crate alloc;

use app::{init, println, hlt_loop};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use app::task::Executor;
use app::task::keyboard;
use alloc::boxed::Box;


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("xairorosx");
    init(boot_info);

    let mut executor = Executor::new();
    executor.spawn(Box::pin(keyboard::print_keypresses()));
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
