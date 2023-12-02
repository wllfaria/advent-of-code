#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

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

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;

  int part_one_total = 0;

  while (std::getline(file, line)) {
    part_one_total += part_one(line);
  }

  std::cout << part_one_total << "\n";
}
