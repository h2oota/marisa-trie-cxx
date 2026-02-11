#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"


marisa_Keyset* keyset_create()
{
     return reinterpret_cast<marisa_Keyset*>(new marisa::Keyset());
}

void keyset_destroy(marisa_Keyset* keyset)
{
     delete reinterpret_cast<marisa::Keyset*>(keyset);
}

void keyset_push_back_0(marisa_Keyset* keyset, const marisa_Key& key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const marisa::Key&>(key));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_1(marisa_Keyset* keyset, const marisa_Key& key, unsigned char end_marker, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const marisa::Key&>(key), (char)end_marker);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_2(marisa_Keyset* keyset, const unsigned char* str, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const char*>(str));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_3(marisa_Keyset* keyset, const unsigned char* ptr, std::size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const char*>(ptr), length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_push_back_4(marisa_Keyset* keyset, const unsigned char* ptr, std::size_t length, float weight, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->push_back(reinterpret_cast<const char*>(ptr), length, weight);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

const marisa_Key* keyset_get(const marisa_Keyset* keyset, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa_Key*>(&(*reinterpret_cast<const marisa::Keyset*>(keyset))[i]);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t keyset_num_keys(const marisa_Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->num_keys();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0; // dummy, shoult not evaluate
}

bool keyset_empty(const marisa_Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->empty();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return true; // dummy, shoult not evaluate
}

std::size_t keyset_size(const marisa_Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->size();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t keyset_total_length(const marisa_Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Keyset*>(keyset)->total_length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void keyset_reset(marisa_Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->reset();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_clear(marisa_Keyset* keyset, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void keyset_swap(marisa_Keyset* keyset, marisa_Keyset& rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Keyset*>(keyset)->swap(reinterpret_cast<marisa::Keyset&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
