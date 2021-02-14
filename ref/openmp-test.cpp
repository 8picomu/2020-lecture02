#include <omp.h>

int f(int i, int j) { return i + j; }

int main() {
#pragma omp parallel for schedule(dynamic, 1)
  for (int i = 0; i < 100000; ++i) {
    for (int j = 0; j < 100000; ++j) {
      f(i, j);
    }
  }

  return 0;
}