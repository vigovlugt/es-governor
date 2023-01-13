import json
import matplotlib.pyplot as plt

networks = [
    # "alexnet",
    "googlenet",
    # "mobilenet",
    # "resnet50",
    # "squeezenet"
]

prop_cycle = plt.rcParams["axes.prop_cycle"]
colors = prop_cycle.by_key()['color']


def make_graph(data, name, used_component):

    for i, dpoint in enumerate(data):
        component = dpoint["component"]  # "L", "B" or "G"
        if component != used_component:
            continue
        partition1, partition2 = dpoint['partition_point1'], dpoint['partition_point2']

        stage = ["stage_one", "stage_two"][dpoint["stage"] - 1]

        duration = partition2 - partition1

        if duration > 4:
            continue

        itime = dpoint["result"][stage]["inference_time"] / duration
        # itime = dpoint["result"][stage]["total_time"] / duration

        color = colors[0 if i < len(data) // 2 else 1]

        plt.semilogy([partition1 + 0.1, partition2 - 0.1], [itime, itime],
                     #  color=colors[(duration) %
                     #               len(colors)],
                     color=color,
                     linewidth=2 / duration)

    plt.title(f"{name} - {used_component}")
    plt.xlabel("Stage")
    plt.ylabel("Inference time")
    plt.savefig(
        f"./output/performance-benchmarks/stages-{name}-{used_component}-full.png")
    plt.close()


def main():
    data_by_network = {}
    for network in networks:
        with open(f"./output/performance-benchmarks/{network}-stages.json") as file:
            data_by_network[network] = json.loads(file.read())

    for network in networks:
        make_graph(data_by_network[network], network, "G")


main()
