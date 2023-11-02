#ifndef MAPPING_H
#define MAPPING_H
#include <unordered_map>
class mapping : public std::unordered_map<int, int> {
public:
  int get(const int key);
  int get_key(const int value);
};
#endif
