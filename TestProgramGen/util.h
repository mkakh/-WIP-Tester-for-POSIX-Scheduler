#ifndef UTIL_H
#define UTIL_H

/******************/
/* TEST SETTINGS */
/******************/
#define BUSY_LOOP_COUNT (10000000)
#define ALRM_TIME (6)
/****************/

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include "mapping.h"
#include <algorithm>
#include <cassert>
#include <csignal>
#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <pthread.h>
#include <sched.h>
#include <string>
#include <unistd.h>
#include <vector>

#define UNKNOWN (-1)
#define READY 0
#define RUNNING 1
#define WAITING 2
#define TERMINATED 3

using ull = unsigned long long;
using ll = long long;

typedef struct {
  std::vector<int> thread_state;
} exp_val_t;

typedef struct {
  std::string func_name;
  std::vector<int> arg;
  int invoker;
  exp_val_t exp;
} test_t;

/************************************/
/*  テストパラメータ・テストケース  */
/************************************/
#define NUM_CORES 2

// mapping from real tid to formalized tid
extern mapping tid_mapping;

/******************/
/* グローバル変数 */
/******************/

// the index of execution sequence
extern volatile unsigned int seq_idx;

// for initialization
extern volatile bool init_end;

extern ull expected_errno;
extern pthread_mutex_t mutex;

extern test_t test_seq[];
extern size_t test_seq_size;

// async-signal-safe
extern void debug_print(const char *msg);

// not async-signal-safe
extern void debug_printf(const char *format, ...);

// not async-signal-safe
extern void debug_print_mapping(const mapping &m);
#endif
