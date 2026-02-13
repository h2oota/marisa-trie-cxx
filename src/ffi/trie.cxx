#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"


marisa::Trie* trie_create()
{
     return new marisa::Trie();
}


void trie_destroy(marisa::Trie* trie)
{
     delete trie;
}


void trie_build(marisa::Trie* trie, marisa::Keyset& keyset , int config_flag, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->build(keyset, config_flag);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_mmap(marisa::Trie* trie, const unsigned char *filename, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->mmap(reinterpret_cast<const char *>(filename));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_map(marisa::Trie* trie, const void *ptr, std::size_t size, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->map(ptr, size);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_load(marisa::Trie* trie, const unsigned char* filename, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->load(reinterpret_cast<const char *>(filename));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_read(marisa::Trie* trie, const int fd, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->read(fd);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


void trie_save(const marisa::Trie* trie, const unsigned char* filename, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->save(reinterpret_cast<const char *>(filename));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_write(const marisa::Trie* trie, const int fd, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->write(fd);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


bool trie_lookup(const marisa::Trie* trie, marisa::Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->lookup(agent);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false; // dummy, shoult not evaluate
}


void trie_reverse_lookup(const marisa::Trie* trie, marisa::Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->reverse_lookup(agent);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


bool trie_common_prefix_search(const marisa::Trie* trie, marisa::Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->common_prefix_search(agent);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false; // dummy, shoult not evaluate
}

bool trie_predictive_search(const marisa::Trie* trie, marisa::Agent& agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->predictive_search(agent);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false; // dummy, shoult not evaluate
}

std::size_t trie_num_tries(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->num_tries();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

std::size_t trie_num_keys(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->num_keys();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

std::size_t trie_num_nodes(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->num_nodes();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}


marisa::TailMode trie_tail_mode(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->tail_mode();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return MARISA_TEXT_TAIL;
}

marisa::NodeOrder trie_node_order(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->node_order();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return MARISA_WEIGHT_ORDER;
}

bool trie_empty(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->empty();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return true;
}

std::size_t trie_size(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t trie_total_size(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->total_size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t trie_io_size(const marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return trie->io_size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}


void trie_clear(marisa::Trie* trie, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void trie_swap(marisa::Trie* trie, marisa::Trie& rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  trie->swap(rhs);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
