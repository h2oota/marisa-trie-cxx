#pragma once
#include <stdint.h>
#include <cstddef>
#include <cstdio>
#include <stdexcept>
#include <system_error>
#include <typeinfo>

#include <marisa/trie.h>

namespace marisa {
     struct Test {
	  int a, b;
     };
}

extern "C" {
//     typedef marisa::TailMode marisa_TailMode;
//     typedef marisa::NodeOrder marisa_NodeOrder;

#pragma pack(1)
     typedef unsigned char marisa_Key[sizeof(marisa::Key)];
     typedef unsigned char marisa_Query[sizeof(marisa::Query)];
     typedef unsigned char marisa_Keyset[sizeof(marisa::Keyset)];
     typedef unsigned char marisa_Agent[sizeof(marisa::Agent)];
     typedef unsigned char marisa_Trie[sizeof(marisa::Trie)];
#pragma pack()
     // marisa::Key

     marisa_Key* key_create();
     void key_destroy(marisa_Key*);

     unsigned char key_index(const marisa_Key*, std::size_t, const struct exception_record **); // operator[]
     const unsigned char* key_ptr(const marisa_Key*, const struct exception_record **);
     std::size_t key_length(const marisa_Key*, const struct exception_record **);
     std::size_t key_id(const marisa_Key*, const struct exception_record **);

     float key_weight(const marisa_Key*, const struct exception_record **);
     void key_set_str(marisa_Key*, const char *, std::size_t, const struct exception_record **);
     void key_set_id(marisa_Key*, std::size_t, const struct exception_record **);
     void key_set_weight(marisa_Key*, float, const struct exception_record **);

     // Query

     marisa_Query* query_create();
     void query_destroy(marisa_Query*);
     unsigned char query_get(const marisa_Query*, std::size_t, const struct exception_record **); // operator[]
     const unsigned char* query_ptr(const marisa_Query*, const struct exception_record **);
     std::size_t query_length(const marisa_Query*, const struct exception_record **);
     std::size_t query_id(const marisa_Query*, const struct exception_record **);
     void query_set_str(marisa_Query*, const char *, std::size_t, const struct exception_record **);
     void query_set_id(marisa_Query*, std::size_t, const struct exception_record **);
     void query_clear(marisa_Query*, const struct exception_record **);
     void query_swap(marisa_Query*, marisa_Query &, const struct exception_record **);

     // Keyset

     marisa_Keyset* keyset_create();
     void keyset_destroy(marisa_Keyset*);

     void keyset_push_back_0(marisa_Keyset*, const marisa_Key&, const struct exception_record **);
     void keyset_push_back_1(marisa_Keyset*, const marisa_Key&, unsigned char, const struct exception_record **);	// push_back1
     void keyset_push_back_2(marisa_Keyset*, const unsigned char *, const struct exception_record **);		// push_back2
     void keyset_push_back_3(marisa_Keyset*, const unsigned char *, std::size_t, const struct exception_record **);	// push_back3
     void keyset_push_back_4(marisa_Keyset*,							// push_back4
			     const unsigned char *, std::size_t, float, const struct exception_record **);

     const marisa_Key* get(const marisa_Keyset*, std::size_t, const struct exception_record **);
     marisa_Key* set(marisa_Keyset*, std::size_t, const struct exception_record **);

     const marisa_Key* keyset_get(const marisa_Keyset*, std::size_t, const struct exception_record **);
     marisa_Key* keyset_put(marisa_Keyset*, std::size_t, marisa_Key&, const struct exception_record **);
     std::size_t keyset_num_keys(const marisa_Keyset*, const struct exception_record **);
     bool keyset_empty(const marisa_Keyset*, const struct exception_record **);
     std::size_t keyset_size(const marisa_Keyset*, const struct exception_record **);
     std::size_t keyset_total_length(const marisa_Keyset*, const struct exception_record **);
     void keyset_reset(marisa_Keyset*, const struct exception_record **);
     void keyset_clear(marisa_Keyset*, const struct exception_record **);
     void keyset_swap(marisa_Keyset*, marisa_Keyset& rhs, const struct exception_record **);

     // Agent

     marisa_Agent* agent_create();
     void agent_destroy(marisa_Agent*);
//	  const Query& query(const Agent*, const struct exception_record **);
//	  const Key& key(const Agent*, const struct exception_record **);
     const marisa_Query* agent_query(const marisa_Agent*, const struct exception_record **);
     const marisa_Key* agent_key(const marisa_Agent*, const struct exception_record **);
     void agent_set_query_0(marisa_Agent*, const unsigned char*, const struct exception_record **);
     void agent_set_query_1(marisa_Agent*, const unsigned char*, std::size_t, const struct exception_record **);
     void agent_set_query_2(marisa_Agent*, std::size_t, const struct exception_record **);
     void agent_set_key_0(marisa_Agent*, const unsigned char*, const struct exception_record **);
     void agent_set_key_1(marisa_Agent*, const unsigned char*, std::size_t, const struct exception_record **);
     void agent_set_key_2(marisa_Agent*, std::size_t, const struct exception_record **);
     bool agent_has_state(const marisa_Agent*, const struct exception_record **);
     void agent_init_state(marisa_Agent*, const struct exception_record **);
     void agent_clear(marisa_Agent*, const struct exception_record **);
     void agent_swap(marisa_Agent*, marisa_Agent &, const struct exception_record **);

     // Trie

     marisa_Trie* trie_create();
     void trie_destroy(marisa_Trie*);

     void trie_build(marisa_Trie*, marisa_Keyset&, int, const struct exception_record **);
     void trie_mmap(marisa_Trie*, const unsigned char*, const struct exception_record **);
     void trie_map(marisa_Trie*, const void*, std::size_t, const struct exception_record **);

     void trie_load(marisa_Trie*, const unsigned char*, const struct exception_record **);
     void trie_read(marisa_Trie*, int, const struct exception_record **);

     void trie_save(const marisa_Trie*, const unsigned char *, const struct exception_record **);
     void trie_write(const marisa_Trie*, int fd, const struct exception_record **);

     bool trie_lookup(const marisa_Trie*, marisa_Agent&, const struct exception_record **);
     void trie_reverse_lookup(const marisa_Trie*, marisa_Agent&, const struct exception_record **);
     bool trie_common_prefix_search(const marisa_Trie*, marisa_Agent&, const struct exception_record **);
     bool trie_predictive_search(const marisa_Trie*, marisa_Agent&, const struct exception_record **);

     std::size_t trie_num_tries(const marisa_Trie*, const struct exception_record **);
     std::size_t trie_num_keys(const marisa_Trie*, const struct exception_record **);
     std::size_t trie_num_nodes(const marisa_Trie*, const struct exception_record **);

     marisa::TailMode trie_tail_mode(const marisa_Trie*, const struct exception_record **);
     marisa::NodeOrder trie_node_order(const marisa_Trie*, const struct exception_record **);

     bool trie_empty(const marisa_Trie*, const struct exception_record **);
     std::size_t trie_size(const marisa_Trie*, const struct exception_record **);
     std::size_t trie_total_size(const marisa_Trie*, const struct exception_record **);
     std::size_t trie_io_size(const marisa_Trie*, const struct exception_record **);

     void trie_clear(marisa_Trie*, const struct exception_record **);
     void trie_swap(marisa_Trie*, marisa_Trie&, const struct exception_record **);

     const char *exception_name(const exception_record *);
     const char *exception_message(const exception_record *);
}
