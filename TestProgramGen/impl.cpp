#include "util.h"

extern void *thread(void *);


volatile unsigned int new_tid = 1;

void impl_pthread_create(const int invoker, const int new_prio) {
  pthread_t tid;
  pthread_attr_t attr;
  pthread_attr_init(&attr);
  pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
  pthread_attr_setinheritsched(&attr, PTHREAD_EXPLICIT_SCHED);
  pthread_attr_setschedpolicy(&attr, SCHED_FIFO);
  struct sched_param param;
  param.sched_priority = new_prio;
  pthread_attr_setschedparam(&attr, &param);
  pthread_create(&tid, &attr, thread, (void *)(unsigned long long int)new_tid);
  tid_mapping.emplace(tid, new_tid);
  pthread_attr_destroy(&attr);
  new_tid++;
  /* old impl */
  // const int new_tid{arg[0]};
  // const int new_prio{arg[1]};
  // debug_printf("%u: PthreadCreate(%llu, %d, %d)\n", seq_idx, tid, new_tid,
  //        new_prio);
  // pthread_t tid;
  // pthread_attr_t attr;
  // pthread_attr_init(&attr);
  // pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
  // pthread_attr_setinheritsched(&attr, PTHREAD_EXPLICIT_SCHED);
  // pthread_attr_setschedpolicy(&attr, SCHED_FIFO);

  // struct sched_param param;
  // pthread_attr_getschedparam(&attr, &param);
  // param.sched_priority = new_prio + 1;
  // if ((errno = pthread_attr_setschedparam(&attr, &param))) {
  //   perror("pthread_attr_setschedparam");
  //   exit(EXIT_FAILURE);
  // }

  // init_end = false;
  // if ((expected_errno =
  //          pthread_create(&tid, &attr, thread, (void *)new_tid))) {
  //   perror("pthread_create");
  //   exit(EXIT_FAILURE);
  // }
  //// wait for creating the new thread
  // while (!init_end)
  //   ;
}

void impl_pthread_exit(const int invoker) {
  expected_errno = 0;
  pthread_exit(NULL);
}

void impl_pthread_mutex_lock(const int invoker) {
  debug_printf("%d: PthreadMutexLock(%llu)\n", seq_idx, invoker);
  expected_errno = pthread_mutex_lock(&mutex);
  /* old impl */
  // debug_printf("%llu: PthreadMutexLock(%llu)\n", tc_counter, tid);
  // // incr(&mutex_waiting_counter);
  // expected_errno = pthread_mutex_lock(&mutex);
  // // dec(&mutex_waiting_counter);
}

void impl_pthread_mutex_try_lock(const int invoker) {
  debug_printf("%d: PthreadMutexTrylock(%llu)\n", seq_idx, invoker);
  expected_errno = pthread_mutex_trylock(&mutex);
}

void impl_pthread_mutex_unlock(const int invoker) {
  debug_printf("%d: PthreadMutexUnlock(%llu)\n", seq_idx, invoker);
  expected_errno = pthread_mutex_unlock(&mutex);
}
