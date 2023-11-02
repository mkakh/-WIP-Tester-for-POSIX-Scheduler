#include "checker.h"
#include "impl.h"
#include "util.h"

/******************/
/* グローバル変数 */
/******************/
// the index of execution sequence
volatile unsigned int seq_idx;

// for initialization
volatile bool init_end;

ull expected_errno;
pthread_mutex_t mutex;

/******************/
/*  テスト関数群  */
/******************/
void *thread(void *a_tid);
void init_sigalrm(void (*handler)(int));

void alarm_handler2(int signum) {
  debug_print("The debug print for the current states has failed (TIMEOUT).\n");
  _exit(EXIT_FAILURE);
}

void alarm_handler(int signum) {
  // the program encounters a deadlock
  debug_print("The test program has failed (TIMEOUT).\n");
  init_sigalrm(alarm_handler2);
  alarm(ALRM_TIME);
  // print the current states for debugging (not async-signal-safe)
  if (seq_idx != 0 && test_seq[seq_idx - 1].func_name == "PthreadExit") {
    debug_print_current_st(test_seq[seq_idx - 1].exp);
  } else {
    debug_print_current_st(test_seq[seq_idx].exp);
  }
  _exit(EXIT_FAILURE);
}

void check_finish() {
  if (seq_idx >= test_seq_size) {
    unsigned int idx = seq_idx;
    if (idx != 0 && test_seq[idx - 1].func_name == "PthreadExit") {
      // Since after pthread_exit, the thread is terminated and not chech the
      // expected values, so we need to check the expected values here
      while (!checker(test_seq[idx - 1].exp))
        ;
    }
    debug_print("The test program has finished successfully.\n");
    _exit(EXIT_SUCCESS);
  }
}

void thread_context(const ull tid) {
  unsigned int idx = seq_idx;
  // if the idx is out of range, then return.
  if (idx >= test_seq_size) {
    return;
  }
  const std::string func_name{test_seq[idx].func_name};
  const std::vector<int> arg{test_seq[idx].arg};
  const int invoker = test_seq[idx].invoker;

  if (tid == invoker) {
    debug_print_mem_usage();
    if (idx != 0 && test_seq[idx - 1].func_name == "PthreadExit") {
      // Since after pthread_exit, the thread is terminated and not chech the
      // expected values, so we need to check the expected values here
      while (!checker(test_seq[idx - 1].exp))
        ;
    }
    if (func_name == "PthreadCreate") {
      debug_printf("%d: PthreadCreate[%d] (TID: %d)\n", idx, arg[0], invoker);
      impl_pthread_create(tid, arg[0]);
    } else if (func_name == "PthreadExit") {
      debug_printf("%d: PthreadExit[] (TID: %d)\n", seq_idx, invoker);
      // Since after pthread_exit, the thread is terminated, so we need to incr
      // the seq_idx here.
      seq_idx++;
      impl_pthread_exit(tid);
    } else if (func_name == "PthreadMutexLock") {
      impl_pthread_mutex_lock(tid);
    } else if (func_name == "PthreadMutexTrylock") {
      impl_pthread_mutex_try_lock(tid);
    } else if (func_name == "PthreadMutexUnlock") {
      impl_pthread_mutex_unlock(tid);
    }
    while (!checker(test_seq[idx].exp))
      ;
    seq_idx++;
    alarm(ALRM_TIME);
  }
}

void *thread(void *a_tid) {
  const ull tid = (ull)a_tid;
  tid_mapping.emplace(tid, (int)gettid());
  init_end = true;

  while (1) {
    thread_context(tid);
    check_finish();
  }
}

void init_sigalrm(void (*handler)(int)) {
  struct sigaction sa;
  sa.sa_handler = handler;
  sigemptyset(&sa.sa_mask);
  sa.sa_flags = 0;
  sigaction(SIGALRM, &sa, NULL);
}

/******************/
/*   メイン関数   */
/******************/

int main() {
  debug_print_mem_usage();
  init_sigalrm(alarm_handler);
  alarm(ALRM_TIME);
  debug_print("[DEBUG] alarm init done\n");
  struct sched_param param;
  param.sched_priority = sched_get_priority_min(SCHED_FIFO);
  if (sched_setscheduler(0, SCHED_FIFO, &param) != 0) {
    perror("SET POLICY FAILED\n");
    exit(EXIT_FAILURE);
  }
  debug_print("[DEBUG] set policy to SCHED_FIFO\n");

  pthread_mutexattr_t mtx_attr;
  pthread_mutexattr_init(&mtx_attr);
  // pthread_mutexattr_setrobust(&mtx_attr, PTHREAD_MUTEX_ROBUST);
  //  pthread_mutexattr_settype(&mtx_attr, PTHREAD_MUTEX_ERRORCHECK);
  pthread_mutex_init(&mutex, &mtx_attr);
  debug_print("[DEBUG] mutex init done\n");
  tid_mapping.emplace(0, (int)gettid());
  while (1) {
    thread_context(0);
    check_finish();
  }
}
