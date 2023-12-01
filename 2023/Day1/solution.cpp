#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int part_one(std::string line) {
  int lo = 0;
  int hi = line.size() - 1;
  int found_lo = 0;
  int found_hi = 0;
  int sum = 0;

  while (!found_lo || !found_hi) {
    if (std::isdigit(line[lo]) && !found_lo)
      found_lo = line[lo] - '0';

    if (std::isdigit(line[hi]) && !found_hi)
      found_hi = line[hi] - '0';

    lo++;
    hi--;
  }

  return found_lo * 10 + found_hi;
}

int part_two(std::string line) {
  std::unordered_map<std::string, int> numbers{
      {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5},
      {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9},
  };

  int lo = 0;
  int hi = line.size() - 1;

  int found_hi = 0;
  int found_lo = 0;

  std::string lo_str = "";
  std::string hi_str = "";

  while (!found_hi || !found_lo) {
    if (!std::isdigit(line[lo]) && !found_lo) {
      lo_str += line[lo];
      for (const auto it : numbers) {
        if (lo_str.find(it.first) != std::string::npos) {
          found_lo = it.second;
        }
      }
    } else if (!found_lo && std::isdigit(line[lo])) {
      found_lo = line[lo] - '0';
    }

    if (!std::isdigit(line[hi]) && !found_hi) {
      hi_str.insert(0, 1, line[hi]);
      for (const auto it : numbers) {
        if (hi_str.find(it.first) != std::string::npos) {
          found_hi = it.second;
        }
      }
    } else if (!found_hi && std::isdigit(line[hi])) {
      found_hi = line[hi] - '0';
    }

    lo++;
    hi--;
  }

  return found_lo * 10 + found_hi;
}

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;

  int part_one_total = 0;
  int part_two_total = 0;

  while (std::getline(file, line)) {
    part_one_total += part_one(line);
    part_two_total += part_two(line);
  }

  std::cout << "Part 1: " << part_one_total << "\n";
  std::cout << "Part 2: " << part_two_total << "\n";
}
