
#![allow(dead_code)]

use std::os::raw::c_char;
use std::ptr;
use std::marker::PhantomData;
use std::ops::Index;

use super::*;
use marisa_wrapper::*;

/// marisa-trieオブジェクトをラップする。オブジェクトに対する操作はこの構造体に実装する。
/// Wraps a marisa-trie object. Operations on the object are implemented in this structure.
pub struct RawObjectMut<T> {
    ptr: *mut T,
}


trait RawObjectPtr<T> {
    fn get_ptr(&self) -> *const T;
    fn from_ptr(ptr: *const T) -> Self;
}


impl<T> RawObjectPtr<T> for RawObjectMut<T> {
    fn get_ptr(&self) -> *const T
    {
	self.ptr as *const T
    }
}


trait RawObjectMutPtr<T> {
    fn get_mut_ptr(&self) -> *mut T;
    fn from_mut_ptr(ptr: *mut T) -> Self;
}


impl<T> RawObjectMutPtr<T> for RawObjectMut<T> {
    fn get_mut_ptr(&self) -> *mut T
    {
	self.ptr
    }

    fn from_mut_ptr(ptr: *mut T) -> Self
    {
	Self {
	    ptr
	}
    }
}


pub struct RawObject<T> {
    ptr: *const T,
}


impl<T> RawObjectPtr<T> for RawObject<T> {
    fn get_ptr(&self) -> *const T
    {
	self.ptr
    }

    fn from_ptr(ptr: *const T) -> Self {
	Self {
	    ptr
	}
    }
}

/// marisa-trieオブジェクトの生成、破棄。
/// Creating and destroying a marisa-trie object.
trait RawObjectMutTrait<T>: RawObjectMutPtr<T> {
    const CREATE: unsafe extern "C" fn() -> *mut T;
    const DESTROY: unsafe extern "C" fn(ptr: *mut T);
    // const CREATE: fn() -> *mut T;
    // const DESTROY: fn(*mut T);

    fn new() -> Self where Self: Sized {
	Self::from_mut_ptr(unsafe{ (Self::CREATE)() })
    }

    fn destroy(&self) {
	unsafe { (Self::DESTROY)(Self::get_mut_ptr(self)) }
    }
}


impl RawObjectMutTrait<marisa_Key> for RawObjectMut<marisa_Key> {
    const CREATE: unsafe extern "C" fn() -> *mut marisa_Key = key_create;
    const DESTROY: unsafe extern "C" fn(ptr: *mut marisa_Key) = key_destroy;
}


impl RawObjectMutTrait<marisa_Query> for RawObjectMut<marisa_Query> {
    const CREATE: unsafe extern "C" fn() -> *mut marisa_Query = query_create;
    const DESTROY: unsafe extern "C" fn(ptr: *mut marisa_Query) = query_destroy;
}


impl RawObjectMutTrait<marisa_Keyset> for RawObjectMut<marisa_Keyset> {
    const CREATE: unsafe extern "C" fn() -> *mut marisa_Keyset = keyset_create;
    const DESTROY: unsafe extern "C" fn(ptr: *mut marisa_Keyset) = keyset_destroy;
}


impl RawObjectMutTrait<marisa_Agent> for RawObjectMut<marisa_Agent> {
    const CREATE: unsafe extern "C" fn() -> *mut marisa_Agent = agent_create;
    const DESTROY: unsafe extern "C" fn(ptr: *mut marisa_Agent) = agent_destroy;
}


impl RawObjectMutTrait<marisa_Trie> for RawObjectMut<marisa_Trie> {
    const CREATE: unsafe extern "C" fn() -> *mut marisa_Trie = trie_create;
    const DESTROY: unsafe extern "C" fn(ptr: *mut marisa_Trie) = trie_destroy;
}


pub struct KeyQueryFuncs0<T> {
//    get: fn(*const T, index: usize, *mut *const exception_record) -> utils::cuchar,
    ptr: unsafe extern "C" fn(*const T, *mut *const exception_record) -> *const utils::cuchar,
    length: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize,
    id: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize,
}

pub struct KeyQueryFuncs1<T> {
    set_str: unsafe extern "C" fn(*mut T, *const c_char, length: usize, *mut *const exception_record),
    set_id: unsafe extern "C" fn(*mut T, id: usize, *mut *const exception_record),
}

pub trait KeyQueryMutTrait0<T>: RawObjectPtr<T> {
    const FUNCS: KeyQueryFuncs0<T>;

    fn get_ptr_length(&self) ->	Result<(*const utils::cuchar, usize), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	let ptr = {
	    let ptr = unsafe { (Self::FUNCS.ptr)(self.get_ptr(), &mut err_record) };
	    if !err_record.is_null() {
		return Err(MarisaException::from_exception(err_record));
	    }
	    ptr
	};
	let length = {
	    let length = unsafe { (Self::FUNCS.length)(self.get_ptr(), &mut err_record) };
	    if !err_record.is_null() {
		return Err(MarisaException::from_exception(err_record));
	    }
	    length
	};
	Ok((ptr, length))
    }

    fn str(&self) -> Result<&str, MarisaError> {
	let (ptr, length) = self.get_ptr_length()?;
	if ptr.is_null() && length > 0 {
	    Err(MarisaError {
		source: "KQTrait::str".to_string(),
		message: "get_ptr_length returns NULL".to_string()
	    })
	} else if length == 0 {
	    Ok("")
	} else {
            std::str::from_utf8(unsafe {std::slice::from_raw_parts(ptr, length)})
		.map_err(|e| MarisaError {
		    source: "KQTrait::str std::str::from_utf8".to_string(),
		    message: e.to_string()
		})
	}
    }

    fn bin(&self) -> Result<&[u8], MarisaError> {
	let (ptr, length) = self.get_ptr_length()?;
	if ptr.is_null() && length > 0 {
	    Err(MarisaError {
		source: "KQTrait::bin".to_string(),
		message: "get_ptr_length returns Error".to_string()
	    })
	} else if length == 0 {
	    Ok(b"")
        } else {
	    Ok((unsafe { std::slice::from_raw_parts(ptr, length) }) as &[u8])
	}
    }

/*
    fn get(&self, i: usize) -> Result<char, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.get)(self.ptr, i, &mut err_record) as char };
	if err_record.is_null() {
	    Ok(value as char)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
*/

//    #[cfg(not(release))]
    fn ptr(&self) -> Result<*const u8, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.ptr)(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value as *const u8)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn id(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.id)(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn length(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.length)(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

//impl KeyQueryMutTrait0<marisa_Key> for RawObjectPtr<marisa_Key> {
impl<T: RawObjectPtr<marisa_Key>> KeyQueryMutTrait0<marisa_Key> for T {
    const FUNCS: KeyQueryFuncs0<marisa_Key> =
	KeyQueryFuncs0::<marisa_Key> {
	    ptr: key_ptr,
	    length: key_length,
	    id: key_id,
	};

}

//impl KeyQueryMutTrait0<marisa_Query> for RawObjectPtr<marisa_Query> {
impl<T: RawObjectPtr<marisa_Query>> KeyQueryMutTrait0<marisa_Query> for T {
    const FUNCS: KeyQueryFuncs0<marisa_Query> =
	KeyQueryFuncs0::<marisa_Query> {
	    ptr: query_ptr,
	    length: query_length,
	    id: query_id,
	};

}

pub trait KeyQueryMutTrait1<T>: RawObjectMutPtr<T> {
    const FUNCS: KeyQueryFuncs1<T>;

//    #[cfg(not(release))]
    fn set_str(&mut self, str: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FUNCS.set_str)(self.get_mut_ptr(), str.as_ptr() as *const c_char, str.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_str_length(&mut self, str: &str, length: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FUNCS.set_str)(self.get_mut_ptr(), str.as_ptr() as *const c_char, length, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FUNCS.set_id)(self.get_mut_ptr(), id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}


pub trait KeysetTrait0: RawObjectPtr<marisa_Keyset> {
    fn index(&self, i: usize) -> Result<RawObjectPtr<marisa_Key>, MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	let key_ptr = unsafe { (agent_key)(self.get_ptr(), &mut err_record) };

    }

    fn num_keys(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_num_keys(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn empty(&self) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_empty(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn size(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_size(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn total_length(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_total_length(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

pub trait KeysetTrait1: RawObjectMutPtr<marisa_Keyset> {
    fn push_back_key(&mut self, key: &KeyObject) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_0)(self.get_mut_ptr(), key.const_pointer(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn push_back_key_em(&mut self, key: &KeyObject, end_marker: char) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_1)(
	    self.get_mut_ptr(), key.const_pointer(), end_marker as utils::cuchar, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    // Rust strings are not null-terminated, so
    // void push_back(marisa::Keyset*, const char *);
    // wrapper will not be implemented
    fn push_back_str(&mut self, key: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_3)(self.get_mut_ptr(), key.as_ptr(), key.len(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn push_back_str_weight(&mut self, key: &str, weight: utils::cfloat) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_4)(self.get_mut_ptr(), key.as_ptr(), key.len(), weight, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn push_back_bin_weight(&mut self, key: &[u8], weight: utils::cfloat) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_4)(self.get_mut_ptr(), key.as_ptr(), key.len(), weight, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn reset(&mut self) -> Result<(), MarisaError>  {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_reset)(self.get_mut_ptr(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn clear(&mut self) -> Result<(), MarisaError>  {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_clear)(self.get_mut_ptr(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn swap(&mut self, mut rhs: KeysetObject) -> Result<(), MarisaError>  {
	let mut err_record: *const exception_record = ptr::null();
	let rhs_obj = rhs.mut_pointer();
	unsafe { (keyset_swap)(self.get_mut_ptr(), rhs_obj, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

impl std::ops::Index<usize> for RawObjectMutPtr<marisa_Keyset> {
}

//pub trait KeysetTrait0: RawObjectPtr<marisa_Keyset> {
//pub trait KeysetTrait1: RawObjectMutPtr<marisa_Keyset> {

/*
pub trait AgentTrait0: RawObjectPtr<marisa_Agent> {
    fn query(&self) -> Result<RawObjectPtr<marisa_Query>, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let query_ptr = unsafe { (agent_query)(self.get_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(RawObjectPtr::<marisa_Query>::from_ptr(query_ptr))
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn key(&self) -> Result<KeyObject<'_>, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let key_ptr = unsafe { (agent_key)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(RawObjectPtr::<marisa_Key>::from_ptr(key_ptr))
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn has_state(&self) -> Result<bool, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (agent_has_state)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }
}
*/

/*
pub trait AgentTrait1: RawObjectPtr<marisa_Agent> {
    fn set_query_str(&mut self, s: &str) -> Result<() , MarisaError>
    {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { agent_set_query_1(obj, s.as_ptr(), s.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_query_str_len(&mut self, s: &str, l: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { agent_set_query_1(obj, s.as_ptr(), l, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_query_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_query_2)(obj, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_key_str(&mut self, s: &str) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_key_1)(obj, s.as_ptr(), s.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_key_str_length(&mut self, s: &str, l: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_key_1)(obj, s.as_ptr(), l, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_key_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_key_2)(obj, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn init_state(&mut self) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_init_state)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }
    fn clear(&mut self) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_clear)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn swap(&mut self, mut rhs: AgentObject) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let rhs_obj = rhs.mut_pointer();
	unsafe { (agent_swap)(obj, rhs_obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }
}
*/
