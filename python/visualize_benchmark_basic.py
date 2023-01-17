import json
import matplotlib.pyplot as plt

networks = [
    "alexnet",
    # "googlenet",
    # "mobilenet",
    # "resnet50",
    # "squeezenet"
]


def make_graph(data_by_network, component, network):
    n_frames = 90
    component_data = [x for x in data_by_network[network]
                      if x['component'] == component]

    frequencies = [x['frequency'] for x in component_data]
    fps = [x["result"]["fps"] for x in component_data]

    # min_fps = fps[1]
    # normalized_fps = [x / min_fps for x in fps]

    if component == "G":
        plt.bar([0], fps, label=network)
    else:
        plt.plot(frequencies, fps, label=n_frames)

    plt.legend()
    plt.title(component)
    plt.xlabel("Frequency")
    plt.ylabel("FPS")
    plt.savefig(
        f"./output/performance-benchmarks/{network}-{component}-basic.png")
    plt.close()


def main():
    data_by_network = {}
    for network in networks:
        with open(f"./output/performance-benchmarks/{network}-basic.json") as file:
            data_by_network[network] = json.loads(file.read())

    make_graph(data_by_network, "L", "alexnet")
    make_graph(data_by_network, "B", "alexnet")
    # make_graph(data_by_network, "G")


main()
