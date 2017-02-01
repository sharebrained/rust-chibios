
#[allow(non_camel_case_types)]
pub type eventmask_t = u32;

extern "C" {
	pub fn rust_hal_lld_get_counter_value() -> u32;

	pub fn chEvtWaitAny(mask: eventmask_t) -> eventmask_t;

	pub fn chCoreAlloc(size: usize) -> *mut u8;
	pub fn chCoreAllocI(size: usize) -> *mut u8;
	pub fn chCoreStatus() -> usize;
	
	pub fn chHeapAlloc(heapp: *mut u8, size: usize) -> *mut u8;
	pub fn chHeapFree(p: *mut u8);
	pub fn chHeapStatus(heapp: *mut u8, sizep: *mut usize) -> usize;
}
