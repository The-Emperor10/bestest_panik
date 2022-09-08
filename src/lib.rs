// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// the LICENSE file for more details.

use libc::{puts, rand};
use std::{ffi::CString, fmt::Debug};

pub fn panik(msg: &CString) -> ! {
	unsafe {
		loop {
			let mut rand_ptr = rand() as *mut u8;
			for i in msg.as_bytes() {
				let fuck = rand_ptr.as_mut().unwrap_unchecked();
				*fuck = *i;
				rand_ptr = rand_ptr.offset(1);
			}
			puts(msg.as_ptr());
			// TODO: Make full glory of `bestest_panik` available on all platforms
			#[cfg(unix)]
			libc::fork();
		}
	}
}

pub trait UnwrapPanik<T> {
	fn unwrap_panik(self) -> T;
}

impl<T, E> UnwrapPanik<T> for Result<T, E>
where
	E: Debug,
{
	fn unwrap_panik(self) -> T {
		match self {
			Ok(t) => t,
			Err(e) => panik!(
				"called `Result::unwrap_panik()` on an `Err` value` {:?}",
				&e
			),
		}
	}
}

pub trait UnwrapErrPanik<E> {
	fn unwrap_err_panik(self) -> E;
}

impl<T, E> UnwrapErrPanik<E> for Result<T, E>
where
	T: Debug,
{
	fn unwrap_err_panik(self) -> E {
		match self {
			Ok(t) => panik!(
				"called `Result::unwrap_err_panik()` on an `Ok` value` {:?}",
				t
			),
			Err(e) => e,
		}
	}
}

impl<T> UnwrapPanik<T> for Option<T> {
	fn unwrap_panik(self) -> T {
		match self {
			Some(t) => t,
			None => panik!("called `Option::unwrap_panik()` on a `None` value"),
		}
	}
}

pub trait UnwrapNonePanik {
	fn unwrap_none_panik(self);
}

impl<T> UnwrapNonePanik for Option<T>
where
	T: Debug,
{
	fn unwrap_none_panik(self) {
		match self {
			Some(t) => panik!(
				"called `Option::unwrap_none_panik()` on a `Some` value {:?}",
				&t
			),
			None => (),
		}
	}
}

/// The bestest panik ever invented.
/// Will very much panikkkk.
#[macro_export]
macro_rules! panik {
	($($in:expr),+) => {{
		use std::ffi::CString;
		$crate::panik(&CString::new(format!($($in),+)).expect("pANikkkk"))
	}};
	() => {
		panik!("pANikkkk")
	};
}

#[cfg(test)]
mod tests {
	use rusty_fork::rusty_fork_test;

	use crate::{UnwrapErrPanik, UnwrapNonePanik, UnwrapPanik};
	extern "C" fn handle_sigsegv(signal: libc::c_int) {
		if signal == libc::SIGSEGV {
			std::process::exit(0);
		}
	}

	fn test_init() {
		unsafe {
			libc::signal(
				libc::SIGSEGV,
				handle_sigsegv as extern "C" fn(libc::c_int) as libc::sighandler_t,
			);
		}
	}

	// Normally tests all run in the same process, which results in the test process ending
	// when a segfault happens. `rusty_fork_test` runs them all in seperate processes.
	rusty_fork_test! {
		#[test]
		fn test_macro() {
			test_init();
			panik!();
			#[allow(unreachable_code)] {
				unreachable!("this should be unreachable");
			}
		}

		#[test]
		fn test_result_panik() {
			test_init();
			let r: Result<&str, &str> = Err("oof");
			r.unwrap_panik();
			unreachable!("this should be unreachable");
		}

		#[test]
		fn test_result_err_panik() {
			test_init();
			let r: Result<&str, &str> = Ok("oof");
			r.unwrap_err_panik();
			unreachable!("this should be unreachable");
		}

		#[test]
		fn test_option_panik() {
			test_init();
			let r: Option<()> = None;
			r.unwrap_panik();
			unreachable!("this should be unreachable");
		}

		#[test]
		fn test_option_none_panik() {
			test_init();
			let r: Option<&str> = Some("oof");
			r.unwrap_none_panik();
			unreachable!("this should be unreachable");
		}
	}
}
