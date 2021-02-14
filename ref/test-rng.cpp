#include <iostream>

#include "rng.h"

int main() {
  RNG rng;

  for (int i = 0; i < 100; ++i) {
    std::cout << rng.getNext() << std::endl;
  }

  return 0;
}