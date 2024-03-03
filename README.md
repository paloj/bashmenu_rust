# BashMenu Rust Version

This project is a Rust-based rewrite of a Bash script that provides a simple terminal menu to execute commands stored in `.menuitem` files. It allows users to select a command to execute from a dynamically generated menu based on the `.menuitem` files present in the project's directory. Additionally, users can add new commands to the menu.

## Features

- Dynamically generates a menu from `.menuitem` files in the current directory.
- Executes commands selected by the user.
- Allows users to add new commands through a simple interface.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/bashmenu_rust.git
   ```
2. Navigate to the project directory:
   ```sh
   cd bashmenu_rust
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```
4. strip the binary (optional):
   ```sh
   strip target/release/bashmenu_rust
   ```

### Usage

To run BashMenu Rust, use the following command from the project directory:

```sh
cargo run
```

Follow the on-screen prompts to select a command to execute, add a new command, or quit the menu.

## Adding New Commands

To add a new command, select the "new" option from the menu, then enter the command and a nickname when prompted. The command will be saved as a `.menuitem` file in the current directory and will be available in the menu the next time BashMenu Rust is run.

## Create shortcut (optional)

Create a single letter alias to open the menu.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to suggest improvements or report bugs.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Original BashMenu script developers
- Rust community for providing an excellent programming language
