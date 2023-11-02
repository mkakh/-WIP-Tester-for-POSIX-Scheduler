#!/bin/env python

N = 3
TIMEOUT=300

import os
import shutil
import subprocess
import time

def clear():
    subprocess.run('./clear.sh', shell=True)
    time.sleep(3)

def is_duplicate(log1, log2):
    if not os.path.exists(log1) or not os.path.exists(log2):
        return False
    # If each argument points to the identical file, it should not be removed.
    if log1 == log2:
        return False
    with open(log1) as f1, open(log2) as f2:
        lines1 = list(filter(lambda line: "time:" not in line and "used:" not in line, f1.readlines()))
        lines2 = list(filter(lambda line: "time:" not in line and "used:" not in line, f2.readlines()))
        return lines1 == lines2

def get_time(log):
    if not os.path.exists(log):
        return "TIMEOUT"
    else:
        with open(log) as f:
            for line in f:
                if "time:" in line:
                    return line.split()[1]

def get_machine_info():
    subprocess.run('./get_machine_info.sh', shell=True)
    time.sleep(3)

# cd to the directory of the script
os.chdir(os.path.dirname(os.path.abspath(__file__))) 

# clear log files
for f in os.listdir('logs'):
    if f.startswith('log') or f == 'summary' or f == 'dmesg.log' or f == 'machine_info':
        os.remove(os.path.join('logs', f))

# init the list for summary
summary = [[0, []] for _ in range(N)]

get_machine_info()

subprocess.run('cargo build', shell=True)
subprocess.run('sudo dmesg -C', shell=True)

idx = 0 
for _ in range(N):
    clear()
    subprocess.run('sudo dmesg -C', shell=True)

    # run command and write output to log file using tee command
    for i in range(3):
        try:
            subprocess.run(f'cargo run 2>&1 | tee logs/log{idx}', shell=True, timeout=TIMEOUT)
            subprocess.run(f'sudo dmesg > logs/dmesg.log{idx}', shell=True)
            break
        except subprocess.TimeoutExpired as e:
            clear()
            # write timeout error to log file
            with open(f'logs/log{idx}', 'a') as f:
                f.write(f"Terminated by Python script (TIMEOUT: {TIMEOUT} sec)\n")
            print(f"Terminated by Python script (TIMEOUT: {TIMEOUT} sec)")
            # rename log file to log{idx}_timeout{i}
            shutil.move(f'logs/log{idx}', f'logs/log{idx}_timeout{i}')
            subprocess.run(f'sudo dmesg > logs/dmesg.log{idx}_timeout{i}', shell=True)
            subprocess.run('sudo dmesg -C', shell=True)

    
    # check if log file is duplicate
    is_dup = False
    for i in range(idx):
        if is_duplicate(f'logs/log{idx}', f'logs/log{i}'):
            summary[i][0] += 1
            summary[i][1].append(get_time(f'logs/log{idx}'))
            is_dup = True
            # remove duplicate log file
            os.remove(f'logs/log{idx}')
            break
    
    # if not duplicate, increment summary count
    if not is_dup:
        summary[idx][0] = 1
        summary[idx][1].append(get_time(f'logs/log{idx}'))

        # increment index    
        idx += 1

# write and show the summary
with open('logs/summary', 'w') as f:
    for i in range(len(summary)):
        if summary[i][0] == 0:
            break
        print(f'log{i}: {summary[i][0]} times')
        print(", ".join(summary[i][1]))
        f.write(f'log{i}: {summary[i][0]} times\n') 
        f.write(", ".join(summary[i][1]))
        f.write('\n')

