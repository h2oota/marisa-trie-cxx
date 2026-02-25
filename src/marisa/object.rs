use std::marker::PhantomData;

use crate::marisa::raw_object::api;


pub struct HiddenObject {
    _private: [u8; 0]
}


pub struct Object<T: api::LifeTrait> {
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


impl<T: api::LifeTrait> Drop for Object<T>
{
    fn drop(&mut self)
    {
	self.object.destroy();
    }
}
