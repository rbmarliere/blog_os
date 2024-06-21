#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	blog_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello world{}", "!");
	// panic!("This is a panic message");

	blog_os::init();

	// unsafe {
	// 	*(0xdeadbeef as *mut u8) = 42;
	// }

	// fn stack_overflow() {
	// 	stack_overflow();
	// }
	// stack_overflow();

	// let ptr = 0xdeadbeef as *mut u8;
	// unsafe { *ptr = 42; }
	// let ptr = 0x20436c as *mut u8;
	// unsafe { let x = *ptr; }
	// println!("read worked");
	// unsafe { *ptr = 42; }
	// println!("write worked");
	// use x86_64::registers::control::Cr3;
	// let (level_4_page_table, _) = Cr3::read();
	// println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

	#[cfg(test)]
	test_main();

	println!("It did not crash!");
	blog_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}
