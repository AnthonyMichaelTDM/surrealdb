//! Stores the next and available freed IDs for documents
use crate::key::category::Categorise;
use crate::key::category::Category;
use crate::kvs::impl_key;
use serde::{Deserialize, Serialize};

// Table ID generator
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ti {
	__: u8,
	_a: u8,
	pub ns: u32,
	_b: u8,
	pub db: u32,
	_c: u8,
	_d: u8,
	_e: u8,
}
impl_key!(Ti);

pub fn new(ns: u32, db: u32) -> Ti {
	Ti::new(ns, db)
}

impl Categorise for Ti {
	fn categorise(&self) -> Category {
		Category::DatabaseTableIdentifier
	}
}

impl Ti {
	pub fn new(ns: u32, db: u32) -> Self {
		Ti {
			__: b'/',
			_a: b'+',
			ns,
			_b: b'*',
			db,
			_c: b'!',
			_d: b't',
			_e: b'i',
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::kvs::{KeyDecode, KeyEncode};
	#[test]
	fn key() {
		use super::*;
		#[rustfmt::skip]
		let val = Ti::new(
			123u32,
			234u32,
		);
		let enc = Ti::encode(&val).unwrap();
		let dec = Ti::decode(&enc).unwrap();
		assert_eq!(val, dec);
	}
}
