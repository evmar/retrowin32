#include <memory>
#include <string_view>
#include <vector>

#include <io.h>
#include <fcntl.h>

#include "miniz.h"

namespace {

std::vector<uint8_t> read_file(const char* path) {
  std::vector<uint8_t> out;
  uint8_t buf[16 << 10];

  FILE* f = fopen(path, "rb");
  assert(f);
  for (;;) {
    auto n = fread(buf, 1, sizeof(buf), f);
    if (n <= 0) break;
    out.insert(out.end(), buf, buf + n);
  }
  fclose(f);
  return out;
}

std::vector<uint8_t> construct_input(const char* arg) {
  int kb = 1;
  if (arg) {
    if (std::string(arg).find_first_not_of("0123456789") != std::string::npos) {
      return read_file(arg);
    }
    kb = std::atoi(arg);
  }
  std::vector<uint8_t> input;
  std::string_view text = "sixteen letters!";
  for (auto i = 0; i < kb * (1024 / text.size()); i++) {
    input.insert(input.end(), text.begin(), text.end());
  }
  return input;
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
  std::vector<uint8_t> input = construct_input(argc > 1 ? argv[1] : nullptr);
  printf("input size: %d\n", input.size());
  auto output = compress(input);
  printf("compressed size: %d\n", output.size());
  auto reinput = decompress(output);
  printf("recompressed size: %d\n", reinput.size());
  if (input != reinput) {
    printf("ERROR: round trip failed!\n");
    return 1;
  }
  return 0; 
} 