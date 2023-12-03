#include <cctype>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

std::pair<int, int> solution(std::string line, int game_id) {
  bool is_possible = true;
  int game_power = 1;
  std::string cubes = line.substr(line.find(":") + 2);

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

  std::string color = "";
  std::string amount = "";
  int parsed_amount = 0;
  bool found_color = false;

  for (int i = 0; i < cubes.size(); i++) {
    found_color = false;
    if (std::isdigit(cubes[i])) {
      amount += cubes[i];
    } else if (cubes[i] == ' ') {
      parsed_amount = amount.size() > 1 ? std::stoi(amount) : (amount[0] - '0');

      if (color.size() > 0) {
        found_color = true;
      }

      for (const auto it : rules) {
        if (color.find(it.first) != std::string::npos) {
          color = it.first;
        }
      }

      if (found_color) {
        amounts[color] = std::max(amounts[color], parsed_amount);

        if (parsed_amount > rules[color]) {
          is_possible = false;
        }
      }

      color = "";
      amount = found_color ? "" : amount;
    } else {
      color += cubes[i];
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

  int i = 0;
  while (std::getline(file, line)) {
    part_one_total += solution(line, i).first;
    part_two_total += solution(line, i).second;
    i++;
  }

  std::cout << "Part 1: " << part_one_total << "\n";
  std::cout << "Part 2: " << part_two_total << "\n";
}
