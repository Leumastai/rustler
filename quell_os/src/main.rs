
#![no_std] //disabling all standard libraries in the crate. try runnning cargo build
#![no_main] //telling rust that we don't want to use the main entrypoint of crt0
#![feature(asm)] //using experimental feature in nightly called asm
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffers;
use core::panic::PanicInfo;

// fn main() {}

// [profile.dev]
// panic = "abort"




// writing a custom unit test as normal unit test is in the std library/
// #[cfg(test)] // this makes it to run test on only functions with tests and not just any function
// pub fn test_runner(tests: &[&dyn Fn()]) { // &[&dyn Fn()] is a list of reference to types that can be called like a function
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }

//     exit_qemu(QemuExitCode::Success)
// }

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}


#[panic_handler]
fn panic (info: &PanicInfo) -> ! {
    println! ("{}", info);
    loop {
        // This terminate as it will never return
    }
}


// #[no_mangle] //prevents the Rust compiler from mangling the function name.
// //mang
// pub extern "C" fn _start() -> ! { //specifies the C calling convention, making the function callable from C code.
//     loop {}
// }


// static HELLO: &[u8] = b"Hello World";
// // 0xb8000 is the address that the cga buffer is at,
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;

//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//         }
//     }

//     loop {}
// }



// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     // vga_buffers::print_something();
//     use core::fmt::Write;

//     vga_buffers::WRITER.lock().write_str("Hello World").unwrap();
//     write!(vga_buffers::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();


//     loop {}
// }

#[test_case]
fn trivial_assertation() {
    print! ("trivial assertion.. ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    
    //panic!("You're Panicking!!");
    loop {}
}


// rustup target add thumbv7em-none-eabihf
// then run cargo build --target thumbv7em-none-eabihf
// to use rust-nightly for your project, create a 
// rust-toolchain folder and write `nightly` on line 1