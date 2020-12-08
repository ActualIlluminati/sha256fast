#include <ctime>
#include <cstdio>

#include "sha.hxx"

#if defined(__GNUC__)
# define ALWAYS_INLINE __attribute__ ((always_inline))
#else
# define ALWAYS_INLINE
#endif

// The DoNotOptimize(...) function can be used to prevent a value or
// expression from being optimized away by the compiler. This function is
// intended to add little to no overhead.
// See: https://youtu.be/nXaxk27zwlk?t=2441
template <class Tp>
inline ALWAYS_INLINE void DoNotOptimize(Tp const& value) {
  asm volatile("" : : "r,m"(value) : "memory");
}

template <class Tp>
inline ALWAYS_INLINE void DoNotOptimize(Tp& value) {
#if defined(__clang__)
  asm volatile("" : "+r,m"(value) : : "memory");
#else
  asm volatile("" : "+m,r"(value) : : "memory");
#endif
}

// Force the compiler to flush pending writes to global memory. Acts as an
// effective read/write barrier
inline ALWAYS_INLINE void ClobberMemory() {
  asm volatile("" : : : "memory");
}

typedef unsigned long long ticks;

static __inline__ ticks getticks(void)
{
     unsigned a, d;
     asm volatile("rdtsc" : "=a" (a), "=d" (d));

     return (((ticks)a) | (((ticks)d) << 32));
}

int main(int, char**) {
  struct sha256_state st;
  unsigned char const input1[] = "abc";
  unsigned char const input2[] =
      "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn"
      "hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
  unsigned char const input3[] =
      "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnzzzzzzzz";
  unsigned char out[32];

  /* printf("state:\n");
   * for (int i = 0; i < sizeof(st.state) / sizeof(*st.state); ++i) {
   *   printf("%04X", st.state[i]);
   * }
   * printf("\n");
   * printf("sz.count: %lld\n", st.count); */

  sha256_init(&st);
  sha256_update(&st, input1, sizeof(input1) - 1);
  sha256_final(&st, out);

  printf("out:");
  for (std::size_t i = 0; i < sizeof(out); ++i) {
    if (i % 4 == 0) {
      printf(" ");
    }
    printf("%02X", out[i]);
  }
  printf("\n");
  printf("cmp: ba7816bf 8f01cfea 414140de 5dae2223 b00361a3 96177a9c b410ff61 f20015ad\n");

  sha256_init(&st);
  sha256_update(&st, input2, sizeof(input2) - 1);
  sha256_final(&st, out);

  printf("out:");
  for (std::size_t i = 0; i < sizeof(out); ++i) {
    if (i % 4 == 0) {
      printf(" ");
    }
    printf("%02X", out[i]);
  }
  printf("\n");
  printf("cmp: cf5b16a7 78af8380 036ce59e 7b049237 0b249b11 e8f07a51 afac4503 7afee9d1\n");

  int const times = 10000000;

  {
    printf("Bench 1:\n");
    clock_t clock_start = clock();
    ticks start = getticks();
    for (int i = 0; i < times; ++i) {
      sha256_init(&st);
      sha256_update(&st, input1, sizeof(input1) - 1);
      sha256_final(&st, out);
      DoNotOptimize(out);
      ClobberMemory();
    }
    ticks finish = getticks();
    clock_t clock_finish = clock();
    printf("Took %llu cycles per iteration\n", (finish - start) / times);
    printf("Took %f s\n", (double)(clock_finish - clock_start) / CLOCKS_PER_SEC);
  }

  {
    printf("Bench 2:\n");
    clock_t clock_start = clock();
    ticks start = getticks();
    for (int i = 0; i < times; ++i) {
      sha256_init(&st);
      sha256_update(&st, input2, sizeof(input2) - 1);
      sha256_final(&st, out);
      DoNotOptimize(out);
      ClobberMemory();
    }
    ticks finish = getticks();
    clock_t clock_finish = clock();
    printf("Took %llu cycles per iteration\n", (finish - start) / times);
    printf("Took %f s\n", (double)(clock_finish - clock_start) / CLOCKS_PER_SEC);
  }

  {
    printf("Bench 3:\n");
    clock_t clock_start = clock();
    ticks start = getticks();
    for (int i = 0; i < times; ++i) {
      sha256_init(&st);
      sha256_update(&st, input3, sizeof(input3) - 1);
      sha256_final(&st, out);
      DoNotOptimize(out);
      ClobberMemory();
    }
    ticks finish = getticks();
    clock_t clock_finish = clock();
    printf("Took %llu cycles per iteration\n", (finish - start) / times);
    printf("Took %f s\n", (double)(clock_finish - clock_start) / CLOCKS_PER_SEC);
  }
}
