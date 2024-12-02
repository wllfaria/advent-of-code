#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

std::string hash(int row, int start, int end) {
  std::string key =
      std::to_string(row) + std::to_string(start) + std::to_string(end);
  return key;
}

std::vector<int> horizontal_scan(std::string line, int col) {
  int l = col - 1;
  int r = col;

  std::string number = "";

  while (std::isdigit(line[l]) || std::isdigit(line[r])) {
    char cl = line[l];
    char cr = line[r];

    if (std::isdigit(cr)) {
      number += cr;
      r++;
    }
    if (std::isdigit(cl)) {
      number.insert(0, 1, cl);
      l--;
    }
  }

  return {r - 1, l + 1, std::stoi(number)};
}

std::vector<int> find_adjacent(int row, int col, std::vector<std::string> parts,
                               std::unordered_set<std::string> *set) {
  int top = std::max(0, row - 1);
  int bottom = std::min(row + 1, static_cast<int>(parts.size()) - 1);
  int left = std::max(0, col - 1);
  int right = std::min(col + 1, static_cast<int>(parts[row].size()) - 1);

  std::vector<int> digits;

  std::vector<std::vector<int>> adjacent{
      {row, left}, {row, right}, {top, col},     {bottom, col},
      {top, left}, {top, right}, {bottom, left}, {bottom, right},
  };

  for (const auto adj : adjacent) {
    if (std::isdigit(parts[adj[0]][adj[1]])) {
      auto result = horizontal_scan(parts[adj[0]], adj[1]);
      std::string key = hash(adj[0], result[1], result[0]);

      if (set->find(key) == set->end()) {
        set->insert(key);
        digits.push_back(result[2]);
      } else {
        continue;
      }
    }
  }

  return digits;
}

std::pair<int, int> solve(std::vector<std::string> parts) {
  int sum = 0;
  int total_ratio = 0;
  std::unordered_set<std::string> set;
  std::vector<int> digits;

  for (int i = 0; i < parts.size(); i++) {
    for (int j = 0; j < parts[i].size(); j++) {
      if (!std::isdigit(parts[i][j]) && parts[i][j] != '.') {
        std::vector<int> adjacent = find_adjacent(i, j, parts, &set);

        if (adjacent.size() == 2) {
          int ratio = adjacent[0] * adjacent[1];
          total_ratio += ratio;
        }

        for (auto const a : adjacent) {
          sum += a;
          digits.push_back(a);
        }
      }
    }
  }

  return std::make_pair(sum, total_ratio);
}

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;

  std::vector<std::string> gear_parts;

  while (std::getline(file, line)) {
    gear_parts.push_back(line);
  }

  auto solution = solve(gear_parts);

  std::cout << "Part 1: " << solution.first << "\n";
  std::cout << "Part 2: " << solution.second << "\n";
}
