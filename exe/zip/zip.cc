#include <memory>
#include <string_view>
#include <vector>

#include <io.h>
#include <fcntl.h>

#include "miniz.h"

namespace {

std::vector<uint8_t> read_stdin() {
  std::vector<uint8_t> out;
  uint8_t buf[16 << 10];
  _setmode(0, _O_BINARY);  // you stay crazy, windows
  for (;;) {
    auto n = fread(buf, 1, sizeof(buf), stdin);
    if (n <= 0) break;
    out.insert(out.end(), buf, buf + n);
  }
  return out;
}

mz_bool put_vec(const void *pBuf, int len, void *pUser) {
  auto vec = static_cast<std::vector<uint8_t>*>(pUser);
  std::string_view buf(static_cast<const char*>(pBuf), len);
  vec->insert(vec->end(), buf.begin(), buf.end());
  return MZ_TRUE;
}

std::vector<uint8_t> compress(const std::vector<uint8_t>& input) {
  std::vector<uint8_t> out;
  tdefl_compress_mem_to_output(input.data(), input.size(), put_vec, &out, 0);
  return out;
}

std::vector<uint8_t> decompress(const std::vector<uint8_t>& input) {
  std::vector<uint8_t> out;
  size_t input_size = input.size();
  tinfl_decompress_mem_to_callback(input.data(), &input_size, put_vec, &out, 0);
  return out;
}

}

int main(int argc, const char* argv[]) {
  auto input = read_stdin();
  auto output = compress(input);
  auto reinput = decompress(output);
  printf("input size %d, compressed size %d, recompressed %d\n", input.size(), output.size(), reinput.size());
  if (input != reinput) {
    printf("ERROR: round trip failed!\n");
    return 1;
  }
  return 0; 
} 