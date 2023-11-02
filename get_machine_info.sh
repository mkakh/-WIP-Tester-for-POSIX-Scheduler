#!/bin/sh

chdir $(dirname "$0")

echo "[N_CORE]" > logs/machine_info
nproc 2>&1 >> logs/machine_info
echo "" >> logs/machine_info
echo "[Memory]" >> logs/machine_info
free -h 2>&1 >> logs/machine_info
echo "" >> logs/machine_info
echo "[Disk]" >> logs/machine_info
df -h 2>&1 >> logs/machine_info
echo "" >> logs/machine_info
echo "[Kernel]" >> logs/machine_info
uname -a 2>&1 >> logs/machine_info
echo "" >> logs/machine_info
echo "[Git]" >> logs/machine_info
git log | head -n1 2>&1 >> logs/machine_info
echo "" >> logs/machine_info
git diff 2>&1 >> logs/machine_info
