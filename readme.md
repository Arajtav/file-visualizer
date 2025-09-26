# file-visualizer

[![License](https://img.shields.io/github/license/arajtav/file-visualizer)](https://raw.githubusercontent.com/Arajtav/file-visualizer/meow/LICENSE)

A program to visualize binary data.

## Installation

1. Clone the repository `git clone https://github.com/Arajtav/file-visualizer.git`.
2. Go into the cloned repository and build with `cargo build --release`.
3. The binary will be in `./target/release/file-visualizer` (with `.exe` on windows).

## Usage

`file-visualizer <MODE> <FILE> <OUT>`
where `FILE` is the input file and `OUT` is the output file (should end in `.png`).

### Modes

Currently the only mode is `adjacency`, it generates adjacency matrix of the input file.

### Options

`-i` - ignores the most frequent during normalization. Often needed because zero bytes are so prevalent no other data can be seen.

### Normalization

Normalization is controlled with `--normalization`, it has 2 modes,
- `max` - the default mode, divides everything by the highest value,
- `min-max` - subtracts the smallest value from everything before normalization, so that the contrast is higher.
