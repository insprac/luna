# Luna

Luna is a command-line tool uses gpt-4-turbo to interact with files within a local directory. It generates responses based on the contents and structure of the directory it operates in.

## Installation

To install Luna, you need Rust's package manager, Cargo. Clone the repository and build the project:

```bash
git clone https://github.com/insprac/luna.git
cd luna
cargo build --release
```

## Usage

To use the build you can find the binary in the `target/release` directory:

```bash
./target/release/luna --help
./target/release/luna ask "Write a better readme file for the project"
```

You can also add the binary to your path:

```bash
export PATH=$PATH:/path/to/luna/target/release
```

Then use `luna` anywhere:

```bash
luna --help
luna ask "Briefly describe the project"
```
