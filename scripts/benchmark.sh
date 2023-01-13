# Change strategy in main.rs to BenchmarkPerformanceStrategy

GRAPH=('alexnet' 'googlenet' 'mobilenet' 'resnet50' 'squeezenet')
PARTITION_PARTS=(8 13 28 18 19)

for (( c=0; c<=4; c++ ))
do 
    G=${GRAPH[$c]}
    PP=${PARTITION_PARTS[$c]}
    time ./scripts/run.sh graph_${G}_all_pipe_sync ${PP} 5 400 $@ && adb shell cat /data/local/Working_dir/benchmark.json > ./output/performance-benchmarks/${G}.json
    print "Done with ${G}"
done
