#ifndef SHA_H

#include <arpa/inet.h>
#include <cstdint>
#include <cstring>

#define htonll(x) ((((uint64_t)htonl(x)) << 32) + htonl((x) >> 32))

#if defined (__GNUC__)
# define unlikely(x)     __builtin_expect((x), 0)
#else
# define unlikely(x)     (x)
#endif

//
// https://github.com/torvalds/linux/blob/master/include/crypto/sha.h
//

#define SHA256_DIGEST_SIZE      32
#define SHA256_BLOCK_SIZE       64

#define SHA256_H0	0x6a09e667UL
#define SHA256_H1	0xbb67ae85UL
#define SHA256_H2	0x3c6ef372UL
#define SHA256_H3	0xa54ff53aUL
#define SHA256_H4	0x510e527fUL
#define SHA256_H5	0x9b05688cUL
#define SHA256_H6	0x1f83d9abUL
#define SHA256_H7	0x5be0cd19UL

extern const unsigned char sha256_zero_message_hash[SHA256_DIGEST_SIZE];

struct sha256_state {
  uint32_t state[SHA256_DIGEST_SIZE / 4];
  uint64_t count;
  unsigned char buf[SHA256_BLOCK_SIZE];
};

static inline void sha256_init(sha256_state* sctx) {
  sctx->state[0] = SHA256_H0;
  sctx->state[1] = SHA256_H1;
  sctx->state[2] = SHA256_H2;
  sctx->state[3] = SHA256_H3;
  sctx->state[4] = SHA256_H4;
  sctx->state[5] = SHA256_H5;
  sctx->state[6] = SHA256_H6;
  sctx->state[7] = SHA256_H7;
  sctx->count = 0;
}

extern "C" {
void sha256_update(sha256_state*, unsigned char const *data, unsigned len);
void sha256_final(sha256_state*, unsigned char* out);
}

typedef void (sha256_block_fn)(sha256_state*, unsigned char const*, int);

static inline int sha256_base_init(sha256_state* sctx) {
  sha256_init(sctx);
  return 0;
}

template <sha256_block_fn* block_fn>
static inline int sha256_base_do_update(sha256_state* sctx,
					unsigned char const* data,
					unsigned len) {
  unsigned int partial = sctx->count % SHA256_BLOCK_SIZE;

  sctx->count += len;

  if (unlikely((partial + len) >= SHA256_BLOCK_SIZE)) {
    int blocks;

    if (partial) {
      int p = SHA256_BLOCK_SIZE - partial;

      memcpy(sctx->buf + partial, data, p);
      data += p;
      len -= p;

      block_fn(sctx, sctx->buf, 1);
    }

    blocks = len / SHA256_BLOCK_SIZE;
    len %= SHA256_BLOCK_SIZE;

    if (blocks) {
      block_fn(sctx, data, blocks);
      data += blocks * SHA256_BLOCK_SIZE;
    }
    partial = 0;
  }
  if (len)
    memcpy(sctx->buf + partial, data, len);

  return 0;
}

template <sha256_block_fn* block_fn>
static inline int sha256_base_do_finalize(sha256_state* sctx) {
  int const bit_offset = SHA256_BLOCK_SIZE - sizeof(std::uint64_t);
  uint64_t *bits = (uint64_t *)(sctx->buf + bit_offset);
  unsigned partial = sctx->count % SHA256_BLOCK_SIZE;

  sctx->buf[partial++] = 0x80;
  if (partial > bit_offset) {
    memset(sctx->buf + partial, 0x0, SHA256_BLOCK_SIZE - partial);
    partial = 0;

    block_fn(sctx, sctx->buf, 1);
  }

  memset(sctx->buf + partial, 0x0, bit_offset - partial);
  *bits = htonll(sctx->count << 3);
  block_fn(sctx, sctx->buf, 1);

  return 0;
}

static inline int sha256_base_finish(sha256_state* sctx, unsigned char* out) {
  unsigned int digest_size = SHA256_DIGEST_SIZE;
  uint32_t *digest = (uint32_t *)out;
  int i;

  for (i = 0; digest_size > 0; i++, digest_size -= sizeof(uint32_t)) {
    *digest = htonl(sctx->state[i]);
    ++digest;
  }

  return 0;
}
#endif
