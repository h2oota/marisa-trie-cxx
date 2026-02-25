#![allow(unused_imports)]


mod raw_object;
mod object;
mod error;

pub use crate::ffi::{
    MARISA_MIN_NUM_TRIES,
    MARISA_MAX_NUM_TRIES,
    MARISA_DEFAULT_NUM_TRIES,
    MARISA_HUGE_CACHE,
    MARISA_LARGE_CACHE,
    MARISA_NORMAL_CACHE,
    MARISA_SMALL_CACHE,
    MARISA_TINY_CACHE,
    MARISA_DEFAULT_CACHE,
    MARISA_TEXT_TAIL,
    MARISA_BINARY_TAIL,
    MARISA_DEFAULT_TAIL,
    MARISA_LABEL_ORDER,
    MARISA_WEIGHT_ORDER,
    MARISA_DEFAULT_ORDER,
    MARISA_NUM_TRIES_MASK,
    MARISA_CACHE_LEVEL_MASK,
    MARISA_TAIL_MODE_MASK,
    MARISA_NODE_ORDER_MASK,
    MARISA_CONFIG_MASK,
    TailMode,
    NodeOrder,
};

use error::*;


#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::marisa::marisa_wrapper::*;
    use crate::marisa::object::*;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    pub struct RandomBinary<'a, T> {
	rng: &'a mut T
    }

    impl<T: Rng> Iterator for RandomBinary<'_, T> {
	type Item = Box<[u8]>;

	fn next(&mut self) -> Option<Self::Item> {
	    let l = self.rng.gen::<u32>() % (marisa_Keyset_EXTRA_BLOCK_SIZE * 2);
	    let mut b: Box<[u8]> = vec![0; l as usize].into_boxed_slice();
	    for i in 0 .. b.len() {
		b[i] = self.rng.gen::<u8>() & 0xff;
	    }
	    Some(b)
	}
    }

    impl<'a, T> RandomBinary<'a, T> {
	pub fn new(rng: &'a mut T) -> Self {
	    Self {
		rng: rng
	    }
	}
    }

    pub fn make_keyset(num_keys: usize,
		       tail_mode: marisa_TailMode,
		       keyset: &mut KeysetObject) -> Result<(), MarisaError>  {
	let mut key_buf: [u8; 16] = [0; 16];
	let mut rng = rand::thread_rng(); // StdRng::seed_from_u64(53885746); //
	for _i in 0 .. num_keys {
	    let length = rng.gen::<usize>() % key_buf.len();
	    for j in 0 .. length {
		key_buf[j] = (rng.gen::<u32>() % 10) as u8;
		if tail_mode == MARISA_TEXT_TAIL {
		    key_buf[j] += 32; // '0'
		}
	    }
	    keyset.push_back_bin_weight(&key_buf[0 .. length], 1.0)?
	}

	Ok(())
    }
}

#[cfg(test)]
mod basic_tests {
    use super::*;
    use super::test_helper::*;
    use crate::marisa::marisa_wrapper::*;

    use rand::Rng;

    #[test]
    fn test_key() -> Result<(), MarisaError> {
	let str = "apple";
	let mut key = KeyObject::new();

	assert!(key.str().is_ok());
	assert_eq!(0, key.length()?);

	assert!(key.set_str(&str).is_ok());
	assert_eq!(str.len(), key.length().unwrap());

	assert!(key.set_weight(1.0).is_ok());

	let weigth = key.weight()?;
	assert!((weigth - 1.0).abs() < 1e-6, "{} != 1.0", weigth);
	Ok(())
    }


    const KEYS_LEN: usize = 1000;

    #[test]
    fn test_keyset() -> Result<(), MarisaError> {
	let mut keyset = KeysetObject::new();
	let keys: Vec<&str> = vec!["apple", "orange", "banana"];
	let mut total_length: usize = 0;


	for (index, value) in keys.iter().enumerate() {
	    assert!(keyset.push_back_str(value).is_ok());
	    assert_eq!(keyset.size()?, index + 1);
	    assert!(!keyset.empty()?);
	    let length = value.len();

	    total_length += length;

	    assert_eq!(keyset.total_length()?, total_length);

	    assert_eq!(keyset.get(index)?.length()?, length);

	    assert_eq!(keyset.get(index)?.str()?, *value);

	    let value = keyset.get(index)?.weight()?;

	    assert!((value - 1.0).abs() < 1e-6, "{} != 1.0", value);
	}
	assert!(keyset.clear().is_ok());

	let mut key = KeyObject::new();
	assert!(key.set_str("123").is_ok());

	assert!(keyset.push_back_key(&key).is_ok());
	assert_eq!(3, keyset.get(0)?.length()?);

	assert!(key.set_str("456").is_ok());
	assert!(keyset.push_back_key_em(&key, '\0').is_ok());
	assert_eq!(3, keyset.get(1)?.length()?);
	assert_eq!("456", keyset.get(1)?.str()?);

	assert!(key.set_str("789").is_ok());
	assert!(keyset.push_back_key_em(&key, '0').is_ok());
	assert_eq!(3, keyset.get(2)?.length()?);
	// assert_eq!(b"7890", keyset.get(2)?.bin()?, "{} {}", keyset.get(2)?.length()?, 4);

	assert!(key.set_str("").is_ok());
	assert!(keyset.push_back_key(&key).is_ok());
	assert_eq!(0, keyset.get(3)?.length()?);
	assert_eq!("", keyset.get(3)?.str()?);


	assert!(keyset.clear().is_ok());


	let mut rng = rand::thread_rng(); // StdRng::seed_from_u64(53885746); //
	let mut weights: [f32; KEYS_LEN] = [0.0; KEYS_LEN];

	let keys = {
	    let mut iter = RandomBinary::new(&mut rng).take(weights.len());
	    let arr: [Box<[u8]>; KEYS_LEN] =
		std::array::from_fn(|_| iter.next().unwrap());
	    arr
	};
	let mut total_length = 0;

	for i in 0 .. weights.len() {
	    weights[i] = rng.gen::<f32>() * 100.0;
	    assert!(keyset.push_back_bin_weight(&keys[i], weights[i]).is_ok());
	    total_length += keys[i].len();
	    assert_eq!(total_length, keyset.total_length()?, "at {}, {} {:?}", i, keyset.get(i)?.length()?, keys[i].len());
	}

	assert_eq!(keyset.size()?, keys.len());

	for i in 0 .. weights.len() {
	    assert_eq!(keys[i].len(), keyset.get(i)?.length()?);
	    assert_eq!(&*keys[i], keyset.get(i)?.bin()?, "at {} {} {}", i, keys[i].len(), keyset.get(i)?.length()?);
	    assert_eq!(weights[i], keyset.get(i)?.weight()?);
	}

	Ok(())
    }

    #[test]
    fn test_keyset_payload() -> Result<(), MarisaError> {
	let mut keyset = KeysetObject::new();
	let keys: Vec<&str> = vec!["apple\0data1", "orange\0data2", "banana\0data3"];
	let mut total_length: usize = 0;

	for (index, value) in keys.iter().enumerate() {
	    assert!(keyset.push_back_str(value).is_ok());
	    assert_eq!(keyset.size()?, index + 1);
	    assert!(!keyset.empty()?);
	    let length = value.len();

	    total_length += length;

	    assert_eq!(keyset.total_length()?, total_length);

	    assert_eq!(keyset.get(index)?.length()?, length);

	    assert_eq!(keyset.get(index)?.str()?, *value);

	    let value = keyset.get(index)?.weight()?;

	    assert!((value - 1.0).abs() < 1e-6, "{} != 1.0", value);
	}
	Ok(())
    }

    #[test]
    fn test_query() -> Result<(), MarisaError> {
	let mut query = QueryObject::new();
	assert!(query.ptr()?.is_null());
	assert_eq!(0, query.length()?);
	assert_eq!(0, query.id()?);

	let str = "apple";
	assert!(query.set_str(&str).is_ok());
	assert_eq!(str.len(), query.length()?);

	assert!(query.set_str_length(&str, 3).is_ok());
	assert_eq!(3, query.length()?);

	assert!(query.set_id(100).is_ok());
	assert_eq!(100, query.id()?);

	assert!(query.clear().is_ok());
	assert!(query.ptr()?.is_null());
	assert_eq!(0, query.length()?);
	assert_eq!(0, query.id()?);

	Ok(())
    }

    #[test]
    fn test_agent() -> Result<(), MarisaError> {
	let mut agent = AgentObject::new();
	assert!(agent.query()?.ptr()?.is_null());
	assert_eq!(0, agent.query()?.length()?);
	assert_eq!(0, agent.query()?.id()?);

	assert!(!agent.has_state()?);

	let query_str = "query";
	let key_str = "key";

	assert!(agent.set_query_str(&query_str).is_ok());
	assert!(agent.set_query_id(123).is_ok());
	assert!(agent.set_key_str(&key_str).is_ok());
	assert!(agent.set_key_id(234).is_ok());

	assert!(agent.init_state().is_ok());

	assert!(agent.has_state()?);

	let result = agent.init_state();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("state_ != nullptr"));
	}

	assert!(agent.clear().is_ok());

	assert!(agent.query()?.ptr()?.is_null());
	assert_eq!(0, agent.query()?.length()?);
	assert_eq!(0, agent.query()?.id()?);

	assert!(agent.key()?.ptr()?.is_null());
	assert_eq!(0, agent.key()?.length()?);

	assert!(!agent.has_state()?);

	Ok(())
    }
}

#[cfg(test)]
mod marisa_tests {
    use crate::marisa::*;
    use crate::marisa::marisa_wrapper::*;
    use super::test_helper::*;

    #[test]
    fn empty_trie() -> Result<(), MarisaError> {
	let mut trie = TrieObject::new();

	let result = trie.save("marisa-test.dat");
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let mut agent = AgentObject::new();

	let result = trie.lookup(&mut agent);
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.reverse_lookup(&mut agent);
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    println!("{:?}", err.message);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.common_prefix_search(&mut agent);
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.predictive_search(&mut agent);
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.num_tries();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.num_keys();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.num_nodes();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.tail_mode();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.node_order();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.empty();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.size();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.total_size();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let result = trie.io_size();
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::logic_error", err.source);
	    assert!(err.message.contains("trie_ == nullptr"));
	}

	let mut keyset = KeysetObject::new();
	assert!(trie.build(&mut keyset, 0).is_ok());

	assert!(!trie.lookup(&mut agent)?);

	let result = trie.reverse_lookup(&mut agent);
	assert!(result.is_err());
	if let Err(err) = result {
	    assert_eq!("std::out_of_range", err.source);
	    assert!(err.message.contains("query().id() >= size()"));
	}

	assert!(!trie.common_prefix_search(&mut agent)?);
	assert!(!trie.predictive_search(&mut agent)?);

	assert_eq!(1, trie.num_tries()?);
	assert_eq!(0, trie.num_keys()?);
	assert_eq!(1, trie.num_nodes()?);

	assert_eq!(MARISA_DEFAULT_TAIL, trie.tail_mode()?);
	assert_eq!(MARISA_DEFAULT_ORDER, trie.node_order()?);

	assert!(trie.empty()?);
	assert_eq!(0, trie.size()?);
	assert_ne!(0, trie.total_size()?);
	assert_ne!(0, trie.io_size()?);


	assert!(keyset.push_back_str("").is_ok());
	assert!(trie.build(&mut keyset, 0).is_ok());

	assert!(trie.lookup(&mut agent)?);
	assert!(trie.reverse_lookup(&mut agent).is_ok());

	assert!(trie.common_prefix_search(&mut agent)?);
	assert!(!trie.common_prefix_search(&mut agent)?);
	assert!(trie.predictive_search(&mut agent)?);
	assert!(!trie.predictive_search(&mut agent)?);

	assert_eq!(1, trie.num_keys()?);
	assert_eq!(1, trie.num_nodes()?);

	assert!(!trie.empty()?);
	assert_eq!(1, trie.size()?);
	assert_ne!(0, trie.total_size()?);
	assert_ne!(0, trie.io_size()?);


	Ok(())
    }

    #[test]
    fn tiny_trie() -> Result<(), MarisaError> {
	let mut keyset = KeysetObject::new();

	assert!(keyset.push_back_str("bach").is_ok());
	assert!(keyset.push_back_str("bet").is_ok());
	assert!(keyset.push_back_str("chat").is_ok());
	assert!(keyset.push_back_str("check").is_ok());
	assert!(keyset.push_back_str("check").is_ok());

	let mut trie = TrieObject::new();
	assert!(trie.build(&mut keyset, 1).is_ok());

	assert_eq!(1, trie.num_tries()?);
	assert_eq!(4, trie.num_keys()?);
	assert_eq!(7, trie.num_nodes()?);

	assert_eq!(MARISA_DEFAULT_TAIL, trie.tail_mode()?);
	assert_eq!(MARISA_DEFAULT_ORDER, trie.node_order()?);

	assert_eq!(2, keyset.get(0)?.id()?);
	assert_eq!(3, keyset.get(1)?.id()?);
	assert_eq!(1, keyset.get(2)?.id()?);
	assert_eq!(0, keyset.get(3)?.id()?);
	assert_eq!(0, keyset.get(4)?.id()?);

	let mut agent = AgentObject::new();

	for i in 0 .. keyset.size()? {
	    let key = keyset.get(i)?;
	    let s = key.str()?;

	    assert!(agent.set_query_str(s).is_ok());
	    assert!(trie.lookup(&mut agent)?);
	    assert_eq!(keyset.get(i)?.id()?, agent.key()?.id()?);

	    assert!(agent.set_query_id(keyset.get(i)?.id()?).is_ok());

	    assert!(trie.reverse_lookup(&mut agent).is_ok());
	    assert_eq!(keyset.get(i)?.length()?, agent.key()?.length()?);
	    assert_eq!(keyset.get(i)?.str()?, agent.key()?.str()?);
	}

	let mut agent = AgentObject::new();
	assert!(agent.set_query_str("be").is_ok());
	assert!(!trie.common_prefix_search(&mut agent)?);
	assert!(agent.set_query_str("beX").is_ok());
	assert!(!trie.common_prefix_search(&mut agent)?);
	assert!(agent.set_query_str("bet").is_ok());
	assert!(trie.common_prefix_search(&mut agent)?);
	assert!(!trie.common_prefix_search(&mut agent)?);
	assert!(agent.set_query_str("betX").is_ok());
	assert!(trie.common_prefix_search(&mut agent)?);
	assert!(!trie.common_prefix_search(&mut agent)?);

	assert!(agent.set_query_str("chatX").is_ok());
	assert!(!trie.predictive_search(&mut agent)?);
	assert!(agent.set_query_str("chat").is_ok());
	assert!(trie.predictive_search(&mut agent)?);
	assert_eq!(4, agent.key()?.length()?);
	assert!(!trie.predictive_search(&mut agent)?);

	assert!(agent.set_query_str("cha").is_ok());
	assert!(trie.predictive_search(&mut agent)?);
	assert_eq!(4, agent.key()?.length()?);
	assert!(!trie.predictive_search(&mut agent)?);

	assert!(agent.set_query_str("c").is_ok());
	assert!(trie.predictive_search(&mut agent)?);
	assert_eq!(5, agent.key()?.length()?);
	assert_eq!("check", agent.key()?.str()?);
	assert!(trie.predictive_search(&mut agent)?);
	assert_eq!(4, agent.key()?.length()?);
	assert_eq!("chat", agent.key()?.str()?);
	assert!(!trie.predictive_search(&mut agent)?);


	assert!(agent.set_query_str("ch").is_ok());
	assert!(trie.predictive_search(&mut agent)?);
	assert_eq!(5, agent.key()?.length()?);
	assert_eq!("check", agent.key()?.str()?);
	assert!(trie.predictive_search(&mut agent)?);
	assert_eq!(4, agent.key()?.length()?);
	assert_eq!("chat", agent.key()?.str()?);
	assert!(!trie.predictive_search(&mut agent)?);

	assert!(trie.build(&mut keyset, 1 | MARISA_LABEL_ORDER).is_ok());

	assert_eq!(1, trie.num_tries()?);
	assert_eq!(4, trie.num_keys()?);
	assert_eq!(7, trie.num_nodes()?);

	assert_eq!(MARISA_DEFAULT_TAIL, trie.tail_mode()?);
	assert_eq!(MARISA_LABEL_ORDER, trie.node_order()?);

	assert_eq!(0, keyset.get(0)?.id()?);
	assert_eq!(1, keyset.get(1)?.id()?);
	assert_eq!(2, keyset.get(2)?.id()?);
	assert_eq!(3, keyset.get(3)?.id()?);
	assert_eq!(3, keyset.get(4)?.id()?);

	let mut agent = AgentObject::new();
	for i in 0 .. keyset.size()? {
	    assert!(agent.set_query_str(keyset.get(i)?.str()?).is_ok());
	    assert!(trie.lookup(&mut agent)?);
	    assert_eq!(keyset.get(i)?.id()?, agent.key()?.id()?);

	    assert!(agent.set_query_id(keyset.get(i)?.id()?).is_ok());
	    assert!(trie.reverse_lookup(&mut agent).is_ok());
	    assert_eq!(keyset.get(i)?.length()?, agent.key()?.length()?);
	    assert_eq!(keyset.get(i)?.str()?, agent.key()?.str()?);
	}

	assert!(agent.set_query_str("").is_ok());
	for i in 0 .. trie.size()? {
	    assert!(trie.predictive_search(&mut agent)?);
	    assert_eq!(i, agent.key()?.id()?);
	}
	assert!(!trie.predictive_search(&mut agent)?);

	Ok(())
    }

    fn lookup(trie: &TrieObject, keyset: &KeysetObject) -> Result<(), MarisaError> {
	let mut agent = AgentObject::new();
	for i in 0 .. keyset.size()? {
	    assert!(agent.set_query_str(keyset.get(i)?.str()?).is_ok());
	    assert!(trie.lookup(&mut agent)?);
	    assert_eq!(keyset.get(i)?.id()?, agent.key()?.id()?);

	    assert!(agent.set_query_id(keyset.get(i)?.id()?).is_ok());
	    assert!(trie.reverse_lookup(&mut agent).is_ok());
	    assert_eq!(keyset.get(i)?.length()?, agent.key()?.length()?);
	    assert_eq!(keyset.get(i)?.str()?, agent.key()?.str()?);
	}
	Ok(())
    }

    fn common_prefix_search(trie: &TrieObject, keyset: &KeysetObject) -> Result<(), MarisaError> {
	let mut agent = AgentObject::new();
	for i in 0 .. keyset.size()? {
	    assert!(agent.set_query_str(keyset.get(i)?.str()?).is_ok());
	    assert!(trie.common_prefix_search(&mut agent)?);
	    assert!(keyset.get(i)?.id()? >= agent.key()?.id()?);
	    while trie.common_prefix_search(&mut agent)? {
		assert!(keyset.get(i)?.id()? >= agent.key()?.id()?);
	    }
	    assert_eq!(keyset.get(i)?.id()?, agent.key()?.id()?);
	}
	Ok(())
    }

    fn test_trie3(num_tries: usize,
		  tail_mode: marisa_tail_mode,
		  node_order: marisa_node_order,
		  keyset: &mut KeysetObject) -> Result<(), MarisaError> {
	let mut trie = TrieObject::new();

	assert!(trie.build(keyset, num_tries as u32 | tail_mode | node_order).is_ok());

	assert_eq!(num_tries, trie.num_tries()?);
	assert!(keyset.size()? >= trie.num_keys()?);

	assert_eq!(tail_mode, trie.tail_mode()?);
	assert_eq!(node_order, trie.node_order()?);

	assert!(lookup(&mut trie, keyset).is_ok());

	assert!(common_prefix_search(&mut trie, keyset).is_ok());

	assert_eq!(num_tries, trie.num_tries()?);
	assert!(keyset.size()? >= trie.num_keys()?);

	assert_eq!(tail_mode, trie.tail_mode()?);
	assert_eq!(node_order, trie.node_order()?);

	assert!(lookup(&mut trie, keyset).is_ok());

	assert!(trie.clear().is_ok());

	Ok(())
    }

    fn test_trie2(tail_mode: marisa_tail_mode,
		  node_order: marisa_node_order,
		  keyset: &mut KeysetObject) -> Result<(), MarisaError> {

	println!("{}, {}",
		 if tail_mode == MARISA_TEXT_TAIL {"TEXT"} else {"BINARY"},
		 if node_order == MARISA_WEIGHT_ORDER {"WEIGHT"} else {"LABEL"});

	for i in 1 .. 5 {
	    test_trie3(i, tail_mode, node_order, keyset)?;
	}
	Ok(())
    }

    fn test_trie1(tail_mode: marisa_tail_mode) -> Result<(), MarisaError> {
	let mut keyset = KeysetObject::new();

	assert!(make_keyset(1000, tail_mode, &mut keyset).is_ok());
	assert!(test_trie2(tail_mode, MARISA_WEIGHT_ORDER, &mut keyset).is_ok());

	assert!(keyset.reset().is_ok());
	assert!(make_keyset(1000, tail_mode, &mut keyset).is_ok());
	assert!(test_trie2(tail_mode, MARISA_LABEL_ORDER, &mut keyset).is_ok());

	Ok(())
    }

    #[test]
    fn test_trie() -> Result<(), MarisaError> {
	test_trie1(MARISA_TEXT_TAIL)?;
	test_trie1(MARISA_BINARY_TAIL)?;
	Ok(())
    }
}

/*
#[cfg(test)]
mod tests_thread_safety {
    use crate::marisa::*;

    use static_assertions::assert_impl_all;

    #[test]
    fn test_library()
    {
	assert_impl_all!(TrieObject: Send, Sync);
	assert_impl_all!(AgentObject: Send, Sync);
	assert_impl_all!(KeysetObject: Send, Sync);
	assert_impl_all!(QueryObject: Send, Sync);
	assert_impl_all!(KeyObject: Send, Sync);
    }
}
*/
