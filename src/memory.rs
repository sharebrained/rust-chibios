
pub mod core {
	use c::chCoreStatus;

	pub struct Status {
		pub bytes_free: usize,
	}

	impl Status {
		pub fn get() -> Status {
			Status {
				bytes_free: unsafe { chCoreStatus() },
			}
		}
	}
}

pub mod heap {
	use c::chHeapStatus;

	pub struct Status {
		pub fragmented_free_space: usize,
		pub fragments: usize,
	}

	/// ChibiOS chHeapFree wrapper
	impl Status {
		pub fn get() -> Status {
			let mut fragmented_free_space: usize = 0;
			let fragments = unsafe { chHeapStatus(0 as *mut u8, &mut fragmented_free_space as *mut usize) };
			Status {
				fragmented_free_space: fragmented_free_space,
				fragments: fragments,
			}
		}
	}
}
