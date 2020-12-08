#include "sha.hxx"

extern "C" {
  void sha256_transform_ssse3(struct sha256_state* state,
                              unsigned char const* data,
                              int blocks);
}

void sha256_update(sha256_state* sctx, unsigned char const* data, unsigned len) {
  sha256_base_do_update<sha256_transform_ssse3>(sctx, data, len);
}

void sha256_final(sha256_state* sctx, unsigned char* out) {
  sha256_base_do_finalize<sha256_transform_ssse3>(sctx);
  sha256_base_finish(sctx, out);
}
