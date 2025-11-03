# geolocate-rs

A simple, fast command-line tool for geolocating IP addresses and domain names written in Rust.

## Features

- 🚀 **Fast** - Built with Rust for optimal performance
- 🌍 **IP Geolocation** - Query geolocation data for any IP address
- 🔍 **Domain Resolution** - Automatically resolves domains to IPs and geolocates them
- 🗺️ **Google Maps Integration** - Provides direct links to Google Maps for coordinates
- 💻 **CLI Tool** - Simple command-line interface
- 📦 **Single Binary** - No runtime dependencies, just download and run

## Installation

### Option 1: Install from AUR (Recommended)

Install using your favorite AUR helper:

```bash
# Using yay
yay -S geolocate-rs

# Using paru
paru -S geolocate-rs

# Or manually
git clone https://aur.archlinux.org/geolocate-rs.git
cd geolocate-rs
makepkg -si
```

### Option 2: Build from Source

#### Prerequisites

- Rust and Cargo (install from https://rustup.rs/ or `sudo pacman -S rust`)

#### Build Instructions

```bash
# Clone the repository
git clone https://github.com/r3dg0d/geolocate.git
cd geolocate

# Build in release mode
cargo build --release

# The binary will be at: target/release/geolocate
```

To install system-wide:
```bash
sudo cp target/release/geolocate /usr/local/bin/
```

### Option 3: Download Pre-built Binary

Check the [Releases](https://github.com/r3dg0d/geolocate/releases) page for pre-built binaries.

## Usage

### Basic Usage

```bash
# Geolocate an IP address
geolocate-rs 8.8.8.8

# Geolocate a domain name
geolocate-rs github.com

# Geolocate a full URL
geolocate-rs https://www.google.com
```

### Examples

#### Geolocate Google's DNS
```bash
$ geolocate-rs 8.8.8.8
Target: 8.8.8.8
IP: 8.8.8.8
Location: Ashburn, Virginia, United States
Coordinates: 39.03, -77.5
Google Maps URL: https://maps.google.com/maps?q=39.03,-77.5
```

#### Geolocate a Domain
```bash
$ geolocate-rs github.com
Target: github.com
IP: 140.82.116.4
Location: San Francisco, California, United States
Coordinates: 37.7823, -122.391
Google Maps URL: https://maps.google.com/maps?q=37.7823,-122.391
```

#### Geolocate a URL
```bash
$ geolocate-rs https://www.reddit.com
Target: https://www.reddit.com
IP: 151.101.1.140
Location: San Francisco, California, United States
Coordinates: 37.7823, -122.391
Google Maps URL: https://maps.google.com/maps?q=37.7823,-122.391
```

## API

This tool uses the free [ip-api.com](http://ip-api.com) service for geolocation data.

**Rate Limits:**
- Free tier: 45 requests/minute from the same IP
- For higher limits, see [ip-api.com/pricing](http://ip-api.com/pricing)

## Output Format

The tool outputs:
1. **Target**: The original input (IP, domain, or URL)
2. **IP**: The resolved IP address (same as target if IP was provided)
3. **Location**: City, Region, Country
4. **Coordinates**: Latitude and Longitude
5. **Google Maps URL**: Direct link to view the location on Google Maps

## Supported Input Formats

- **IP Address**: `8.8.8.8`, `2001:4860:4860::8888` (IPv4 and IPv6)
- **Domain Name**: `github.com`, `www.google.com` (automatically prepends `https://`)
- **Full URL**: `https://example.com`, `http://example.com`

## Error Handling

The tool handles various error cases:
- Invalid IP addresses
- Invalid or unresolvable domain names
- Network errors
- API errors

## Development

### Prerequisites

- Rust 1.70+ and Cargo
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/r3dg0d/geolocate.git
cd geolocate

# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run with a target
cargo run -- 8.8.8.8
```

## Dependencies

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [dns-lookup](https://crates.io/crates/dns-lookup) - DNS resolution
- [url](https://github.com/servo/rust-url) - URL parsing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details

## Author

[r3dg0d](https://github.com/r3dg0d)

## Acknowledgments

- [ip-api.com](http://ip-api.com) for providing the free geolocation API

