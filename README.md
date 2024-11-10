# Bevy Shop Menu

A simple shop menu implementation using the Bevy game engine. This project demonstrates how to create an interactive UI with buttons, state management, and resource handling in Bevy.

## Features

- Interactive shop menu with hover and click states
- Resource management (coins)
- Upgradeable player stats (damage and speed)
- Real-time UI updates
- Debug logging system

## Prerequisites

- Rust (latest stable version)
- A C++ linker (clang, lld, or Microsoft Visual C++)

## Dependencies

- Bevy 0.14.2
- env_logger
- log

## Building and Running

Clone the repository:

```bash
git clone https://github.com/gh057-ai/bevy-shop-menu.git
cd bevy-shop-menu
```

Run the project:

```bash
cargo run
```

For debug logging, run with:

```bash
RUST_LOG=debug cargo run
```

## Controls

- Mouse hover over buttons to see interaction states
- Left-click to purchase upgrades
- ESC to exit the application

## Project Structure

- `src/main.rs` - Application entry point and camera setup
- `src/shop.rs` - Shop implementation including UI, state management, and interactions
- `.cargo/config.toml` - Cargo configuration for different platforms
- `Cargo.toml` - Project dependencies and configuration

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
