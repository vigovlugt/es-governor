import json
import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit

freqs = [
    500000,
    667000,
    1000000,
    1200000,
    1398000,
    1512000,
    1608000,
    1704000,
    1800000
]

amps = [
    0.420,
    0.430,
    0.450,
    0.470,
    0.485,
    0.495,
    0.510,
    0.520,
    0.540
]


# def func(x, a, b, c):
#     return c + a * b**(x/500000)


# popt, pcov = curve_fit(func, freqs, amps)
# print(popt)
# print(func(freqs[0], *popt))
# print(func(freqs[len(freqs) - 1], *popt))


freqsbig = [
    500000,
    667000,
    1000000,
    1200000,
    1398000,
    1512000,
    1608000,
    # 1704000,
    1800000,
    # 1908000,
    # 2016000,
    2100000,
    2208000
]
ampsbig = [
    0.440,
    0.470,
    0.520,
    0.560,
    0.620,
    0.650,
    0.690,
    # ?,
    0.760,
    # ?,
    # ?,
    0.850,
    0.910
]


# popt_B, pcov_B = curve_fit(func, freqsbig, ampsbig,
#                            bounds=([0.3, 0, 1], [0.45, 1, 2]))
# print(popt_B)
# print(func(freqsbig[0], *popt_B))
# print(func(freqsbig[len(freqsbig) - 1], *popt_B))

ndata = None
with open(f"./output/performance-benchmarks/alexnet-basic.json") as file:
    ndata = json.loads(file.read())

littleFps = []
for freq in freqs:
    res = [dp["result"]["fps"]
           for dp in ndata if dp["component"] == "L" and dp["frequency"] == freq]
    if len(res) != 1:
        print("Error, too long")
    else:
        littleFps.append(res[0])


bigFps = []
for freq in freqsbig:
    res = [dp["result"]["fps"]
           for dp in ndata if dp["component"] == "B" and dp["frequency"] == freq]
    if len(res) != 1:
        print("Error, too long")
    else:
        bigFps.append(res[0])


plt.plot(ampsbig, bigFps, label="Big CPU")
plt.plot(amps, littleFps, label="Little CPU")

gpuperf = None
res = [dp["result"]["fps"]
       for dp in ndata if dp["component"] == "G"]
if len(res) == 1:
    gpuperf = res[0]
else:
    print("error")

plt.scatter([0.640], [gpuperf], label="GPU")
plt.axvline(0.380, label="Idle")
plt.xlabel("power (A) on 5V")
# plt.xlabel("clocks")
plt.ylabel("FPS")
plt.title("Performance per amp by component when running alexnet")
plt.legend()

plt.savefig(f"./output/performance-benchmarks/ppa-alexnet.png")
plt.show()
