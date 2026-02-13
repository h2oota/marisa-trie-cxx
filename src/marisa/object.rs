
#![allow(dead_code)]

use std::os::raw::c_char;
use std::ptr;
use std::marker::PhantomData;

use super::error::*;
use crate::utils;

use crate::ffi::*;
use crate::ffi::marisa_trie::{
    marisa_Key,
    marisa_Query,
    marisa_Keyset,
    marisa_Agent,
    marisa_Trie,

    key_create,
    key_destroy,
    key_index,
    key_ptr,
    key_length,
    key_id,
    key_weight,
    key_set_str,
    key_set_id,
    key_set_weight,

    query_create,
    query_destroy,
    query_get,
    query_ptr,
    query_length,
    query_id,
    query_set_str,
    query_set_id,
    query_clear,
    query_swap,

    keyset_create,
    keyset_destroy,
    keyset_push_back_0,
    keyset_push_back_1,
    keyset_push_back_2,
    keyset_push_back_3,
    keyset_push_back_4,
    keyset_get,
    keyset_put,
    keyset_num_keys,
    keyset_empty,
    keyset_size,
    keyset_total_length,
    keyset_reset,
    keyset_clear,
    keyset_swap,

    agent_create,
    agent_destroy,
    agent_query,
    agent_key,
    agent_set_query_0,
    agent_set_query_1,
    agent_set_query_2,
    agent_set_key_0,
    agent_set_key_1,
    agent_set_key_2,
    agent_has_state,
    agent_init_state,
    agent_clear,
    agent_swap,

    trie_create,
    trie_destroy,
    trie_build,
    trie_mmap,
    trie_map,
    trie_load,
    trie_read,
    trie_save,
    trie_write,
    trie_lookup,
    trie_reverse_lookup,
    trie_common_prefix_search,
    trie_predictive_search,
    trie_num_tries,
    trie_num_keys,
    trie_num_nodes,
    trie_tail_mode,
    trie_node_order,
    trie_empty,
    trie_size,
    trie_total_size,
    trie_io_size,
    trie_clear,
    trie_swap,

    exception_record,
    exception_name,
    exception_message,
};


pub trait KeyQueryRawFuncs<T> {
    const FN_PTR: unsafe extern "C" fn(*const T, *mut *const exception_record) -> *const utils::cuchar;
    const FN_LENGTH: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize;
    const FN_ID: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize;
    const FN_SET_STR: unsafe extern "C" fn(*mut T, *const c_char, length: usize, *mut *const exception_record);
    const FN_SET_ID: unsafe extern "C" fn(*mut T, id: usize, *mut *const exception_record);
}

pub trait KeyQueryTrait<T>: KeyQueryRawFuncs<T>
{
    fn get_ptr_length(&self) ->	Result<(*const utils::cuchar, usize), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	let ptr = {
	    let ptr = unsafe { (Self::FN_PTR)(self as *const Self as *const T, &mut err_record) };
	    if !err_record.is_null() {
		return Err(MarisaException::from_exception(err_record));
	    }
	    ptr
	};
	let length = {
	    let length = unsafe { (Self::FN_LENGTH)(self as *const Self as *const T, &mut err_record) };
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
	    Err(MarisaError::new("KeyQueryTrait::str","get_ptr_length returns NULL"))
	} else if length == 0 {
	    Ok("")
	} else {
	    std::str::from_utf8(unsafe {std::slice::from_raw_parts(ptr, length)})
	       .map_err(|e| MarisaError::new("KeyQueryTrait::str", "get_ptr_length returns NULL"))
	}
    }

    fn bin(&self) -> Result<&[u8], MarisaError> {
	let (ptr, length) = self.get_ptr_length()?;
	if ptr.is_null() && length > 0 {
	    Err(MarisaError::new("KeyQueryTrait::bin", "get_ptr_length returns Error"))
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
	let value = unsafe { (Self::FN_PTR)(self as *const Self as *const T, &mut err_record) };
	if err_record.is_null() {
	    Ok(value as *const u8)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn id(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FN_ID)(self as *const Self as *const T, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn length(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FN_LENGTH)(self as *const Self as *const T, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

pub trait KeyQueryMutTrait<T>: KeyQueryRawFuncs<T> {

//    #[cfg(not(release))]
    fn set_str(&mut self, str: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FN_SET_STR)(self as *mut Self as *mut T, str.as_ptr() as *const c_char, str.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_str_length(&mut self, str: &str, length: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FN_SET_STR)(self as *mut Self as *mut T, str.as_ptr() as *const c_char, length, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FN_SET_ID)(self as *mut Self as *mut T, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

impl KeyQueryRawFuncs<marisa_Key> for marisa_Key {
    const FN_PTR: unsafe extern "C" fn(*const marisa_Key, *mut *const exception_record) -> *const utils::cuchar = key_ptr;
    const FN_LENGTH: unsafe extern "C" fn(*const marisa_Key, *mut *const exception_record) -> usize = key_length;
    const FN_ID: unsafe extern "C" fn(*const marisa_Key, *mut *const exception_record) -> usize = key_id;
    const FN_SET_STR: unsafe extern "C" fn(*mut marisa_Key, *const c_char, length: usize, *mut *const exception_record) = key_set_str;
    const FN_SET_ID: unsafe extern "C" fn(*mut marisa_Key, id: usize, *mut *const exception_record) = key_set_id;
}


impl KeyQueryRawFuncs<marisa_Query> for marisa_Query {
    const FN_PTR: unsafe extern "C" fn(*const marisa_Query, *mut *const exception_record) -> *const utils::cuchar = query_ptr;
    const FN_LENGTH: unsafe extern "C" fn(*const marisa_Query, *mut *const exception_record) -> usize = query_length;
    const FN_ID: unsafe extern "C" fn(*const marisa_Query, *mut *const exception_record) -> usize = query_id;
    const FN_SET_STR: unsafe extern "C" fn(*mut marisa_Query, *const c_char, length: usize, *mut *const exception_record) = query_set_str;
    const FN_SET_ID: unsafe extern "C" fn(*mut marisa_Query, id: usize, *mut *const exception_record) = query_set_id;
}


/*
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
*/
/*
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
*/
/*
impl std::ops::Index<usize> for RawObjectMutPtr<marisa_Keyset> {
}
*/

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
