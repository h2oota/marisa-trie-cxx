#include <marisa/trie.h>
#include "marisa-wrapper.hpp"
#include "except.hpp"

Query* query_create()
{
     return reinterpret_cast<Query*>(new marisa::Query());
}

void query_destroy(Query* query)
{
     delete reinterpret_cast<marisa::Query*>(query);
}

unsigned char query_get(const Query* query, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (unsigned char)(*reinterpret_cast<const marisa::Query*>(query))[i];
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

const unsigned char *query_ptr(const Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (const unsigned char *)reinterpret_cast<const marisa::Query*>(query)->ptr();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t query_length(const Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(query)->length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t query_id(const Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(query)->id();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void query_set_str(Query* query, const char *ptr, size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->set_str(ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_set_id(Query* query, std::size_t id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->set_id(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_clear(Query* query, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void query_swap(Query* query, Query &rhs, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Query*>(query)->swap(reinterpret_cast<marisa::Query&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
