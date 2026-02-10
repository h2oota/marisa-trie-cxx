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

#pragma pack(1)
     typedef unsigned char Key[sizeof(marisa::Key)];
     typedef unsigned char Query[sizeof(marisa::Query)];
     typedef unsigned char Keyset[sizeof(marisa::Keyset)];
     typedef unsigned char Agent[sizeof(marisa::Agent)];
     typedef unsigned char Trie[sizeof(marisa::Trie)];
#pragma pack()
     // marisa::Key

     Key* key_create();
     void key_destroy(Key*);

     unsigned char key_index(const Key*, std::size_t, const struct exception_record **); // operator[]
     const unsigned char* key_ptr(const Key*, const struct exception_record **);
     std::size_t key_length(const Key*, const struct exception_record **);
     std::size_t key_id(const Key*, const struct exception_record **);

     float key_weight(const Key*, const struct exception_record **);
     void key_set_str(Key* key, const char *ptr, std::size_t, const struct exception_record **);
     void key_set_id(Key* key, std::size_t id, const struct exception_record **);
     void key_set_weight(Key* key, float, const struct exception_record **);

     // Query

     Query* query_create();
     void query_destroy(Query*);
     unsigned char query_get(const Query*, std::size_t, const struct exception_record **); // operator[]
     const unsigned char* query_ptr(const Query*, const struct exception_record **);
     std::size_t query_length(const Query*, const struct exception_record **);
     std::size_t query_id(const Query*, const struct exception_record **);
     void query_set_str(Query*, const char *, std::size_t, const struct exception_record **);
     void query_set_id(Query*, std::size_t, const struct exception_record **);
     void query_clear(Query*, const struct exception_record **);
     void query_swap(Query*, Query &, const struct exception_record **);

     // Keyset

     Keyset* keyset_create();
     void keyset_destroy(Keyset*);

     void keyset_push_back_0(Keyset*, const Key&, const struct exception_record **);
     void keyset_push_back_1(Keyset*, const Key&, unsigned char, const struct exception_record **);	// push_back1
     void keyset_push_back_2(Keyset*, const unsigned char *, const struct exception_record **);		// push_back2
     void keyset_push_back_3(Keyset*, const unsigned char *, std::size_t, const struct exception_record **);	// push_back3
     void keyset_push_back_4(Keyset*,							// push_back4
			     const unsigned char *, std::size_t, float, const struct exception_record **);

     const Key* get(const Keyset*, std::size_t, const struct exception_record **);
     Key* set(Keyset*, std::size_t, const struct exception_record **);

     const Key* keyset_get(const Keyset*, std::size_t, const struct exception_record **);
     Key* keyset_put(Keyset*, std::size_t, Key&, const struct exception_record **);
     std::size_t keyset_num_keys(const Keyset*, const struct exception_record **);
     bool keyset_empty(const Keyset*, const struct exception_record **);
     std::size_t keyset_size(const Keyset*, const struct exception_record **);
     std::size_t keyset_total_length(const Keyset*, const struct exception_record **);
     void keyset_reset(Keyset*, const struct exception_record **);
     void keyset_clear(Keyset*, const struct exception_record **);
     void keyset_swap(Keyset*, Keyset& rhs, const struct exception_record **);

     // Agent

     Agent* agent_create();
     void agent_destroy(Agent*);
//	  const Query& query(const Agent*, const struct exception_record **);
//	  const Key& key(const Agent*, const struct exception_record **);
     const Query* agent_query(const Agent*, const struct exception_record **);
     const Key* agent_key(const Agent*, const struct exception_record **);
     void agent_set_query_0(Agent*, const unsigned char*, const struct exception_record **);
     void agent_set_query_1(Agent*, const unsigned char*, std::size_t, const struct exception_record **);
     void agent_set_query_2(Agent*, std::size_t, const struct exception_record **);
     void agent_set_key_0(Agent*, const unsigned char*, const struct exception_record **);
     void agent_set_key_1(Agent*, const unsigned char*, std::size_t, const struct exception_record **);
     void agent_set_key_2(Agent*, std::size_t, const struct exception_record **);
     bool agent_has_state(const Agent*, const struct exception_record **);
     void agent_init_state(Agent*, const struct exception_record **);
     void agent_clear(Agent*, const struct exception_record **);
     void agent_swap(Agent*, Agent &, const struct exception_record **);

     // Trie

     Trie* trie_create();
     void trie_destroy(Trie*);

     void trie_build(Trie*, Keyset&, int, const struct exception_record **);
     void trie_mmap(Trie*, const unsigned char*, const struct exception_record **);
     void trie_map(Trie*, const void*, std::size_t, const struct exception_record **);

     void trie_load(Trie*, const unsigned char*, const struct exception_record **);
     void trie_read(Trie*, int, const struct exception_record **);

     void trie_save(const Trie*, const unsigned char *, const struct exception_record **);
     void trie_write(const Trie*, int fd, const struct exception_record **);

     bool trie_lookup(const Trie*, Agent&, const struct exception_record **);
     void trie_reverse_lookup(const Trie*, Agent&, const struct exception_record **);
     bool trie_common_prefix_search(const Trie*, Agent&, const struct exception_record **);
     bool trie_predictive_search(const Trie*, Agent&, const struct exception_record **);

     std::size_t trie_num_tries(const Trie*, const struct exception_record **);
     std::size_t trie_num_keys(const Trie*, const struct exception_record **);
     std::size_t trie_num_nodes(const Trie*, const struct exception_record **);

     marisa::TailMode trie_tail_mode(const Trie*, const struct exception_record **);
     marisa::NodeOrder trie_node_order(const Trie*, const struct exception_record **);

     bool trie_empty(const Trie*, const struct exception_record **);
     std::size_t trie_size(const Trie*, const struct exception_record **);
     std::size_t trie_total_size(const Trie*, const struct exception_record **);
     std::size_t trie_io_size(const Trie*, const struct exception_record **);

     void trie_clear(Trie*, const struct exception_record **);
     void trie_swap(Trie*, Trie&, const struct exception_record **);

     const char *exception_name(const exception_record *);
     const char *exception_message(const exception_record *);
}
