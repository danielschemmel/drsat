use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub trait USizeCast {
	fn to_usize(self) -> usize;
	fn from_usize(val: usize) -> Self;
}

impl USizeCast for usize {
	fn to_usize(self) -> usize {
		self
	}

	fn from_usize(val: usize) -> Self {
		val
	}
}

impl USizeCast for u32 {
	fn to_usize(self) -> usize {
		self as usize // sub 32 bit machines are unsupported anyway
	}

	fn from_usize(val: usize) -> Self {
		debug_assert!(val < ::std::u32::MAX as usize);
		val as Self
	}
}

pub struct IndexedVec<Key, Value> {
	data: Vec<Value>,
	key_type: PhantomData<Key>,
}

impl<Key, Value> IndexedVec<Key, Value> {
	pub fn from_vec(vars: Vec<Value>) -> IndexedVec<Key, Value> {
		IndexedVec {
			data: vars,
			key_type: PhantomData,
		}
	}

	pub fn len(&self) -> Key
		where Key: USizeCast
	{
		Key::from_usize(self.data.len())
	}
}

impl<Key, Value> Deref for IndexedVec<Key, Value> {
	type Target = [Value];

	fn deref(&self) -> &[Value] {
		&self.data
	}
}

impl<Key, Value> DerefMut for IndexedVec<Key, Value> {
	fn deref_mut(&mut self) -> &mut [Value] {
		&mut self.data
	}
}

impl<Key, Value> Index<Key> for IndexedVec<Key, Value>
  where Key: USizeCast
{
	type Output = Value;

	fn index(&self, index: Key) -> &Value {
		&self.data[index.to_usize()]
	}
}

impl<Key, Value> IndexMut<Key> for IndexedVec<Key, Value>
  where Key: USizeCast
{
	fn index_mut<'a>(&'a mut self, index: Key) -> &'a mut Value {
		&mut self.data[index.to_usize()]
	}
}

impl<Key, Value> fmt::Debug for IndexedVec<Key, Value>
  where Value: fmt::Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(&self.data as &fmt::Debug).fmt(f)
	}
}
