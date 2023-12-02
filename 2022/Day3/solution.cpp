#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int part_one(std::string line) {
  int size = line.size();
  int mid = size / 2;
  int priority = -1;

  std::unordered_map<int, int> chars;

  std::string compartment_one = line.substr(0, mid);
  std::string compartment_two = line.substr(mid);

  for (const auto c : compartment_one) {
    int priority_one = static_cast<int>(c);
    priority_one = priority_one < 97 ? priority_one - 38 : priority_one - 96;
    chars[priority_one]++;
  }

  for (const auto c : compartment_two) {
    int priority_two = static_cast<int>(c);
    priority_two = priority_two < 97 ? priority_two - 38 : priority_two - 96;

    if (chars[priority_two] > 0) {
      priority = priority_two;
      break;
    }
  }

  return priority;
}

int part_two(std::vector<std::string> lines) {
  std::vector<std::vector<std::string>> groups;
  int priority = 0;
  int group = -1;

  for (int i = 0; i < lines.size(); i++) {
    if (i % 3 == 0) {
      group++;
      groups.push_back({});
    }
    groups[group].push_back(lines[i]);
  }

  for (const auto group : groups) {
    std::string group_a = group[0];
    std::string group_b = group[1];
    std::string group_c = group[2];

    std::unordered_map<int, int> chars;

    for (const auto c : group_a) {
      int priority_a = static_cast<int>(c);
      priority_a = priority_a < 97 ? priority_a - 38 : priority_a - 96;

      if (chars[priority_a] != 0) {
        continue;
      }

      chars[priority_a] = 1;
    }

    for (const auto c : group_b) {
      int priority_b = static_cast<int>(c);
      priority_b = priority_b < 97 ? priority_b - 38 : priority_b - 96;

      if (chars[priority_b] != 1) {
        continue;
      }

      chars[priority_b]++;
    }

    for (const auto c : group_c) {
      int priority_c = static_cast<int>(c);
      priority_c = priority_c < 97 ? priority_c - 38 : priority_c - 96;

      if (chars[priority_c] != 2) {
        continue;
      }

      chars[priority_c]++;
    }

    for (const auto p : chars) {
      if (p.second == 3) {
        priority += p.first;
        break;
      }
    }
  }

  return priority;
}

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;
  std::vector<std::string> lines;

  int part_one_total = 0;
  int part_two_total = 0;

  while (std::getline(file, line)) {
    part_one_total += part_one(line);
    lines.push_back(line);
  }

  part_two_total = part_two(lines);

  std::cout << part_one_total << "\n";
  std::cout << part_two_total << "\n";
}
