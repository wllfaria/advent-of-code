#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
  std::ifstream file(argv[1]);
  std::vector<int> elf_calories{0};

  std::string line;

  int i = 0;
  while (std::getline(file, line)) {
    if (line.empty()) {
      i++;
      elf_calories.push_back(0);
      continue;
    }
    elf_calories[i] += std::stoi(line);
  }

  std::sort(elf_calories.begin(), elf_calories.end(), std::greater<int>());
  int sum = std::accumulate(elf_calories.begin(), elf_calories.begin() + 3, 0);

  std::cout << "Part 1: " << elf_calories[0] << "\n";
  std::cout << "Part 2: " << sum << "\n";
}
