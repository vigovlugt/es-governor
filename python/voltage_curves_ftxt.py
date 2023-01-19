import json
import matplotlib.pyplot as plt

networks = ['alexnet', 'googlenet', 'mobilenet', 'resnet50', 'squeezenet']


def read_value(line):
    return float(line.replace(" FPS", "").replace(" ms", "").split()[-1])


# MODE = 'ppa'
# MODE = 'freqcurve'
# MODE = 'powcurve'
MODE = "freqlat"

labels = {
    "power": "Power (A) on 5V",
    "performance": "FPS",
    "frequency": "Frequency (MHz)",
    "latency": "Latency (ms)"
}


for network in networks:
    lfreqs = []
    lpows = []
    lperfs = []
    llatency = []
    bfreqs = []
    bpows = []
    bperfs = []
    blatency = []
    gpupow = None
    gpuperf = None
    glatency = None
    mixfreqs = []
    mixpows = []
    mixperfs = []
    mixlatency = []

    with open(f"./output/performance-benchmarks/{network}-power-final.txt") as file:
        lines = file.readlines()
        index = 0
        for gm in range(2):
            mixlatency.append(read_value(lines.pop()))
            mixperfs.append(read_value(lines.pop()))
            mixpows.append(read_value(lines.pop()))
            mixfreqs.append(read_value(lines.pop()) // 1000)

        gpulatency = read_value(lines.pop())
        gpuperf = read_value(lines.pop())
        gpupow = read_value(lines.pop())
        lines.pop()

        for b in range(13):
            blatency.append(read_value(lines.pop()))
            bperfs.append(read_value(lines.pop()))
            bpows.append(read_value(lines.pop()))
            bfreqs.append(read_value(lines.pop()) // 1000)

        for l in range(9):
            llatency.append(read_value(lines.pop()))
            lperfs.append(read_value(lines.pop()))
            lpows.append(read_value(lines.pop()))
            lfreqs.append(read_value(lines.pop()) // 1000)

    if MODE == 'ppa':
        plt.plot(bpows, bperfs, label="Big CPU")
        plt.plot(lpows, lperfs, label="Little CPU")

        plt.scatter([gpupow], [gpuperf], label="GPU")
        plt.scatter(mixpows, mixperfs, label="GPU + Big")
        plt.axvline(0.380, label="Idle")
        plt.xlabel(labels["power"])
        plt.ylabel(labels["performance"])
        plt.title(f"Performance per amp by component when running {network}")
    elif MODE == 'freqcurve':
        plt.plot(bfreqs, bperfs, label="Big CPU")
        plt.plot(lfreqs, lperfs, label="Little CPU")

        plt.axhline(gpuperf, label="GPU", alpha=0.4)
        # Might not want this, stretches the chart a lot
        plt.scatter(mixfreqs, mixperfs, label="GPU + Big")
        plt.xlabel(labels["frequency"])
        plt.ylabel(labels["performance"])
        plt.title(
            f"Performance vs frequency by component when running {network}")
    elif MODE == 'powcurve':
        plt.plot(bfreqs, bpows, label="Big CPU")
        plt.plot(lfreqs, lpows, label="Little CPU")

        plt.axhline(gpupow, label="GPU", alpha=0.4)
        plt.scatter(mixfreqs, mixpows, label="GPU + Big")
        plt.axhline(0.380, label="Idle")
        plt.xlabel(labels["frequency"])
        plt.ylabel(labels["power"])
        plt.title(f"Power vs frequency by component when running {network}")
    elif MODE == 'freqlat':
        plt.plot(bfreqs, blatency, label="Big CPU")
        plt.plot(lfreqs, llatency, label="Little CPU")

        plt.axhline(gpulatency, label="GPU", alpha=0.4)
        plt.scatter(mixfreqs, mixlatency, label="GPU + Big")
        plt.xlabel(labels["frequency"])
        plt.ylabel(labels["latency"])
        plt.title(f"Latency by frequency by component when running {network}")

    plt.legend()
    plt.savefig(f"./output/performance-benchmarks/{MODE}-{network}.png")
    # plt.show()
    plt.close()
