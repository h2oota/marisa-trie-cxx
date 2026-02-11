#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"

marisa_Query* query_create()
{
     return reinterpret_cast<marisa_Query*>(new marisa::Query());
}

void query_destroy(marisa_Query* query)
{
     delete reinterpret_cast<marisa::Query*>(query);
}

unsigned char query_get(const marisa_Query* query, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (unsigned char)(*reinterpret_cast<const marisa::Query*>(query))[i];
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

const unsigned char *query_ptr(const marisa_Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (const unsigned char *)reinterpret_cast<const marisa::Query*>(query)->ptr();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t query_length(const marisa_Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(query)->length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t query_id(const marisa_Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(query)->id();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void query_set_str(marisa_Query* query, const char *ptr, size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->set_str(ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_set_id(marisa_Query* query, std::size_t id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->set_id(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_clear(marisa_Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_swap(marisa_Query* query, marisa_Query &rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->swap(reinterpret_cast<marisa::Query&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
