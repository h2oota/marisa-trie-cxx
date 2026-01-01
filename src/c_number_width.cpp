// helper functions used in build.rs
#include <climits>
#include <cstddef>

extern "C" {
     std::size_t char_bits()
     {
	  return CHAR_BIT;
     }

     std::size_t pointer_width()
     {
	  return sizeof(void *) * CHAR_BIT;
     }

     std::size_t int_width()
     {
	  return sizeof(int) * CHAR_BIT;
     }

     std::size_t float_width()
     {
	  return sizeof(float) * CHAR_BIT;
     }

     std::size_t double_width()
     {
	  return sizeof(double) * CHAR_BIT;
     }
}
