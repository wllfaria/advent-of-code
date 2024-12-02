#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

int find_matches(std::string card) {
  int sum = 0;
  std::string numbers = card.substr(card.find(':') + 2);
  std::unordered_map<int, int> map;
  bool found_divisor = false;

  std::string number_string = "";
  for (int i = 0; i < numbers.size(); i++) {
    if (!std::isdigit(numbers[i]) && number_string.size() > 0) {
      if (!found_divisor) {
        map[std::stoi(number_string)]++;
      } else if (found_divisor) {
        if (map.find(std::stoi(number_string)) != map.end()) {
          sum++;
        }
      }
      number_string = "";
    } else if (numbers[i] == '|') {
      found_divisor = true;
      continue;
    } else if (std::isdigit(numbers[i])) {
      number_string += numbers[i];
    }
  }

  if (map.find(std::stoi(number_string)) != map.end()) {
    sum++;
  }

  return sum;
}

int part_one(std::string card) {
  int matches = find_matches(card);
  return matches > 1 ? 1 << (matches - 1) : matches;
}

int part_two(std::vector<std::string> *cards) {
  std::unordered_map<int, int> map;
  int current_card_id = 0;
  int total_cards = 0;

  for (int i = 0; i < cards->size(); i++) {
    current_card_id = i + 1;
    std::string card = cards->at(i);
    int matches = find_matches(card);
    for (int j = 1; j <= matches; j++) {
      int copies = map[current_card_id];
      map[current_card_id + j] += 1 + copies;
    }
  }

  for (const auto c : map) {
    total_cards += c.second;
  }

  return total_cards + current_card_id;
}

int main(int argc, char *argv[]) {
  std::fstream file(argv[1]);
  std::string line;
  std::vector<std::string> cards;

  int part_one_total = 0;
  int part_two_total = 0;

  while (std::getline(file, line)) {
    part_one_total += part_one(line);
    cards.push_back(line);
  }

  part_two_total = part_two(&cards);

  std::cout << "Part 1: " << part_one_total << "\n";
  std::cout << "Part 2: " << part_two_total << "\n";
}
