#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "dangling_danger_cpp" ? 0 : 1;
}
