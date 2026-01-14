# batteryctl

A CLI tool written in Rust for retrieving detailed battery information and metrics.

## Description

`batteryctl` allows users to quickly inspect the status of their laptop battery via the terminal. Whether you need to check your battery’s State of Health (SOH), current voltage, or estimate how much time you have left before the battery runs out, `batteryctl` provides a clean interface for accessing this data.

## Features

- **Comprehensive Stats**: Displays Vendor, Model, Technology, Serial number, and specific energy statistics.
- **Health Monitoring**: Check your battery’s State of Health (SOH) and cycle count.
- **Output Formats**: Supports \`default\` (human-readable), \`bare\` (minimal), and \`json\` outputs for integration with other scripts or status bars (e.g., Polybar, i3status).
- **Unit Support**: Flexible temperature units (Celsius/Fahrenheit).

## Installation

Ensure you have Rust and Cargo installed.

```bash
cargo install --path .
```

## Usage

The primary command is \`info\`, which retrieves the current battery status.

### Standard Output

```bash
batteryctl info
```

### JSON Output

Useful for programmatically parsing battery data.

```bash
batteryctl info json
```

### Bare Output

Minimal output format.

```bash
batteryctl info bare
```

### Options

- \`-f, --fahrenheit\`: Display temperature in Fahrenheit.
- \`-d, --debug\`: Enable debug output.

```bash
# Example: JSON output with Fahrenheit temperatures
batteryctl info --display json --fahrenheit
```

## License

[MIT OR Apache-2.0](LICENSE)
