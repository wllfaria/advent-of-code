#include <cctype>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

std::pair<int, int> solution(std::string line, int game_id) {
  bool is_possible = true;
  std::string cubes = line.substr(line.find(":") + 2);
  std::string amount_string = "";
  int max_amounts[3]{0, 0, 0};

  for (int i = 0; i < cubes.size(); i++) {
    if (cubes[i] == 'r') {
      int amount = amount_string.size() > 1 ? std::stoi(amount_string)
                                            : (amount_string[0] - '0');
      if (amount > 12) {
        is_possible = false;
      }

      max_amounts[0] = std::max(max_amounts[0], amount);
      amount_string = "";
    } else if (cubes[i] == 'g') {
      int amount = amount_string.size() > 1 ? std::stoi(amount_string)
                                            : (amount_string[0] - '0');
      if (amount > 13) {
        is_possible = false;
      }

      max_amounts[1] = std::max(max_amounts[1], amount);
      amount_string = "";
    } else if (cubes[i] == 'b') {
      int amount = amount_string.size() > 1 ? std::stoi(amount_string)
                                            : (amount_string[0] - '0');
      if (amount > 14) {
        is_possible = false;
      }

      max_amounts[2] = std::max(max_amounts[2], amount);
      amount_string = "";
    } else if (std::isdigit(cubes[i])) {
      amount_string += cubes[i];
    }
  }

  int game_power = max_amounts[0] * max_amounts[1] * max_amounts[2];
  return std::make_pair(is_possible ? game_id : 0, game_power);
}

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;

  std::pair<int, int> total{0, 0};

  int i = 1;
  while (std::getline(file, line)) {
    auto result = solution(line, i);
    total.first += result.first;
    total.second += result.second;
    i++;
  }

  std::cout << "Part 1: " << total.first << "\n";
  std::cout << "Part 2: " << total.second << "\n";
}
