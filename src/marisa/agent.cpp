#include <marisa/trie.h>
#include "marisa-wrapper.hpp"
#include "except.hpp"


marisa::Agent* agent_create()
{
     return new marisa::Agent();
}

void agent_destroy(marisa::Agent* agent)
{
     delete agent;
}

const marisa::Query* agent_query(const marisa::Agent* agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Query*>(&agent->query());
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

const marisa::Key* agent_key(const marisa::Agent* agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Key*>(&agent->key());
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

void agent_set_query_0(marisa::Agent* agent, const unsigned char* str, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  agent->set_query((const char*)str);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_query_1(marisa::Agent* agent, const unsigned char* ptr, std::size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  agent->set_query((const char*)ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_query_2(marisa::Agent* agent, std::size_t key_id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  agent->set_query(key_id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_key_0(marisa::Agent* agent, const unsigned char* str, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  agent->set_key(reinterpret_cast<const char *>(str));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_key_1(marisa::Agent* agent, const unsigned char* str, std::size_t length, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  agent->set_key(reinterpret_cast<const char *>(str), length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_key_2(marisa::Agent* agent, std::size_t id, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  agent->set_key(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

bool agent_has_state(const marisa::Agent* agent, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  return agent->has_state();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false;
}

void agent_init_state(marisa::Agent* agent, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  agent->init_state();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_clear(marisa::Agent* agent, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  agent->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_swap(marisa::Agent* agent, marisa::Agent &rhs, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  agent->swap(rhs);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
