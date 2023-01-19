# Change strategy in main.rs to BenchmarkPerformanceStrategy

LFREQS=(500000 667000 1000000 1200000 1398000 1512000 1608000 1704000 1800000)
BFREQS=(500000 667000 1000000 1200000 1398000 1512000 1608000 1704000 1800000 1908000 2016000 2100000 2208000)

GRAPH=('alexnet' 'googlenet' 'mobilenet' 'resnet50' 'squeezenet')
PARTITION_PARTS=(8 13 28 18 19)
MIDWAY_POINTS=(5 6 15 9 8)
MIDWAY_POINTS_2=(5 6 15 10 10)

adb shell "echo performance > /sys/devices/system/cpu/cpufreq/policy2/scaling_governor"
adb shell "echo performance > /sys/devices/system/cpu/cpufreq/policy0/scaling_governor"

adb shell "echo 1 > /sys/class/fan/enable && echo 0 > /sys/class/fan/mode && echo 4 > /sys/class/fan/level"
# adb shell "export LD_LIBRARY_PATH=/data/local/Working_dir"
PREFIX="export LD_LIBRARY_PATH=/data/local/Working_dir && "

for (( c=0; c<=4; c++ ))
do 
    # Set all cores to reasonable high clocks
    # Perf cores
    adb shell "echo 1800000 > /sys/devices/system/cpu/cpufreq/policy2/scaling_max_freq"
    # Efficient cores
    adb shell "echo 1512000 > /sys/devices/system/cpu/cpufreq/policy0/scaling_max_freq"

    G=${GRAPH[$c]}
    PP=${PARTITION_PARTS[$c]}
    
    > ./output/performance-benchmarks/${G}-power.txt
    
    for (( f=0; f<=8; f++))
    do
        FRAMES=$((25+5*${f}))
        FREQ=${LFREQS[f]}
        echo "Running little cores at ${FREQ} KHz for ${FRAMES} frames"
        echo "L : ${FREQ}" >> ./output/performance-benchmarks/${G}-power.txt
        adb shell "echo ${FREQ} > /sys/devices/system/cpu/cpufreq/policy0/scaling_max_freq"
        adb shell "${PREFIX}./data/local/Working_dir/graph_${G}_all_pipe_sync --threads=4 --threads2=2 --n=${FRAMES} --total_cores=6 --partition_point=${PP} --partition_point2=${PP} --order=L-B-G" | grep "Frame\|Running Inference" | tee -a ./output/performance-benchmarks/${G}-power.txt

    done

    for (( f=0; f<=12; f++))
    do
        FRAMES=$((50+20*${f}))
        FREQ=${BFREQS[f]}
        echo "Running big cores at ${FREQ} KHz for ${FRAMES} frames"
        echo "B : ${FREQ}" >> ./output/performance-benchmarks/${G}-power.txt
        adb shell "echo ${FREQ} > /sys/devices/system/cpu/cpufreq/policy2/scaling_max_freq"
        adb shell "${PREFIX}./data/local/Working_dir/graph_${G}_all_pipe_sync --threads=4 --threads2=2 --n=${FRAMES} --total_cores=6 --partition_point=${PP} --partition_point2=${PP} --order=B-L-G" | grep "Frame\|Running Inference" | tee -a ./output/performance-benchmarks/${G}-power.txt
    done

    FRAMES=180
    echo "Running GPU for ${FRAMES} frames"
    echo "G" >> ./output/performance-benchmarks/${G}-power.txt
    adb shell "${PREFIX}./data/local/Working_dir/graph_${G}_all_pipe_sync --threads=4 --threads2=2 --n=${FRAMES} --total_cores=6 --partition_point=${PP} --partition_point2=${PP} --order=G-B-L"  | grep "Frame\|Running Inference" | tee -a ./output/performance-benchmarks/${G}-power.txt

    FRAMES=300
    echo "Running GPU + Big CPU at 2208000 kHz for ${FRAMES} frames"
    echo "G+B 2208000" >> ./output/performance-benchmarks/${G}-power.txt
    adb shell "echo 2208000 > /sys/devices/system/cpu/cpufreq/policy2/scaling_max_freq"
    MWP=${MIDWAY_POINTS[$c]}
    adb shell "${PREFIX}./data/local/Working_dir/graph_${G}_all_pipe_sync --threads=4 --threads2=2 --n=${FRAMES} --total_cores=6 --partition_point=${MWP} --partition_point2=${PP} --order=G-B-L" | grep "Frame\|Running Inference" | tee -a ./output/performance-benchmarks/${G}-power.txt

    FRAMES=240
    echo "Running GPU + Big CPU at 1704000 kHz for ${FRAMES} frames"
    echo "G+B 1704000" >> ./output/performance-benchmarks/${G}-power.txt
    adb shell "echo 1704000 > /sys/devices/system/cpu/cpufreq/policy2/scaling_max_freq"
    MWP=${MIDWAY_POINTS_2[$c]}
    adb shell "${PREFIX}./data/local/Working_dir/graph_${G}_all_pipe_sync --threads=4 --threads2=2 --n=${FRAMES} --total_cores=6 --partition_point=${MWP} --partition_point2=${PP} --order=G-B-L" | grep "Frame\|Running Inference" | tee -a ./output/performance-benchmarks/${G}-power.txt
    echo "Done with ${G}"
done
