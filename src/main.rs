#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let target = vga_buffer.wrapping_add(0);

    unsafe {
        *target = b'!';
        *target.offset(1) = 0xb;
    }

    loop {}
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    loop {}
}
