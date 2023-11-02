#ifndef CHECKER_H
#define CHECKER_H
#include "util.h"
int checker(const exp_val_t &exp);
void debug_print_current_st(const exp_val_t &exp);
void debug_print_mem_usage();
#endif
