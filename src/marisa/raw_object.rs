
#![allow(dead_code)]

use std::os::raw::c_char;
use std::ptr;
use std::marker::PhantomData;

use super::error::*;
use crate::utils;

use crate::ffi::*;
use crate::ffi::marisa_trie::{
    marisa_NodeOrder,
    marisa_TailMode,

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

pub mod api {
    use super::*;

    pub trait LifeTrait
    {
	fn new() -> Self;
	fn destroy(&mut self);
    }

    pub trait KeyTrait
    {
	fn weight(&self) -> Result<f32, MarisaError> ;
    }

//    pub trait KeyQueryTrait<T>: KeyQueryRawFuncs<T>
    pub trait KeyQueryTrait
    {
	fn get_ptr_length(&self) -> Result<(*const utils::cuchar, usize), MarisaError>
	{
	    Err(MarisaError::new("KeyQueryTrait::get_ptr_length", "default impl"))
	}
	fn str(&self) -> Result<&str, MarisaError>;
	fn bin(&self) -> Result<&[u8], MarisaError>;
	fn ptr(&self) -> Result<*const u8, MarisaError>;
	fn id(&self) -> Result<usize, MarisaError>;
	fn length(&self) -> Result<usize, MarisaError>;
    }


    pub trait KeyMutTrait
    {
	fn set_weight(&mut self, weight: f32) -> Result<(), MarisaError> ;
    }

//    pub trait KeyQueryMutTrait<T>: KeyQueryRawFuncs<T>
    pub trait KeyQueryMutTrait
    {
	fn set_str(&mut self, str: &str) -> Result<(), MarisaError>;
	fn set_str_length(&mut self, str: &str, length: usize) -> Result<(), MarisaError>;
	fn set_id(&mut self, id: usize) -> Result<(), MarisaError>;
    }


    pub trait KeysetTrait {
	fn num_keys(&self) -> Result<usize, MarisaError>;
	fn empty(&self) -> Result<bool, MarisaError>;
	fn size(&self) -> Result<usize, MarisaError>;
	fn total_length(&self) -> Result<usize, MarisaError>;
    }

    pub trait KeysetMutTrait<T>
    {
	fn push_back_key(&mut self, key: &T) -> Result<(), MarisaError>;
	fn push_back_key_em(&mut self, key: &T, end_marker: char) -> Result<(), MarisaError>;
	fn push_back_str(&mut self, key: &str) -> Result<(), MarisaError>;
	fn push_back_str_weight(&mut self, key: &str, weight: utils::cfloat) -> Result<(), MarisaError>;
	fn push_back_bin_weight(&mut self, key: &[u8], weight: utils::cfloat) -> Result<(), MarisaError>;
	fn reset(&mut self) -> Result<(), MarisaError>;
	fn clear(&mut self) -> Result<(), MarisaError>;
	fn xswap(&mut self, rhs: Self) -> Result<(), MarisaError>;
    }

    pub trait AgentTrait<K, Q>
    {
	fn key(&self) -> Result<K, MarisaError>;
	fn query(&self) -> Result<Q, MarisaError>;
	fn has_state(&self) -> Result<bool, MarisaError>;
    }


    pub trait AgentMutTrait
    {
	fn set_query_str(&mut self, s: &str) -> Result<() , MarisaError>;
	fn set_query_str_len(&mut self, s: &str, l: usize) -> Result<(), MarisaError>;
	fn set_query_id(&mut self, id: usize) -> Result<(), MarisaError>;
	fn set_key_str(&mut self, s: &str) -> Result<(), MarisaError>;
	fn set_key_str_length(&mut self, s: &str, l: usize) -> Result<(), MarisaError>;
	fn set_key_id(&mut self, id: usize) -> Result<(), MarisaError>;
	fn init_state(&mut self) -> Result<(), MarisaError>;
	fn clear(&mut self) -> Result<(), MarisaError>;
	fn swap(&mut self, rhs: Self) -> Result<(), MarisaError>;
    }


    pub trait TrieTrait
    {
	fn save(&self, filename: &str) -> Result<(), MarisaError>;
	fn write(&self, fd: utils::cint) -> Result<(), MarisaError>;
	fn lookup(&self, agent: &mut Agent) -> Result<bool, MarisaError>;
	fn reverse_lookup(&self, agent: &mut Agent) -> Result<(), MarisaError>;
	fn common_prefix_search(&self, agent: &mut Agent) -> Result<bool, MarisaError>;
	fn predictive_search(&self, agent: &mut Agent) -> Result<bool, MarisaError>;
	fn num_tries(&self) -> Result<usize, MarisaError>;
	fn num_keys(&self) -> Result<usize, MarisaError>;
	fn num_nodes(&self) -> Result<usize, MarisaError>;
	fn tail_mode(&self) -> Result<marisa_TailMode, MarisaError>;
	fn node_order(&self) -> Result<marisa_NodeOrder, MarisaError>;
	fn empty(&self) -> Result<bool, MarisaError>;
	fn size(&self) -> Result<usize, MarisaError>;
	fn total_size(&self) -> Result<usize, MarisaError>;
	fn io_size(&self) -> Result<usize, MarisaError>;
    }

    pub trait TrieMutTrait
    {
	fn build(&mut self, keyset: &mut Keyset, config_flags: u32) -> Result<(), MarisaError>;
	fn mmap(&mut self, filename: &str) -> Result<(), MarisaError>;
	fn map(&mut self, ptr: &[u8]) -> Result<(), MarisaError>;
	fn load(&mut self, filename: &str) -> Result<(), MarisaError>;
	fn read(&mut self, fd: utils::cint) -> Result<(), MarisaError>;
	fn clear(&mut self) -> Result<(), MarisaError>;
    }

    pub type Key = *mut marisa_Key;
    pub type Query = *mut marisa_Query;
    pub type Keyset = *mut marisa_Keyset;
    pub type Agent = *mut marisa_Agent;
    pub type Trie = *mut marisa_Trie;

    pub type ConstKey = *const marisa_Key;
    pub type ConstQuery = *const marisa_Query;
/*
    pub type ConstKeyset = *const marisa_Keyset;
    pub type ConstAgent = *const marisa_Agent;
    pub type ConstTrie = *const marisa_Trie;
*/
}

use api::*;

/*
pub trait LifeTrait<T>
{
    fn new() -> Self;
    fn destroy(&mut self);
}
*/

trait KeyQueryRawFuncs<T> {
    const FN_PTR: unsafe extern "C" fn(*const T, *mut *const exception_record) -> *const utils::cuchar;
    const FN_LENGTH: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize;
    const FN_ID: unsafe extern "C" fn(*const T, *mut *const exception_record) -> usize;
    const FN_SET_STR: unsafe extern "C" fn(*mut T, *const c_char, length: usize, *mut *const exception_record);
    const FN_SET_ID: unsafe extern "C" fn(*mut T, id: usize, *mut *const exception_record);
}



impl LifeTrait for *mut marisa_Key
{
    fn new() -> Self {
	unsafe { key_create() }
    }

    fn destroy(&mut self)
    {
	unsafe { key_destroy(*self) };
    }
}


impl LifeTrait for *mut marisa_Query
{
    fn new() -> Self {
	unsafe { query_create() }
    }

    fn destroy(&mut self)
    {
	unsafe { query_destroy(*self) };
    }
}


impl LifeTrait for *mut marisa_Keyset
{
    fn new() -> Self {
	unsafe { keyset_create() }
    }

    fn destroy(&mut self)
    {
	unsafe { keyset_destroy(*self) };
    }
}


impl LifeTrait for *mut marisa_Agent
{
    fn new() -> Self {
	unsafe { agent_create() }
    }

    fn destroy(&mut self)
    {
	unsafe { agent_destroy(*self) };
    }
}


impl LifeTrait for *mut marisa_Trie
{
    fn new() -> Self {
	unsafe { trie_create() }
    }

    fn destroy(&mut self)
    {
	unsafe { trie_destroy(*self) };
    }
}


impl KeyTrait for *mut marisa_Key
{
    fn weight(&self) -> Result<f32, MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	let weight = unsafe { key_weight(*self, &mut err_record) };
	if !err_record.is_null() {
	    return Err(MarisaException::from_exception(err_record));
	}
	Ok(weight)
    }
}


impl KeyMutTrait for *mut marisa_Key
{
    fn set_weight(&mut self, weight: f32) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { key_set_weight(*self, weight, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

impl KeyQueryRawFuncs<marisa_Key> for *mut marisa_Key {
    const FN_PTR: unsafe extern "C" fn(*const marisa_Key, *mut *const exception_record) -> *const utils::cuchar = key_ptr;
    const FN_LENGTH: unsafe extern "C" fn(*const marisa_Key, *mut *const exception_record) -> usize = key_length;
    const FN_ID: unsafe extern "C" fn(*const marisa_Key, *mut *const exception_record) -> usize = key_id;
    const FN_SET_STR: unsafe extern "C" fn(*mut marisa_Key, *const c_char, length: usize, *mut *const exception_record) = key_set_str;
    const FN_SET_ID: unsafe extern "C" fn(*mut marisa_Key, id: usize, *mut *const exception_record) = key_set_id;
}


impl KeyQueryRawFuncs<marisa_Query> for *mut marisa_Query {
    const FN_PTR: unsafe extern "C" fn(*const marisa_Query, *mut *const exception_record) -> *const utils::cuchar = query_ptr;
    const FN_LENGTH: unsafe extern "C" fn(*const marisa_Query, *mut *const exception_record) -> usize = query_length;
    const FN_ID: unsafe extern "C" fn(*const marisa_Query, *mut *const exception_record) -> usize = query_id;
    const FN_SET_STR: unsafe extern "C" fn(*mut marisa_Query, *const c_char, length: usize, *mut *const exception_record) = query_set_str;
    const FN_SET_ID: unsafe extern "C" fn(*mut marisa_Query, id: usize, *mut *const exception_record) = query_set_id;
}


impl<T> KeyQueryTrait
    for *mut T where *mut T: KeyQueryRawFuncs<T>
{
    fn get_ptr_length(&self) ->	Result<(*const utils::cuchar, usize), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	let ptr = {
	    let ptr = unsafe { (Self::FN_PTR)(*self, &mut err_record) };
	    if !err_record.is_null() {
		return Err(MarisaException::from_exception(err_record));
	    }
	    ptr
	};
	let length = {
	    let length = unsafe { (Self::FN_LENGTH)(*self, &mut err_record) };
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
	       .map_err(|_e| MarisaError::new("KeyQueryTrait::str", "get_ptr_length returns NULL"))
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
	let value = unsafe { (Self::FN_PTR)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value as *const u8)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn id(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FN_ID)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn length(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (Self::FN_LENGTH)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}


impl<T> KeyQueryMutTrait
    for *mut T where *mut T: KeyQueryRawFuncs<T>
{

//    #[cfg(not(release))]
    fn set_str(&mut self, str: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FN_SET_STR)(*self, str.as_ptr() as *const c_char, str.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_str_length(&mut self, str: &str, length: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FN_SET_STR)(*self, str.as_ptr() as *const c_char, length, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

//    #[cfg(not(release))]
    fn set_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (Self::FN_SET_ID)(*self, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}


impl KeysetTrait for *mut marisa_Keyset
{
    fn num_keys(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_num_keys(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn empty(&self) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_empty(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn size(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_size(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn total_length(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { keyset_total_length(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}


impl KeysetMutTrait<ConstKey> for *mut marisa_Keyset
{
    fn push_back_key(&mut self, key: &ConstKey) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_0)(*self, *key, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn push_back_key_em(&mut self, key: &ConstKey, end_marker: char) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_1)(*self, *key, end_marker as utils::cuchar, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    // Rust strings are not null-terminated, so
    // void push_back(marisa::Keyset*, const char *);
    // wrapper will not be implemented
    fn push_back_str(&mut self, key: &str) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_3)(*self, key.as_ptr(), key.len(), &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn push_back_str_weight(&mut self, key: &str, weight: utils::cfloat) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_4)(*self, key.as_ptr(), key.len(), weight, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn push_back_bin_weight(&mut self, key: &[u8], weight: utils::cfloat) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe {(keyset_push_back_4)(*self, key.as_ptr(), key.len(), weight, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn reset(&mut self) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_reset)(*self, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn clear(&mut self) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_clear)(*self, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn xswap(&mut self, rhs: Self) -> Result<(), MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (keyset_swap)(*self, rhs, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

/*
impl std::ops::Index<usize> for RawObjectMutPtr<marisa_Keyset> {
}
*/

//pub trait KeysetTrait0: RawObjectPtr<marisa_Keyset> {
//pub trait KeysetTrait1: RawObjectMutPtr<marisa_Keyset> {


impl AgentTrait<ConstKey, ConstQuery> for *mut marisa_Agent
{
    fn key(&self) -> Result<ConstKey, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	if err_record.is_null() {
	    Ok(unsafe { (agent_key)(*self, &mut err_record) })
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn query(&self) -> Result<ConstQuery, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	if err_record.is_null() {
	    Ok(unsafe { (agent_query)(*self, &mut err_record) })
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn has_state(&self) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (agent_has_state)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

impl AgentMutTrait for *mut marisa_Agent
{
    fn set_query_str(&mut self, s: &str) -> Result<() , MarisaError>
    {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { agent_set_query_1(*self, s.as_ptr(), s.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn set_query_str_len(&mut self, s: &str, l: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { agent_set_query_1(*self, s.as_ptr(), l, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn set_query_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_query_2)(*self, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn set_key_str(&mut self, s: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_key_1)(*self, s.as_ptr(), s.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn set_key_str_length(&mut self, s: &str, l: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_key_1)(*self, s.as_ptr(), l, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn set_key_id(&mut self, id: usize) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_set_key_2)(*self, id, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn init_state(&mut self) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_init_state)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }


    fn clear(&mut self) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_clear)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn swap(&mut self, rhs: Self) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (agent_swap)(*self, rhs, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

impl TrieTrait for *mut marisa_Trie
{
    fn save(&self, filename: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_save)(*self, filename.as_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn write(&self, fd: utils::cint) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_write)(*self, fd, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn lookup(&self, agent: &mut Agent) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_lookup)(*self,  *agent, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn reverse_lookup(&self, agent: &mut Agent) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_reverse_lookup)(*self, *agent, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn common_prefix_search(&self, agent: &mut Agent) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_common_prefix_search)(*self, *agent, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn predictive_search(&self, agent: &mut Agent) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_predictive_search)(*self, *agent, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn num_tries(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_num_tries)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn num_keys(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_num_keys)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn num_nodes(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_num_nodes)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn tail_mode(&self) -> Result<marisa_TailMode, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_tail_mode)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn node_order(&self) -> Result<marisa_NodeOrder, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value= unsafe { (trie_node_order)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn empty(&self) -> Result<bool, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_empty)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn size(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_size)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn total_size(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_total_size)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn io_size(&self) -> Result<usize, MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	let value = unsafe { (trie_io_size)(*self, &mut err_record) };
	if err_record.is_null() {
	    Ok(value)
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}

impl TrieMutTrait for *mut marisa_Trie
{
    fn build(&mut self, keyset: &mut Keyset, config_flags: u32) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_build)(*self, *keyset, config_flags as utils::cint, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn mmap(&mut self, filename: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_mmap)(*self, filename.as_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn map(&mut self, ptr: &[u8]) -> Result<(), MarisaError>  {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_map)(*self, ptr.as_ptr() as *const std::os::raw::c_void, ptr.len(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn load(&mut self, filename: &str) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_load)(*self, filename.as_ptr(), &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn read(&mut self, fd: utils::cint) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_read)(*self, fd, &mut err_record) };
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }

    fn clear(&mut self) -> Result<(), MarisaError> {
	let mut err_record: *const exception_record = ptr::null();
	unsafe { (trie_clear)(*self, &mut err_record) }
	if err_record.is_null() {
	    Ok(())
	} else {
	    Err(MarisaException::from_exception(err_record))
	}
    }
}
