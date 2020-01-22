// https://github.com/heim-rs/heim/blob/master/heim-memory/src/sys/macos/memory.rs
// https://github.com/heim-rs/heim/blob/master/heim-memory/src/sys/macos/bindings.rs
// https://github.com/heim-rs/heim/blob/master/heim-common/src/sys/macos/mod.rs

use std::io;
use std::mem;
use std::ptr;

use nix::libc;

use super::common;
use crate::memory::VirtualMemory;
use crate::PAGE_SIZE;

const CTL_HW: libc::c_int = 6;
const HW_MEMSIZE: libc::c_int = 24;

#[allow(trivial_casts)]
unsafe fn hw_memsize() -> io::Result<u64> {
	let mut name: [i32; 2] = [CTL_HW, HW_MEMSIZE];
	let mut value = 0u64;
	let mut length = mem::size_of::<u64>();

	let result = libc::sysctl(
		name.as_mut_ptr(),
		2,
		&mut value as *mut u64 as *mut libc::c_void,
		&mut length,
		ptr::null_mut(),
		0,
	);

	if result == 0 {
		Ok(value)
	} else {
		Err(io::Error::last_os_error())
	}
}

pub fn virtual_memory() -> io::Result<VirtualMemory> {
	let total = unsafe { hw_memsize()? };
	let vm_stats = unsafe { common::host_vm_info()? };
	let page_size = *PAGE_SIZE;

	let available = u64::from(vm_stats.active_count + vm_stats.free_count) * page_size;
	let shared = 0; // TODO
	let free = u64::from(vm_stats.free_count - vm_stats.speculative_count) * page_size;
	let buffers = 0; // TODO
	let cached = 0; // TODO
	let active = u64::from(vm_stats.active_count) * page_size;
	let inactive = u64::from(vm_stats.inactive_count) * page_size;
	let used = u64::from(vm_stats.active_count + vm_stats.wire_count) * page_size;
	let percent = 0.0; // TODO

	let wire = u64::from(vm_stats.wire_count) * page_size;

	Ok(VirtualMemory {
		total,
		available,
		shared,
		free,
		buffers,
		cached,
		active,
		inactive,
		used,
		percent,
	})
}