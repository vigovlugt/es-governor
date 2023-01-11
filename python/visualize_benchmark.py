import json
import matplotlib.pyplot as plt

networks = [
    "alexnet",
    "googlenet",
    "mobilenet",
    "resnet50",
    "squeezenet"
]


def make_graph(data_by_network, component):
    for network in networks:
        component_data = [x for x in data_by_network[network]
                          if x['component'] == component]

        frequencies = [x['frequency'] for x in component_data]
        fps = [x["result"]["fps"] for x in component_data]

        min_fps = fps[0]
        normalized_fps = [x / min_fps for x in fps]

        if component == "G":
            plt.bar([0], fps, label=network)
        else:
            plt.plot(frequencies, normalized_fps, label=network)

    plt.legend()
    plt.title(component)
    plt.xlabel("Frequency")
    plt.ylabel("FPS")
    plt.savefig(f"./output/performance-benchmarks/{component}.png")
    plt.close()


def main():
    data_by_network = {}
    for network in networks:
        with open(f"./output/performance-benchmarks/{network}.json") as file:
            data_by_network[network] = json.loads(file.read())

    make_graph(data_by_network, "L")
    make_graph(data_by_network, "B")
    make_graph(data_by_network, "G")


main()
