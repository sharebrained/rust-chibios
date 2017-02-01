
use c::rust_hal_lld_get_counter_value;

pub fn lld_get_counter_value() -> u32 {
	unsafe {
		rust_hal_lld_get_counter_value()
	}
}
