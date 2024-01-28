//! Data parsed from Syzkaller using
//! [syzlang-parser](https://docs.rs/syzlang-parser/latest/syzlang_parser/).
//! This uses a known version of Syzkaller with test of post-processing to
//! ensure it works.
//!
//! By default, no features are enabled and therefore no data is included.
//! Specify the OS-es you are interested in as features.
//! 
#![cfg_attr(feature = "linux", doc = "
# Example
 
Below is an example where we take ownership of the data and perform
prost-processing.
 
```
let mut state = syzlang_data::linux::PARSED.write().unwrap();
let parsed = std::mem::take(&mut *state);

```
")]

lazy_static::lazy_static! {
	/// Newest git commit hash from version of Syzkaller used when building.
	#[derive(Default)]
	pub static ref SK_VERSION: std::sync::RwLock<String> = {
		let data = include_bytes!(concat!(env!("OUT_DIR"), "/skversion.txt"));
		std::sync::RwLock::new(std::str::from_utf8(data).unwrap().to_string())
	};
}

#[macro_export]
/// Load post-processed version of the data, panics on failure.
/// 
#[cfg_attr(feature = "linux", doc = "
# Example
 
```
let _load = syzlang_data::load_os!(linux);
```
")]
macro_rules! load_os {
	($os:ident) => { {
		let mut state = $crate::$os::PARSED.write().unwrap();
		let mut parsed = std::mem::take(&mut *state);
		parsed.insert_builtin().unwrap();
		parsed.postprocess().unwrap();
		parsed
	} };
}

// Defined all the data in separate modules
include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_version() {
		let version = SK_VERSION.read().unwrap();
		assert!(version.len() == 40);
	}

	#[test]
	fn test_load() {
		// Test post-processing of overything
		#[cfg(feature = "akaros")]
		load_os!(akaros);

		#[cfg(feature = "darwin")]
		load_os!(darwin);

		#[cfg(feature = "freebsd")]
		load_os!(freebsd);

		#[cfg(feature = "fuchsia")]
		load_os!(fuchsia);

		#[cfg(feature = "linux")]
		load_os!(linux);

		#[cfg(feature = "netbsd")]
		load_os!(netbsd);

		#[cfg(feature = "openbsd")]
		load_os!(openbsd);

		#[cfg(feature = "trusty")]
		load_os!(trusty);

		#[cfg(feature = "windows")]
		load_os!(windows);
	}

}
