set -e
adb connect 10.42.0.80
adb root # Takes a while
sleep 20
adb shell "echo 1 > /sys/class/fan/enable && echo 0 > /sys/class/fan/mode && echo 4 > /sys/class/fan/level"