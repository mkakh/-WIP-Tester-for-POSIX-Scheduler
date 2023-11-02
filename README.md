# [WIP] POSIX Scheduler Tester

This program is currently not stable yet.

We have a plan to release the stable version later.

## Q&A

### The debug print for the current states has failed (TIMEOUT)
When the test program has failed, it tried to show the current states of threads. However, sometimes, they fail to get the states using procfs.
In this case, the program shows the message. If you want to know the states, please run the test program again.
