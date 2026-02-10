#include <marisa/trie.h>
#include "marisa-wrapper.hpp"
#include "except.hpp"


Agent* agent_create()
{
     return reinterpret_cast<Agent*>(new marisa::Agent());
}

void agent_destroy(Agent* agent)
{
     delete reinterpret_cast<marisa::Agent*>(agent);
}

const Query* agent_query(const Agent* agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const Query*>(&reinterpret_cast<const marisa::Agent*>(agent)->query());
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

const Key* agent_key(const Agent* agent, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const Key*>(&reinterpret_cast<const marisa::Agent*>(agent)->key());
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

void agent_set_query_0(Agent* agent, const unsigned char* str, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->set_query((const char*)str);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_query_1(Agent* agent, const unsigned char* ptr, std::size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->set_query((const char*)ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_query_2(Agent* agent, std::size_t key_id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->set_query(key_id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_key_0(Agent* agent, const unsigned char* str, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->set_key(reinterpret_cast<const char *>(str));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_key_1(Agent* agent, const unsigned char* str, std::size_t length, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->set_key(reinterpret_cast<const char *>(str), length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_set_key_2(Agent* agent, std::size_t id, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->set_key(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

bool agent_has_state(const Agent* agent, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Agent*>(agent)->has_state();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return false;
}

void agent_init_state(Agent* agent, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->init_state();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_clear(Agent* agent, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->clear();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void agent_swap(Agent* agent, Agent &rhs, const struct exception_record **exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Agent*>(agent)->swap(reinterpret_cast<marisa::Agent&>(rhs));
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
