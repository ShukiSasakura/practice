#!/usr/bin/env python3

import pandas as pd
import matplotlib
#matplotlib.use('TkAgg')
#from matplotlib import pyplot as plt
import sys
import re

#clock = 3400000000
csv_file = sys.argv[1]
df = pd.read_csv(csv_file, skipinitialspace=True)

#print((df.iloc[-1] - df.iloc[0]) / clock)
tsc = (df.iloc[-1]["send times"] - df.iloc[0]["receive times"]) / 1000000000
result = 100000 / tsc
print(f'result: {result}')
