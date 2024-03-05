#!/usr/bin/env python3

import pandas as pd
import matplotlib
#matplotlib.use('TkAgg')
#from matplotlib import pyplot as plt
import sys
import re

#clock = 3400000000
csv_file = sys.argv[1]
num_messages = 100000
df = pd.read_csv(csv_file, skipinitialspace=True)

#print((df.iloc[-1] - df.iloc[0]) / clock)
recv_tsc = (df.iloc[-1]["receive times"] - df.iloc[0]["receive times"]) / 1000000000
send_tsc = (df.iloc[-1]["send times"] - df.iloc[0]["send times"]) / 1000000000
whole_tsc = (df.iloc[-1]["send times"] - df.iloc[0]["receive times"]) / 1000000000

recv_throughput = num_messages / recv_tsc
send_throughput = num_messages / send_tsc
throughput = num_messages / whole_tsc

print(f'recv_throughput: {recv_throughput}')
print(f'send_throughput: {send_throughput}')
print(f'throughput: {throughput}')
