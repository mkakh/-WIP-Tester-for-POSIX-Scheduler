#include "mapping.h"
int mapping::get(const int key) {
  auto it = this->find(key);
  if (it != this->end()) {
    return it->second;
  }
  return -1;
}

// returns the first key found with the given value
int mapping::get_key(const int value) {
  for (const auto &[key, val] : *this) {
    if (val == value)
      return key;
  }
  return -1;
}
