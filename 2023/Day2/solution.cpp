#include <cctype>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

std::pair<int, int> solution(std::string line) {
  int game_id = std::stoi(line.substr(line.find(" ") + 1, line.find(":") - 5));
  bool is_possible = true;
  int game_power = 1;
  std::string cubes = line.substr(line.find(":") + 2);
  std::istringstream iss(cubes);
  std::vector<std::string> tokens;
  std::string token;

  std::unordered_map<std::string, int> rules{
      {"red", 12},
      {"green", 13},
      {"blue", 14},
  };

  std::unordered_map<std::string, int> amounts{
      {"red", 0},
      {"green", 0},
      {"blue", 0},
  };

  while (std::getline(iss, token, ' '))
    tokens.push_back(token);

  for (int i = 0; i < tokens.size(); i += 2) {
    int amount = std::stoi(tokens[i]);
    std::string color = tokens[i + 1];

    for (const auto it : rules) {
      if (color.find(it.first) != std::string::npos) {
        color = it.first;
      }
    }

    amounts[color] = std::max(amounts[color], amount);

    if (amount > rules[color]) {
      is_possible = false;
    }
  }

  for (const auto it : amounts) {
    game_power *= it.second;
  }

  return std::make_pair(is_possible ? game_id : 0, game_power);
}

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;

  int part_one_total = 0;
  int part_two_total = 0;

  while (std::getline(file, line)) {
    part_one_total += solution(line).first;
    part_two_total += solution(line).second;
  }

  std::cout << "Part 1: " << part_one_total << "\n";
  std::cout << "Part 2: " << part_two_total << "\n";
}
