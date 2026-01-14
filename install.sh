set -euo pipefail
cargo build --release
sudo cp target/release/batteryctl /bin/batteryctl
echo "'batteryctl' installed to /bin/batteryctl"