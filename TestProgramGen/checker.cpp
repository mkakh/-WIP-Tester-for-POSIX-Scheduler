#include "util.h"
#define STAT_TID (0)
#define STAT_STATE (2)
#define STAT_UTIME (13)
#define STAT_STIME (14)
#define STAT_PRIORITY (17)
#define STAT_VSIZE (22)
#define STAT_RSS (23)

// mapping from real tid to formalized tid
mapping tid_mapping;

std::vector<std::string> split_whitespaces(const std::string &input) {
  std::vector<std::string> parts;
  std::stringstream ss(input);
  std::string part;
  while (getline(ss, part, ' ')) {
    parts.push_back(part);
  }
  return parts;
}

// returns -1 when unknown state is given
// the definition of states follows man 5 proc in Linux
int parse_state(char ch) {
  switch (ch) {
  // Running
  case 'R':
    return RUNNING;

    // Paging (only before Linux 2.6.0)
    // case 'W':
    // Waiting in uninterruptible disk sleep
  case 'D':
  // Sleeping in an interruptible wait
  case 'S':
  // Idle (Linux 4.14 onward)
  case 'I':
  // Stopped (on a signal) or (before Linux 2.6.33) trace stopped
  case 'T':
  // Tracing stop (Linux 2.6.33 onward)
  case 't':
  // Parked (Linux 3.9 to 3.13 only)
  case 'P':
    return WAITING;

  // Waking (Linux 2.6.33 to 3.13 only)
  case 'W':
  // Wakekill (Linux 2.6.33 to 3.13 only)
  case 'K':
    return READY;

  // Zombie
  case 'Z':
  // Dead (from Linux 2.6.0 onward)
  case 'X':
  // Dead (Linux 2.6.33 to 3.13 only)
  case 'x':
    return TERMINATED;

  default:
    return -1;
  }
}

std::vector<std::string> get_stat(const std::filesystem::directory_entry &ent) {
  static int warn_flag = 0;
  std::string path = ent.path().string() + "/stat";
  std::vector<std::string> strs{};
  while (strs.size() < STAT_RSS) {
    strs.clear();
    std::ifstream file(path);
    if (file) {
      warn_flag = 0;
      std::string line;
      getline(file, line);
      strs = split_whitespaces(line);
    } else {
      if (!warn_flag) {
        warn_flag = 1;
        debug_printf("[WARN] Failed to open %s\n", path.c_str());
      }
    }
  }
  return strs;
}

std::vector<int> check_running(std::vector<std::vector<std::string>> &stats) {
  std::vector<int> v_is_running(stats.size(), 0);
  std::vector<int> v_time(stats.size(), 0);

  // Linux does not distinguish Ready and Running
  // If the task is actually running, the utime should be increased
  for (int i = 0; i < stats.size(); i++) {
    if (stats[i].size() > STAT_STIME) {
      v_time[i] = stoi(stats[i][STAT_UTIME] + stats[i][STAT_STIME]);
    } else {
      const int big_num = 1001001001;
    }
  }

  // busy loop
  for (long long int i = 0; i < BUSY_LOOP_COUNT; i++)
    ;

  for (int i = 0; i < stats.size(); i++) {
    const auto stat = stats[i];
    if (stat.size() > STAT_STATE && stat[STAT_STATE].size() > 0 &&
        parse_state(stat[STAT_STATE][0]) == RUNNING) {
      std::vector<std::string> stat2 =
          get_stat(std::filesystem::directory_entry("/proc/self/task/" +
                                                    stat[STAT_TID]));
      int time2 = stat2.size() > STAT_STIME
                      ? stoi(stat2[STAT_UTIME] + stat2[STAT_STIME])
                      : 0;
      v_is_running[i] = time2 > v_time[i];
    }
  }
  return v_is_running;
}

mapping get_states(std::vector<std::vector<std::string>> &stats) {
  std::vector<int> v_is_running = check_running(stats);
  mapping tid2realstate = {};
  for (int i = 0; i < stats.size(); i++) {
    if (v_is_running[i]) {
      tid2realstate.emplace(i, RUNNING);
    } else {
      if (stats[i].size() > STAT_STATE && stats[i][STAT_STATE].size() > 0) {
        int parsed_st = parse_state(stats[i][STAT_STATE][0]);
        // the ready task is appeared as R in stat
        if (parsed_st == RUNNING) {
          tid2realstate.emplace(i, READY);
        } else {
          tid2realstate.emplace(i, parsed_st);
        }
      } else {
        tid2realstate.emplace(i, TERMINATED);
      }
    }
  }
  return tid2realstate;
}

// int is_running(std::vector<std::string> &stat) {
//   if (parse_state(stat[STAT_STATE][0]) == RUNNING) {
//     // Linux does not distinguish Ready and Running
//     // If the task is actually running, the utime should be increased
//     int time = stoi(stat[STAT_UTIME] + stat[STAT_STIME]);
//     // busy loop
//     for (long long int i = 0; i < BUSY_LOOP_COUNT; i++)
//       ;
//     std::vector<std::string> stat2 = get_stat(
//         std::filesystem::directory_entry("/proc/self/task/" +
//         stat[STAT_TID]));
//     int time2 = stoi(stat2[STAT_UTIME] + stat2[STAT_STIME]);
//     return time2 > time;
//   }
//   return 0;
// }

// int get_state(std::vector<std::string> &stat) {
//   if (is_running(stat)) {
//     return RUNNING;
//   }
//
//   int parsed_st = parse_state(stat[STAT_STATE][0]);
//   // the ready task is appeared as R in stat
//   if (parsed_st == RUNNING) {
//     return READY;
//   } else {
//     return parsed_st;
//   }
// }

// int check_state(std::vector<std::string> &stat, int expected_state) {
//   return get_state(stat) == expected_state;
// }

int check_priority(std::vector<std::string> &stat, int expected_priority) {
  // The priority of a task under a real-time scheduling policy is a number
  // obtained by negating the value from the stat file and subtracting 1,
  // according to the proc man page.
  int priority = stoi(stat[STAT_PRIORITY]) * -1 - 1;
  if (priority == expected_priority) {
    return 1;
  } else {
    return 0;
  }
}

int gettid(std::vector<std::string> &stat) { return stoi(stat[STAT_TID]); }

std::string st_display(const int st) {
  switch (st) {
  case -1:
    return "Unknown";
  case 0:
    return "Ready";
  case 1:
    return "Running";
  case 2:
    return "Waiting";
  case 3:
    return "Terminated";
  default:
    return "Unknown";
  }
}

int is_valid_expected_value(const mapping &m) {
  /* the number of running threads should be the same as the number of the cores
   * number of cores */

  // count the number of running threads
  int running_count = 0;
  for (const auto &[tid, st] : m) {
    if (st == RUNNING) {
      running_count++;
    }
  }

  int num_threads = m.size();

  if (num_threads < NUM_CORES) {
    return running_count == num_threads;
  } else {
    return running_count == NUM_CORES;
  }
}

int checker(const exp_val_t &exp) {
  mapping tid2realstate = {};
  std::vector<std::vector<std::string>> tid2stat = {};
  do {
    tid2realstate.clear();
    tid2stat.clear();
    // busy loop
    for (long long int i = 0; i < BUSY_LOOP_COUNT; i++)
      ;
    for (const auto &entry :
         std::filesystem::directory_iterator("/proc/self/task")) {
      std::vector<std::string> stat = get_stat(entry);
      int tid;

      while ((tid = tid_mapping.get_key(gettid(stat))) == -1)
        ;
      if (tid2stat.size() <= tid) {
        tid2stat.resize(tid + 1);
      }
      tid2stat[tid] = stat;
    }
    tid2realstate = get_states(tid2stat);
  } while (!is_valid_expected_value(tid2realstate));

  for (const auto &[tid, realstate] : tid2realstate) {
    if (exp.thread_state.size() < tid || realstate != exp.thread_state[tid]) {
      return 0;
    }
  }

  return 1;
}

/*********************************/
/****** functions for debug ******/
/*********************************/

void debug_print_mem_usage() {
  static int warn_flag = 0;
  std::vector<std::string> str =
      get_stat(std::filesystem::directory_entry("/proc/self"));
  if (str.size() > STAT_RSS) {
    debug_printf("[DEBUG] Memory Usage (RSS) : %s KB\n", str[STAT_RSS].c_str());
  }
}

// the mapping is from (non-real) tid to real state
void debug_print_st_mapping(const mapping &m, const exp_val_t &exp) {
  int i = 1;
  for (const auto &[tid, st] : m) {
    if (tid == -1) {
      debug_printf("[%d]: TID ?: %s (expected: unknown)\n", i++,
                   st_display(st).c_str());
    } else {
      int exp_state;
      try {
        exp_state = exp.thread_state.at(tid);
        debug_printf("[%d]: TID %d: %s (expected: %s)\n", i++, tid,
                     st_display(st).c_str(), st_display(exp_state).c_str());
      } catch (std::out_of_range &e) {
        debug_printf("[ERROR] TID %d is not found in the expected state\n",
                     tid);
        debug_printf("[%d]: TID %d: %s (expected: ?)\n", i++, tid,
                     st_display(st).c_str());
      }
    }
  }
}

void debug_print_current_st(const exp_val_t &exp) {
  std::vector<std::vector<std::string>> tid2stat = {};
  mapping tid2realstate = {};
  do {
    tid2realstate.clear();
    tid2stat.clear();
    for (const auto &entry :
         std::filesystem::directory_iterator("/proc/self/task")) {
      std::vector<std::string> stat = get_stat(entry);
      int tid;

      while ((tid = tid_mapping.get_key(gettid(stat))) == -1)
        ;

      if (tid2stat.size() <= tid) {
        tid2stat.resize(tid + 1);
      }
      tid2stat[tid] = stat;
    }
    tid2realstate = get_states(tid2stat);
  } while (!is_valid_expected_value(tid2realstate));

  debug_print_st_mapping(tid2realstate, exp);
}
