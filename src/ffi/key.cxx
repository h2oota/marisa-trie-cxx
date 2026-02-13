#include <marisa/trie.h>
#include "ffi.hxx"
#include "except.hxx"

marisa::Key* key_create()
{
     return new marisa::Key();
}

void key_destroy(marisa::Key* key)
{
     delete key;
}

/*
unsigned char key_index(const marisa::Key* key, std::size_t i, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return (unsigned char)(*reinterpret_cast<const marisa::Key*>(key))[i];
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}
*/

const unsigned char *key_ptr(const marisa::Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return reinterpret_cast<const unsigned char *>(key->ptr());
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return nullptr;
}

std::size_t key_length(const marisa::Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return key->length();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

std::size_t key_id(const marisa::Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return key->id();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

float key_weight(const marisa::Key* key, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  return key->weight();
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
     return 0;
}

void key_set_str(marisa::Key *key, const char *ptr, size_t length, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  key->set_str(ptr, length);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void key_set_id(marisa::Key *key, size_t id, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  key->set_id(id);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}

void key_set_weight(marisa::Key *key, float weight, const struct exception_record ** exception)
{
     *exception = nullptr;
     try {
	  key->set_weight(weight);
     } catch (const std::exception &ex) {
	  *exception = save_exception(ex);
     }
}
