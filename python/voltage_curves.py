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


def func(x, a, b, c):
    return c + a * b**(x/500000)


popt, pcov = curve_fit(func, freqs, amps)
print(popt)
print(func(freqs[0], *popt))
print(func(freqs[len(freqs) - 1], *popt))


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


popt_B, pcov_B = curve_fit(func, freqsbig, ampsbig,
                           bounds=([0.3, 0, 1], [0.45, 1, 2]))
print(popt_B)
print(func(freqsbig[0], *popt_B))
print(func(freqsbig[len(freqsbig) - 1], *popt_B))

# plt.plot(freqsbig, ampsbig)
# plt.plot(freqs, amps)
# # Frequency is nonsense for gpu, amps matter:
# plt.scatter(1450000, 0.640)
# plt.xlabel("clocks")
# plt.ylabel("amps used")
# plt.show()
