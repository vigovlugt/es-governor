set -e
# Requires adb connection and adb root
cargo build --target armv7-linux-androideabi --release
adb push target/armv7-linux-androideabi/release/governor /data/local/Working_dir
adb shell mv /data/local/Working_dir/governor /data/local/Working_dir/Governor
