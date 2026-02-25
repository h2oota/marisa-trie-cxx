use std::marker::PhantomData;

use crate::marisa::error::*;
use crate::utils;
pub use crate::marisa::raw_object::api::*;

pub struct HiddenObject {
    _private: [u8; 0]
}


pub struct Object<T: LifeTrait> {
    object: T,
    _marker: PhantomData<HiddenObject>,
}


pub struct ObjectRef<'a, T> {
    object: T,
    _marker: PhantomData<&'a HiddenObject>,
}


pub struct ObjectRefMut<'a, T> {
    object: T,
    _marker: PhantomData<&'a mut HiddenObject>,
}


impl<T: LifeTrait> Drop for Object<T>
{
    fn drop(&mut self)
    {
	self.object.destroy();
    }
}


impl<T: LifeTrait> Default for Object<T>
{
    fn default() -> Self
    {
	Self {
	    object: T::new(),
	    _marker:  PhantomData
	}
    }
}


impl KeyTrait for Object<Key>
{
    fn weight(&self) -> Result<f32, MarisaError>
    {
	self.object.weight()
    }
}


impl KeyMutTrait for Object<Key>
{
    fn set_weight(&mut self, weight: f32) -> Result<(), MarisaError> {
	self.object.set_weight(weight)
    }
}


impl KeyQueryTrait for Object<Key>
{
    fn str(&self) -> Result<&str, MarisaError>
    {
	self.object.str()
    }

    fn bin(&self) -> Result<&[u8], MarisaError>
    {
	self.object.bin()
    }

    fn ptr(&self) -> Result<*const u8, MarisaError>
    {
	self.object.ptr()
    }

    fn id(&self) -> Result<usize, MarisaError>
    {
	self.object.id()
    }

    fn length(&self) -> Result<usize, MarisaError>
    {
	self.object.length()
    }
}


impl KeyQueryMutTrait for Object<Key>
{
    fn set_str(&mut self, str: &str) -> Result<(), MarisaError>
    {
	self.object.set_str(str)
    }

    fn set_str_length(&mut self, str: &str, length: usize) -> Result<(), MarisaError>
    {
	self.object.set_str_length(str, length)
    }

    fn set_id(&mut self, id: usize) -> Result<(), MarisaError>
    {
	self.object.set_id(id)
    }
}

impl KeysetTrait for Object<Keyset>
{
    fn num_keys(&self) -> Result<usize, MarisaError>
    {
	self.object.num_keys()
    }

    fn empty(&self) -> Result<bool, MarisaError>
    {
	self.object.empty()
    }

    fn size(&self) -> Result<usize, MarisaError>
    {
	self.object.size()
    }

    fn total_length(&self) -> Result<usize, MarisaError>
    {
	self.object.total_length()
    }
}

impl KeysetMutTrait<Object<Key>> for Object<Keyset>
{
    fn push_back_key(&mut self, key: &Object<Key>) -> Result<(), MarisaError>
    {
	self.object.push_back_key(&(key.object as ConstKey))
    }

    fn push_back_key_em(&mut self, key: &Object<Key>, end_marker: char) -> Result<(), MarisaError>
    {
	self.object.push_back_key_em(&(key.object as ConstKey), end_marker)
    }

    fn push_back_str(&mut self, key: &str) -> Result<(), MarisaError>
    {
	self.object.push_back_str(key)
    }

    fn push_back_str_weight(&mut self, key: &str, weight: utils::cfloat) -> Result<(), MarisaError>
    {
	self.object.push_back_str_weight(key, weight)
    }

    fn push_back_bin_weight(&mut self, key: &[u8], weight: utils::cfloat) -> Result<(), MarisaError>
    {
	self.object.push_back_bin_weight(key, weight)
    }

    fn reset(&mut self) -> Result<(), MarisaError>
    {
	self.object.reset()
    }

    fn clear(&mut self) -> Result<(), MarisaError>
    {
	self.object.clear()
    }

    fn xswap(&mut self, rhs: Self) -> Result<(), MarisaError>
    {
	self.object.xswap(rhs.object)
    }
}
