#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"


marisa_Trie* trie_create()
{
     return reinterpret_cast<marisa_Trie*>(new marisa::Trie());
}


void trie_destroy(marisa_Trie* trie)
{
     delete reinterpret_cast<marisa::Trie*>(trie);
}


void trie_build(marisa_Trie* trie, marisa_Keyset& keyset , int config_flag, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->build(
	       reinterpret_cast<marisa::Keyset&>(keyset), config_flag);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_mmap(marisa_Trie* trie, const unsigned char *filename, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->mmap(reinterpret_cast<const char *>(filename));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_map(marisa_Trie* trie, const void *ptr, std::size_t size, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->map(ptr, size);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_load(marisa_Trie* trie, const unsigned char* filename, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->load(reinterpret_cast<const char *>(filename));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_read(marisa_Trie* trie, const int fd, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->read(fd);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


void trie_save(const marisa_Trie* trie, const unsigned char* filename, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<const marisa::Trie*>(trie)->save(reinterpret_cast<const char *>(filename));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_write(const marisa_Trie* trie, const int fd, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<const marisa::Trie*>(trie)->write(fd);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


bool trie_lookup(const marisa_Trie* trie, marisa_Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->lookup(reinterpret_cast<marisa::Agent&>(agent));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false; // dummy, shoult not evaluate
}


void trie_reverse_lookup(const marisa_Trie* trie, marisa_Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<const marisa::Trie*>(trie)->reverse_lookup(reinterpret_cast<marisa::Agent&>(agent));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


bool trie_common_prefix_search(const marisa_Trie* trie, marisa_Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->common_prefix_search(reinterpret_cast<marisa::Agent&>(agent));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false; // dummy, shoult not evaluate
}

bool trie_predictive_search(const marisa_Trie* trie, marisa_Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->predictive_search(reinterpret_cast<marisa::Agent&>(agent));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false; // dummy, shoult not evaluate
}

std::size_t trie_num_tries(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->num_tries();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

std::size_t trie_num_keys(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->num_keys();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

std::size_t trie_num_nodes(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->num_nodes();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}


marisa::TailMode trie_tail_mode(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->tail_mode();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return MARISA_TEXT_TAIL;
}

marisa::NodeOrder trie_node_order(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->node_order();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return MARISA_WEIGHT_ORDER;
}

bool trie_empty(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->empty();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return true;
}

std::size_t trie_size(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t trie_total_size(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->total_size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t trie_io_size(const marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Trie*>(trie)->io_size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}


void trie_clear(marisa_Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_swap(marisa_Trie* trie, marisa_Trie& rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Trie*>(trie)->swap(
	       reinterpret_cast<marisa::Trie&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
