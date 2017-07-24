#![feature(lang_items)]
#![feature(unique)]
#![no_std]
#![feature(const_fn)]

mod vga_buffer;
extern crate rlibc;
extern crate volatile;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::print_something();

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}