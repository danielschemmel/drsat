use std::convert::From;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct IndexedVec<Key, Value> {
	data: Vec<Value>,
	key_type: PhantomData<Key>,
}

impl<Key, Value> IndexedVec<Key, Value> {
	pub fn new(vars: Vec<Value>) -> IndexedVec<Key, Value> {
		// FIXME: this should be renamed to from_vec
		IndexedVec {
			data: vars,
			key_type: PhantomData,
		}
	}

	pub fn len(&self) -> Key
		where Key: From<usize>
	{
		Key::from(self.data.len())
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
    where usize: From<Key>
{
	type Output = Value;

	fn index(&self, index: Key) -> &Value {
		&self.data[usize::from(index)]
	}
}

impl<Key, Value> IndexMut<Key> for IndexedVec<Key, Value>
    where usize: From<Key>
{
	fn index_mut<'a>(&'a mut self, index: Key) -> &'a mut Value {
		&mut self.data[usize::from(index)]
	}
}

impl<Key, Value> fmt::Debug for IndexedVec<Key, Value>
    where Value: fmt::Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(&self.data as &fmt::Debug).fmt(f)
	}
}
