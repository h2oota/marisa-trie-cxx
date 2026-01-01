#pragma once
#include <stdint.h>
#include <cstddef>
#include <cstdio>
#include <stdexcept>
#include <system_error>
#include <typeinfo>

#include <marisa/trie.h>

extern "C" {
//     typedef marisa::TailMode marisa_TailMode;
//     typedef marisa::NodeOrder marisa_NodeOrder;

     // marisa::Key

     marisa::Key* key_create();
     void key_destroy(marisa::Key*);

     unsigned char key_get(const marisa::Key*, std::size_t, const struct exception_record **); // operator[]
     const unsigned char* key_ptr(const marisa::Key*, const struct exception_record **);
     std::size_t key_length(const marisa::Key*, const struct exception_record **);
     std::size_t key_id(const marisa::Key*, const struct exception_record **);

     float key_weight(const marisa::Key*, const struct exception_record **);
     void key_set_str(marisa::Key* key, const char *ptr, std::size_t, const struct exception_record **);
     void key_set_id(marisa::Key* key, std::size_t id, const struct exception_record **);
     void key_set_weight(marisa::Key* key, float, const struct exception_record **);

     // marisa::Query

     marisa::Query* query_create();
     void query_destroy(marisa::Query*);
     unsigned char query_get(const marisa::Query*, std::size_t, const struct exception_record **); // operator[]
     const unsigned char* query_ptr(const marisa::Query*, const struct exception_record **);
     std::size_t query_length(const marisa::Query*, const struct exception_record **);
     std::size_t query_id(const marisa::Query*, const struct exception_record **);
     void query_set_str(marisa::Query*, const char *, std::size_t, const struct exception_record **);
     void query_set_id(marisa::Query*, std::size_t, const struct exception_record **);
     void query_clear(marisa::Query*, const struct exception_record **);
     void query_swap(marisa::Query*, marisa::Query &, const struct exception_record **);

     // marisa::Keyset

     marisa::Keyset* keyset_create();
     void keyset_destroy(marisa::Keyset*);

     void keyset_push_back_0(marisa::Keyset*, const marisa::Key&, const struct exception_record **);
     void keyset_push_back_1(marisa::Keyset*, const marisa::Key&, unsigned char, const struct exception_record **);	// push_back1
     void keyset_push_back_2(marisa::Keyset*, const unsigned char *, const struct exception_record **);		// push_back2
     void keyset_push_back_3(marisa::Keyset*, const unsigned char *, std::size_t, const struct exception_record **);	// push_back3
     void keyset_push_back_4(marisa::Keyset*,							// push_back4
					    const unsigned char *, std::size_t, float, const struct exception_record **);

     //const marisa::Key& get(const marisa::Keyset*, std::size_t, const struct exception_record **);
     // marisa::Key& put(marisa::Keyset*, std::size_t, marisa::Key&, const struct exception_record **);
     const marisa::Key* keyset_get(const marisa::Keyset*, std::size_t, const struct exception_record **);
     marisa::Key* keyset_put(marisa::Keyset*, std::size_t, marisa::Key&, const struct exception_record **);
     std::size_t keyset_num_keys(const marisa::Keyset*, const struct exception_record **);
     bool keyset_empty(const marisa::Keyset*, const struct exception_record **);
     std::size_t keyset_size(const marisa::Keyset*, const struct exception_record **);
     std::size_t keyset_total_length(const marisa::Keyset*, const struct exception_record **);
     void keyset_reset(marisa::Keyset*, const struct exception_record **);
     void keyset_clear(marisa::Keyset*, const struct exception_record **);
     void keyset_swap(marisa::Keyset*, marisa::Keyset& rhs, const struct exception_record **);

     // marisa::Agent

     marisa::Agent* agent_create();
     void agent_destroy(marisa::Agent*);
//	  const marisa::Query& query(const marisa::Agent*, const struct exception_record **);
//	  const marisa::Key& key(const marisa::Agent*, const struct exception_record **);
     const marisa::Query* agent_query(const marisa::Agent*, const struct exception_record **);
     const marisa::Key* agent_key(const marisa::Agent*, const struct exception_record **);
     void agent_set_query_0(marisa::Agent*, const unsigned char*, const struct exception_record **);
     void agent_set_query_1(marisa::Agent*, const unsigned char*, std::size_t, const struct exception_record **);
     void agent_set_query_2(marisa::Agent*, std::size_t, const struct exception_record **);
     void agent_set_key_0(marisa::Agent*, const unsigned char*, const struct exception_record **);
     void agent_set_key_1(marisa::Agent*, const unsigned char*, std::size_t, const struct exception_record **);
     void agent_set_key_2(marisa::Agent*, std::size_t, const struct exception_record **);
     bool agent_has_state(const marisa::Agent*, const struct exception_record **);
     void agent_init_state(marisa::Agent*, const struct exception_record **);
     void agent_clear(marisa::Agent*, const struct exception_record **);
     void agent_swap(marisa::Agent*, marisa::Agent &, const struct exception_record **);

     // marisa::Trie

     marisa::Trie* trie_create();
     void trie_destroy(marisa::Trie*);

     void trie_build(marisa::Trie*, marisa::Keyset&, int, const struct exception_record **);
     void trie_mmap(marisa::Trie*, const unsigned char*, const struct exception_record **);
     void trie_map(marisa::Trie*, const void*, std::size_t, const struct exception_record **);

     void trie_load(marisa::Trie*, const unsigned char*, const struct exception_record **);
     // void read(marisa::Trie*, int);

     void trie_save(const marisa::Trie*, const unsigned char *, const struct exception_record **);
     // void write(const marisa::Trie*, int fd) const;

     bool trie_lookup(const marisa::Trie*, marisa::Agent&, const struct exception_record **);
     void trie_reverse_lookup(const marisa::Trie*, marisa::Agent&, const struct exception_record **);
     bool trie_common_prefix_search(const marisa::Trie*, marisa::Agent&, const struct exception_record **);
     bool trie_predictive_search(const marisa::Trie*, marisa::Agent&, const struct exception_record **);

     std::size_t trie_num_tries(const marisa::Trie*, const struct exception_record **);
     std::size_t trie_num_keys(const marisa::Trie*, const struct exception_record **);
     std::size_t trie_num_nodes(const marisa::Trie*, const struct exception_record **);

     marisa::TailMode trie_tail_mode(const marisa::Trie*, const struct exception_record **);
     marisa::NodeOrder trie_node_order(const marisa::Trie*, const struct exception_record **);

     bool trie_empty(const marisa::Trie*, const struct exception_record **);
     std::size_t trie_size(const marisa::Trie*, const struct exception_record **);
     std::size_t trie_total_size(const marisa::Trie*, const struct exception_record **);
     std::size_t trie_io_size(const marisa::Trie*, const struct exception_record **);

     void trie_clear(marisa::Trie*, const struct exception_record **);
     void trie_swap(marisa::Trie*, marisa::Trie&, const struct exception_record **);

     const char *exception_name(const exception_record *);
     const char *exception_message(const exception_record *);
}
