import json
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D

networks = [
    "alexnet",
    "googlenet",
    "mobilenet",
    "resnet50",
    "squeezenet"
]


colormap = {
    "G-L-B": 'b',
    "G-B-L": 'g',
    "B-G-L": 'r',
    "B-L-G": 'm',
    "L-B-G": 'c',
    "L-G-B": 'y',
}


def make_graph(data, name, used_component):

    for i, dpoint in enumerate(data):
        component = dpoint["component"]  # "L", "B" or "G"
        # if component != used_component:
        #     continue
        partition1, partition2 = dpoint['partition_point1'], dpoint['partition_point2']

        stage = ["stage_one", "stage_two"][dpoint["stage"] - 1]

        duration = partition2 - partition1

        if duration > 4:
            continue

        itime = dpoint["result"][stage]["inference_time"] / duration

        halfIndex = 0 if i < len(data) // 2 else 1

        order = None
        if component == "L":
            order = ["G-L-B", "B-L-G"][halfIndex]
        elif component == "B":
            order = ["G-B-L", "L-B-G"][halfIndex]
        elif component == "G":
            order = ["B-G-L", "L-G-B"][halfIndex]

        plt.semilogy([partition1 + 0.1, partition2 - 0.1], [itime, itime],
                     #  color=colors[(duration) %
                     #               len(colors)],
                     color=colormap[order],
                     linewidth=2 / duration,
                     label=order)

    plt.title(f"{name} - all")
    # plt.title(f"{name} - {used_component}")
    plt.xlabel("Stage")
    plt.ylabel("Inference time")

    custom_lines = [Line2D([0], [0], color=color, lw=4)
                    for color in colormap.values()]

    plt.legend(custom_lines, colormap.keys())
    plt.savefig(
        f"./output/performance-benchmarks/stages-{name}-all-full.png")
    # f"./output/performance-benchmarks/stages-{name}-{used_component}-full.png")
    plt.close()


def main():
    data_by_network = {}
    for network in networks:
        with open(f"./output/performance-benchmarks/{network}-stages.json") as file:
            data_by_network[network] = json.loads(file.read())

    for network in networks:
        make_graph(data_by_network[network], network, "L")


main()
