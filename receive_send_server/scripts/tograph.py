#!/usr/bin/env python3

from matplotlib import pyplot as plt
from matplotlib import ticker
import numpy as np
import pypdf
import datetime

message_size = [1, 2, 4]
throughputs = {
    'NativeTcp': (583415, 509817, 380257),
    'WasmTcp': (197919, 184659, 170287),
}
#native_throughput = [1166830, 1019634, 760514]
#wasmedge_throughput = [395837, 369317, 340573]

x = np.arange(len(message_size))
width = 0.4
multiplier = 0

fig, ax = plt.subplots(layout='constrained')

# 棒グラフのプロット
for implement, throughput in throughputs.items():
    offset = width * multiplier
    rects = ax.bar(x + offset, throughput, width, label=implement)
    ax.bar_label(rects, fmt='{:,.0f}', padding=3)
    multiplier += 1

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_xlabel('Message Size (KB)')
ax.set_ylabel('Throughput (msgs/s)')
ax.set_xticks(x + (width / 2), message_size)
ax.legend(loc='upper right', ncols=2)
ax.set_ylim(0, max(throughputs['NativeTcp']) * 1.1)
plt.ticklabel_format(style='plain',axis='y')


# 棒グラフのプロット
# plt.bar(message_size, native_throughput, label='native')
# plt.bar(message_size, wasmedge_throughput, label='wasm')

# グラフの軸ラベル
# plt.xlabel('Message Size(KB)')
# plt.ylabel('Throughput(msgs/s)')

# グラフの pdf 出力
date = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
file_name = f'{date}.pdf'

plt.savefig(file_name)
plt.close()
