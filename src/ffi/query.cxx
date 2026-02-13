#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"

marisa::Query* query_create()
{
     return new marisa::Query();
}

void query_destroy(marisa::Query* query)
{
     delete query;
}

unsigned char query_get(const marisa::Query* query, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (unsigned char)(*reinterpret_cast<const marisa::Query*>(query))[i];
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

const unsigned char *query_ptr(const marisa::Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (const unsigned char *)reinterpret_cast<const marisa::Query*>(query)->ptr();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t query_length(const marisa::Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(query)->length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t query_id(const marisa::Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(query)->id();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void query_set_str(marisa::Query* query, const char *ptr, size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  query->set_str(ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_set_id(marisa::Query* query, std::size_t id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  query->set_id(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_clear(marisa::Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  query->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_swap(marisa::Query* query, marisa::Query &rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  query->swap(reinterpret_cast<marisa::Query&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
