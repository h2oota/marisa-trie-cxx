
#![allow(dead_code)]

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::marker::PhantomData;

use super::*;
use super::marisa_wrapper::*;

// struct enum

enum RawObject {
    Key(*mut marisa_Key),
    Query(*mut marisa_Query),
    Keyset(*mut marisa_Keyset),
    Agent(*mut marisa_Agent),
    Trie(*mut marisa_Trie),
    ConstKey(*const marisa_Key),
    ConstQuery(*const marisa_Query),
}

// Lifetime management of a struct that has a C++ object pointer called const some_Class as a member.
pub struct Object<'a, T> {
    object: RawObject,
    _marker: PhantomData<&'a T>,
}

pub type KeyObject<'a> = Object<'a, marisa_Key>;
pub type QueryObject<'a> = Object<'a, marisa_Query>;
pub type KeysetObject<'a> = Object<'a, marisa_Keyset>;
pub type AgentObject<'a> = Object<'a, marisa_Agent>;
pub type TrieObject<'a> = Object<'a, marisa_Trie>;

// Common
pub trait BaseTrait<T> {
    fn new() -> Self;
}

trait PointerTrait<T> {
    fn const_pointer(&self) -> *const T;
    fn mut_pointer(&mut self) -> *mut T;
}

fn from_exception(err_record: *const exception_record) -> MarisaError
{
    unsafe {
	MarisaError {
	    source: CStr::from_ptr(exception_name(err_record)).to_string_lossy().into_owned(),
	    message: CStr::from_ptr(exception_message(err_record)).to_string_lossy().into_owned()
	}
    }
}

struct KQFunc<T> {
    get: unsafe extern "C" fn(*const T, index: usize, *mut *const exception_record) -> utils::cuchar,
    ptr: unsafe extern "C" fn(*const T, *mut *const exception_record) -> *const utils::cuchar,
    length: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize,
    id: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize,
    set_str: unsafe extern "C" fn(*mut T, *const c_char, length: usize, *mut *const exception_record),
    set_id: unsafe extern "C" fn(*mut T, id: usize, *mut *const exception_record),
}

fn get_ptr_length<T>(obj: *const T, funcs: &KQFunc<T>) ->
    Result<(*const utils::cuchar, usize), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let ptr = {
	    let ptr = unsafe { (funcs.ptr)(obj, &mut err_record) };
	    if !err_record.is_null() {
		return Err(from_exception(err_record));
	    }
	    ptr
	};
	let length = {
	    let length = unsafe { (funcs.length)(obj, &mut err_record) };
	    if !err_record.is_null() {
		return Err(from_exception(err_record));
	    }
	    length
	};
	Ok((ptr, length))
}

// Key and Query Common
pub trait KQTrait<T>: BaseTrait<T> + PointerTrait<T> {
    const FUNCS: KQFunc<T>;

    fn str(&self) -> Result<&str, MarisaError> {
	let obj = self.const_pointer();
	let (ptr, length) = get_ptr_length::<T>(obj, &Self::FUNCS)?;
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
	let obj = self.const_pointer();
	let (ptr, length) = get_ptr_length::<T>(obj, &Self::FUNCS)?;
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

    fn get(&self, i: usize) -> Result<char, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.get)(obj, i, &mut err_record) as char };
	if err_record.is_null() {
	    Ok(value as char)
	} else {
	    Err(from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn ptr(&self) -> Result<*const u8, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.ptr)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value as *const u8)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn id(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.id)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn length(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FUNCS.length)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_str(&mut self, str: &str) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FUNCS.set_str)(obj, str.as_ptr() as *const c_char, str.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_str_length(&mut self, str: &str, length: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FUNCS.set_str)(obj, str.as_ptr() as *const c_char, length, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FUNCS.set_id)(obj, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }
}

pub trait KeyTrait: BaseTrait<marisa_Key> + PointerTrait<marisa_Key>  {
//    #[cfg(not(release))]
    fn weight(&self) -> Result<f32, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe {(key_weight)(obj, &mut err_record)};
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_weight(&mut self, weight: f32) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(key_set_weight)(obj, weight, &mut err_record)};
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }
}

pub trait QueryTrait: BaseTrait<marisa_Query> + PointerTrait<marisa_Query> {
    fn clear(&mut self) -> Result<(), MarisaError>  {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (query_clear)(obj, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }
}

pub trait KeysetTrait: BaseTrait<marisa_Keyset> + PointerTrait<marisa_Keyset> {
    fn push_back_key(&mut self, key: &KeyObject) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_0)(obj, key.const_pointer(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn push_back_key_em(&mut self, key: &KeyObject, end_marker: char) -> Result<(), MarisaError> {
  	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_1)(
	    obj, key.const_pointer(), end_marker as utils::cuchar, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    // Rust strings are not null-terminated, so
    // void push_back(marisa::Keyset*, const char *);
    // wrapper will not be implemented

    fn push_back_str(&mut self, key: &str) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_3)(obj, key.as_ptr(), key.len(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn push_back_str_weight(&mut self, key: &str, weight: utils::cfloat) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_4)(obj, key.as_ptr(), key.len(), weight, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn push_back_bin_weight(&mut self, key: &[u8], weight: utils::cfloat) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_4)(obj, key.as_ptr(), key.len(), weight, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn get(&self, i: usize) -> Result<KeyObject<'_>, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let key_obj = unsafe { (keyset_get)(obj, i, &mut err_record)};

	// Since the returned Object<marisa_Key> cannot acquire a mut_pointer, the data on the libmarisa side will not be deleted.
	if err_record.is_null() {
	    Ok(KeyObject {
		object: RawObject::ConstKey(key_obj),
		_marker: PhantomData
	    })
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn put(&mut self, i: usize, mut v: KeyObject) -> Result<KeyObject<'_>, MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let key_obj = unsafe { (keyset_put)(obj, i, v.mut_pointer(), &mut err_record) };

	// Since the returned Object<marisa_Key> cannot obtain a mut_pointer, the data on the libmarisa side will not be deleted.
	if err_record.is_null() {
	    Ok(KeyObject {
		object: RawObject::ConstKey(key_obj),
		_marker: PhantomData,
	    })
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn num_keys(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_num_keys(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    unsafe {
		Err(MarisaError {
		    source: CStr::from_ptr(exception_name(err_record)).to_string_lossy().into_owned(),
		    message: CStr::from_ptr(exception_message(err_record)).to_string_lossy().into_owned()
		})
	    }
	}
    }

    fn empty(&self) -> Result<bool, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_empty(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn size(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_size(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn total_length(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_total_length(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn reset(&mut self) -> Result<(), MarisaError>  {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_reset)(obj, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn clear(&mut self) -> Result<(), MarisaError>  {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_clear)(obj, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn swap(&mut self, mut rhs: KeysetObject) -> Result<(), MarisaError>  {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let rhs_obj = rhs.mut_pointer();
	unsafe { (keyset_swap)(obj, rhs_obj, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }
}

pub trait AgentTrait: BaseTrait<marisa_Agent> + PointerTrait<marisa_Agent> {
    fn query(&self) -> Result<QueryObject<'_>, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let query_obj = unsafe { (agent_query)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(QueryObject {
		object: RawObject::ConstQuery(query_obj),
		_marker: PhantomData,
	    })
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn key(&self) -> Result<KeyObject<'_>, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let key_obj = unsafe { (agent_key)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(KeyObject {
		object: RawObject::ConstKey(key_obj),
		_marker: PhantomData
	    })
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_query_str(&mut self, s: &str) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_query_1)(obj, s.as_ptr(), s.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn set_query_str_len(&mut self, s: &str, l: usize) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_query_1)(obj, s.as_ptr(), l, &mut err_record) };
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

pub trait TrieTrait: BaseTrait<marisa_Trie> + PointerTrait<marisa_Trie> {
    fn build(&mut self, keyset: &mut KeysetObject, config_flags: utils::cint) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let keyset_obj = keyset.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_build)(obj, keyset_obj, config_flags, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn mmap(&mut self, filename: &str) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_mmap)(obj, filename.as_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn map(&mut self, ptr: &[u8]) -> Result<(), MarisaError>  {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_map)(obj, ptr.as_ptr() as *const std::os::raw::c_void,
					   ptr.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn load(&mut self, filename: &str) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_load)(obj, filename.as_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn save(&self, filename: &str) -> Result<(), MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_save)(obj, filename.as_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn lookup(&self, agent: &mut AgentObject) -> Result<bool, MarisaError> {
	let obj = self.const_pointer();
	let agt_obj = agent.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_lookup)(obj, agt_obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn reverse_lookup(&self, agent: &mut AgentObject) -> Result<(), MarisaError> {
	let obj = self.const_pointer();
	let agt_obj = agent.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_reverse_lookup)(obj, agt_obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn common_prefix_search(&self, agent: &mut AgentObject) -> Result<bool, MarisaError> {
	let obj = self.const_pointer();
	let agt_obj = agent.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_common_prefix_search)(obj, agt_obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn predictive_search(&self, agent: &mut AgentObject) -> Result<bool, MarisaError> {
	let obj = self.const_pointer();
	let agt_obj = agent.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_predictive_search)(obj, agt_obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn num_tries(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_num_tries)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn num_keys(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_num_keys)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn num_nodes(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_num_nodes)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn tail_mode(&self) -> Result<marisa_TailMode, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_tail_mode)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn node_order(&self) -> Result<marisa_NodeOrder, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value= unsafe { (trie_node_order)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn clear(&mut self) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_clear)(obj, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn swap(&mut self, mut rhs: TrieObject) -> Result<(), MarisaError> {
	let obj = self.mut_pointer();
	let rhs_obj = rhs.mut_pointer();
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_swap)(obj, rhs_obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn empty(&self) -> Result<bool, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_empty)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn size(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_size)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn total_size(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_total_size)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }

    fn io_size(&self) -> Result<usize, MarisaError> {
	let obj = self.const_pointer();
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_io_size)(obj, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(from_exception(err_record))
	}
    }
}


// implement


// RawObject
impl Drop for RawObject {
    fn drop(&mut self) {
	match self {
	    Self::Key(pointer) => unsafe { (key_destroy)(*pointer); },
	    Self::Query(pointer) => unsafe { (query_destroy)(*pointer); },
	    Self::Keyset(pointer) => unsafe { (keyset_destroy)(*pointer); },
	    Self::Agent(pointer) => unsafe { (agent_destroy)(*pointer); },
	    Self::Trie(pointer) => unsafe { (trie_destroy)(*pointer); },
	    Self::ConstKey(_) => {},
	    Self::ConstQuery(_) => {},
	}
    }
}

// Key
impl BaseTrait<marisa_Key> for KeyObject<'_> {
    fn new() -> Self {
	Self {
	    object: RawObject::Key(unsafe { (key_create)() }),
	    _marker: PhantomData::<&marisa_Key>,
	}
    }
}

impl PointerTrait<marisa_Key> for KeyObject<'_> {
    fn const_pointer(&self) -> *const marisa_Key {
	match self.object {
	    RawObject::Key(pointer) => pointer as *const marisa_Key,
	    RawObject::ConstKey(pointer) => pointer,
	    _ => panic!("wow!")
	}
    }

    fn mut_pointer(&mut self) -> *mut marisa_Key {
	// self.object.unwrap()
	if let RawObject::Key(pointer) = self.object {
	    pointer
	} else {
	    panic!("wow!")
	}
    }
}

impl KeyTrait for KeyObject<'_> {}

impl KQTrait<marisa_Key> for KeyObject<'_> {
    const FUNCS: KQFunc<marisa_Key> = KQFunc::<marisa_Key> {
	get: key_get,
	ptr: key_ptr,
	length: key_length,
	id: key_id,
	set_str: key_set_str,
	set_id: key_set_id,
    };
}

// Query
impl BaseTrait<marisa_Query> for QueryObject<'_> {
    fn new() -> Self {
	Self {
	    object: RawObject::Query(unsafe { (query_create)() }),
	    _marker: PhantomData::<&marisa_Query>,
	}
    }
}

impl PointerTrait<marisa_Query> for QueryObject<'_> {
    fn const_pointer(&self) -> *const marisa_Query {
	match self.object {
	    RawObject::Query(pointer) => pointer as *const marisa_Query,
	    RawObject::ConstQuery(pointer) => pointer,
	    _ => panic!("wow!")
	}
    }

    fn mut_pointer(&mut self) -> *mut marisa_Query {
	// self.object.unwrap()
	if let RawObject::Query(pointer) = self.object {
	    pointer
	} else {
	    panic!("wow!")
	}
    }
}

impl QueryTrait for QueryObject<'_> {}

impl KQTrait<marisa_Query> for QueryObject<'_> {
    const FUNCS: KQFunc<marisa_Query> = KQFunc::<marisa_Query> {
	get: query_get,
	ptr: query_ptr,
	length: query_length,
	id: query_id,
	set_str: query_set_str,
	set_id: query_set_id,
    };
}

// Keyset
impl BaseTrait<marisa_Keyset> for KeysetObject<'_> {
    fn new() -> Self {
	Self {
	    object: RawObject::Keyset(unsafe { (keyset_create)() }),
	    _marker: PhantomData::<&marisa_Keyset>
	}
    }
}

impl PointerTrait<marisa_Keyset> for KeysetObject<'_> {
    fn const_pointer(&self) -> *const marisa_Keyset {
	match self.object {
	    RawObject::Keyset(pointer) => pointer as *const marisa_Keyset,
	    _ => panic!("wow!")
	}
    }

    fn mut_pointer(&mut self) -> *mut marisa_Keyset {
	// self.object.unwrap()
	if let RawObject::Keyset(pointer) = self.object {
	    pointer
	} else {
	    panic!("wow!")
	}
    }
}

impl KeysetTrait for KeysetObject<'_> {}

// Agent
impl BaseTrait<marisa_Agent> for AgentObject<'_> {
    fn new() -> Self {
	Self {
	    object: RawObject::Agent(unsafe { (agent_create)() }),
	    _marker: PhantomData::<&marisa_Agent>,
	}
    }
}

impl PointerTrait<marisa_Agent> for AgentObject<'_> {
    fn const_pointer(&self) -> *const marisa_Agent {
	match self.object {
	    RawObject::Agent(pointer) => pointer as *const marisa_Agent,
	    _ => panic!("wow!")
	}
    }

    fn mut_pointer(&mut self) -> *mut marisa_Agent {
	// self.object.unwrap()
	if let RawObject::Agent(pointer) = self.object {
	    pointer
	} else {
	    panic!("wow!")
	}
    }
}

impl AgentTrait for AgentObject<'_> { }

// Trie
impl BaseTrait<marisa_Trie> for TrieObject<'_> {
    fn new() -> Self {
	Self {
	    object: RawObject::Trie(unsafe { (trie_create)() }),
	    _marker: PhantomData::<&marisa_Trie>,
	}
    }
}

impl PointerTrait<marisa_Trie> for TrieObject<'_> {
    fn const_pointer(&self) -> *const marisa_Trie {
	match self.object {
	    RawObject::Trie(pointer) => pointer as *const marisa_Trie,
	    _ => panic!("wow!")
	}
    }

    fn mut_pointer(&mut self) -> *mut marisa_Trie {
	// self.object.unwrap()
	if let RawObject::Trie(pointer) = self.object {
	    pointer
	} else {
	    panic!("wow!")
	}
    }
}

impl TrieTrait for TrieObject<'_> {}
