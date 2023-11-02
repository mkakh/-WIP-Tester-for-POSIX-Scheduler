#include "util.h"
extern void impl_pthread_create(const int invoker, const int new_prio);
extern void impl_pthread_exit(const int invoker);
extern void impl_pthread_mutex_lock(const int invoker);
extern void impl_pthread_mutex_try_lock(const int invoker);
extern void impl_pthread_mutex_unlock(const int invoker);
