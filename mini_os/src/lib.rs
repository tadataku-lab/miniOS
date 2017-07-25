#![feature(lang_items)]
#![feature(unique)]
#![no_std]
#![feature(const_fn)]

#[macro_use]
mod vga_buffer;

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}