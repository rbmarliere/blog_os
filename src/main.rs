#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::allocator;
use blog_os::memory::{self, BootInfoFrameAllocator};
use blog_os::println;
use blog_os::task::executor::Executor;
use blog_os::task::{keyboard, Task};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

extern crate alloc;

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

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
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

	// use blog_os::memory::active_level_4_table;
	// use x86_64::{structures::paging::PageTable, VirtAddr};
	// let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	// let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
	// for (i, entry) in l4_table.iter().enumerate() {
	// 	if !entry.is_unused() {
	// 		println!("L4 Entry {}: {:?}", i, entry);
	// 		let phys = entry.frame().unwrap().start_address();
	// 		let virt = phys.as_u64() + boot_info.physical_memory_offset;
	// 		let ptr = VirtAddr::new(virt).as_mut_ptr();
	// 		let l3_table: &PageTable = unsafe { &*ptr };
	// 		for (i, entry) in l3_table.iter().enumerate() {
	// 			if !entry.is_unused() {
	// 				println!("L3 Entry {}: {:?}", i, entry);
	// 			}
	// 		}
	// 	}
	// }

	// // use blog_os::memory::translate_addr;
	// use blog_os::memory;
	// use x86_64::{structures::paging::Translate, VirtAddr};
	// let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	// let addresses = [
	// 	0xb8000,
	// 	0x201008,
	// 	0x0100_02020_1a10,
	// 	boot_info.physical_memory_offset,
	// ];
	// // for &addr in &addresses {
	// // 	let virt = VirtAddr::new(addr);
	// // 	let phys = unsafe { translate_addr(virt, phys_mem_offset) };
	// // 	println!("{:?} -> {:?}", virt, phys);
	// // }
	// let mapper = unsafe { memory::init(phys_mem_offset) };
	// for &addr in &addresses {
	// 	let virt = VirtAddr::new(addr);
	// 	let phys = mapper.translate_addr(virt);
	// 	println!("{:?} -> {:?}", virt, phys);
	// }

	// use blog_os::memory;
	// use x86_64::{structures::paging::Page, VirtAddr};
	// let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	// let mut mapper = unsafe { memory::init(phys_mem_offset) };
	// // let mut frame_allocator = memory::EmptyFrameAllocator;
	// let mut frame_allocator = unsafe {
	// 	memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
	// };
	// let page = Page::containing_address(VirtAddr::new(0));
	// // let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
	// memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
	// let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	// unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	// use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
	// let heap_value = Box::new(41);
	// println!("heap_value at {:p}", heap_value);
	// let mut vec = Vec::new();
	// for i in 0..500 {
	// 	vec.push(i);
	// }
	// println!("vec at {:p}", vec.as_slice());
	// let rc = Rc::new(vec![1, 2, 3]);
	// let clone = rc.clone();
	// println!("current reference count is {}", Rc::strong_count(&clone));
	// core::mem::drop(rc);
	// println!("reference count is now {}", Rc::strong_count(&clone));

	#[cfg(test)]
	test_main();

	// let mut executor = SimpleExecutor::new();
	let mut executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();
}

async fn async_number() -> u32 {
	42
}

async fn example_task() {
	let n = async_number().await;
	println!("async number: {}", n);
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}
