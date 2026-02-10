#include <marisa/trie.h>
#include "marisa-wrapper.hpp"
#include "except.hpp"


Keyset* keyset_create()
{
     return reinterpret_cast<Keyset*>(new marisa::Keyset());
}

void keyset_destroy(Keyset* keyset)
{
     delete reinterpret_cast<marisa::Keyset*>(keyset);
}

void keyset_push_back_0(Keyset* keyset, const Key& key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const marisa::Key&>(key));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_1(Keyset* keyset, const Key& key, unsigned char end_marker, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const marisa::Key&>(key), (char)end_marker);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_2(Keyset* keyset, const unsigned char* str, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const char*>(str));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_3(Keyset* keyset, const unsigned char* ptr, std::size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const char*>(ptr), length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_4(Keyset* keyset, const unsigned char* ptr, std::size_t length, float weight, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const char*>(ptr), length, weight);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

const Key* keyset_get(const Keyset* keyset, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const Key*>(&(*reinterpret_cast<const marisa::Keyset*>(keyset))[i]);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t keyset_num_keys(const Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->num_keys();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

bool keyset_empty(const Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->empty();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return true; // dummy, shoult not evaluate
}

std::size_t keyset_size(const Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t keyset_total_length(const Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->total_length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void keyset_reset(Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->reset();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_clear(Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_swap(Keyset* keyset, marisa::Keyset& rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->swap(reinterpret_cast<marisa::Keyset&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
