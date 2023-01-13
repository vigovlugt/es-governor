set -e
./scripts/build.sh
adb shell "cd /data/local/Working_dir/ && RUST_LOG=debug ./Governor $@"