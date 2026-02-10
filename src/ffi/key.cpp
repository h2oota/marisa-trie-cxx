#include <marisa/trie.h>
#include "marisa-wrapper.hpp"
#include "except.hpp"

Key* key_create()
{
     return reinterpret_cast<Key*>(new marisa::Key());
}

void key_destroy(Key* key)
{
     delete reinterpret_cast<marisa::Key*>(key);
}

unsigned char key_index(const Key* key, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (unsigned char)(*reinterpret_cast<const marisa::Key*>(key))[i];
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

const unsigned char *key_ptr(const Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (const unsigned char *)reinterpret_cast<const marisa::Key*>(key)->ptr();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t key_length(const Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Key*>(key)->length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t key_id(const Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Key*>(key)->id();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

float key_weight(const Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const marisa::Key*>(key)->weight();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void key_set_str(Key *key, const char *ptr, size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Key*>(key)->set_str(ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void key_set_id(marisa::Key *key, size_t id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Key*>(key)->set_id(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void key_set_weight(marisa::Key *key, float weight, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  reinterpret_cast<marisa::Key*>(key)->set_weight(weight);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
