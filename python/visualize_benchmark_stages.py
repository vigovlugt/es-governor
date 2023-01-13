import json
import matplotlib.pyplot as plt

networks = [
    "alexnet",
    # "googlenet",
    # "mobilenet",
    # "resnet50",
    # "squeezenet"
]


def make_graph(data_by_network, network):
    for component in ["L", "B", "G"]:
        component_data = [x for x in data_by_network[network]
                          if x['component'] == component]

        stages = ["stage_one", "stage_two"]

        partitions = [x['partition_point1'] for x in component_data]
        itime = [x["result"][stages[x["stage"] - 1]]
                 ["inference_time"] for x in component_data]

        print(
            f"Sum of stages for component {component} in network {network}: {sum(itime)}")

        plt.plot(partitions, itime, label=component)

    plt.legend()
    plt.title(network)
    plt.xlabel("Stage")
    plt.ylabel("Inference time")
    plt.savefig(f"./output/performance-benchmarks/stages-{network}.png")
    plt.close()


def main():
    data_by_network = {}
    for network in networks:
        with open(f"./output/performance-benchmarks/{network}-stage.json") as file:
            data_by_network[network] = json.loads(file.read())

    for network in networks:
        make_graph(data_by_network, "alexnet")


main()
