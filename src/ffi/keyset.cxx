#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"


marisa::Keyset* keyset_create()
{
     return new marisa::Keyset();
}

void keyset_destroy(marisa::Keyset* keyset)
{
     delete keyset;
}

void keyset_push_back_0(marisa::Keyset* keyset, const marisa::Key& key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->push_back(key);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_1(marisa::Keyset* keyset, const marisa::Key& key, unsigned char end_marker, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->push_back(key, static_cast<char>(end_marker));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_2(marisa::Keyset* keyset, const unsigned char* str, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->push_back(reinterpret_cast<const char*>(str));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_3(marisa::Keyset* keyset, const unsigned char* ptr, std::size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->push_back(reinterpret_cast<const char*>(ptr), length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_4(marisa::Keyset* keyset, const unsigned char* ptr, std::size_t length, float weight, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->push_back(reinterpret_cast<const char*>(ptr), length, weight);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}


const marisa::Key* keyset_get(const marisa::Keyset* keyset, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return &(*keyset)[i];
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t keyset_num_keys(const marisa::Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return keyset->num_keys();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

bool keyset_empty(const marisa::Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return keyset->empty();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return true; // dummy, shoult not evaluate
}

std::size_t keyset_size(const marisa::Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return keyset->size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t keyset_total_length(const marisa::Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return keyset->total_length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void keyset_reset(marisa::Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->reset();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_clear(marisa::Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_swap(marisa::Keyset* keyset, marisa::Keyset& rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  keyset->swap(rhs);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
