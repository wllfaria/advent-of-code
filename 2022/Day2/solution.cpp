#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int main(int argc, char *argv[]) {
  std::ifstream file(argv[1]);
  std::string line;
  int total_score = 0;
  std::unordered_map<int, int> wins{
      {1, 3},
      {2, 1},
      {3, 2},
  };
  std::unordered_map<char, int> points{
      {'A', 1}, {'B', 2}, {'C', 3}, {'X', 1}, {'Y', 2}, {'Z', 3},
  };

  while (std::getline(file, line)) {
    int elf_move = points[line[0]];
    int my_move = points[line[2]];

    total_score += my_move;

    if (elf_move == my_move) {
      total_score += 3;
    } else if (wins[my_move] == elf_move) {
      total_score += 6;
    }
  }

  std::cout << total_score << "\n";
}
