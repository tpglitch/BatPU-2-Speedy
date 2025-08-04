# BatPU-2-Speedy

A high-performance Rust-based assembler and toolchain for the [BatPU-2 redstone computer](https://www.youtube.com/watch?v=3gBZHXqnleU).

## Overview

BatPU-2-Speedy streamlines the development workflow for BatPU-2 programs by providing fast assembly, machine code generation, and Minecraft schematic creation. Built in Rust for maximum performance and reliability.

## Features

- **Assembly**: Compile `.S` assembly files to `.mc` machine code files
- **Schematic Generation**: Convert machine code into `.schem` (Sponge Schematic) files for direct Minecraft import
- **Unified Build Process**: Use the `build` command to assemble and generate schematics in one step
- **Fast Performance**: Rust implementation ensures quick compilation times
- **Error Handling**: Clear error messages and robust file processing

## Quick Start

### Installation from Binary

Downloads available at [the release page](https://github.com/tpglitch/BatPU-2-Speedy/releases)


### Installation from Source

```bash
# Clone the repository
git clone https://github.com/tpglitch/BatPU-2-Speedy.git
cd BatPU-2-Speedy

# Build the project
cargo build --release
```

### Basic Usage

```bash
# Assemble an assembly file
batpu2-speedy assemble -i input.S -o output.mc

# Generate a schematic from machine code
batpu2-speedy schematic -i input.mc -o output.schem

# Build everything at once
batpu2-speedy build -i input.S -o program.schem
```

## Commands

### `assemble`
Converts assembly files to machine code.
```bash
batpu2-speedy assemble -i <input.S> [options]
```

### `schematic`
Generates Minecraft schematics from machine code.
```bash
batpu2-speedy schematic -i <input.mc> [options]
```

### `build`
Runs assembly and schematic generation in sequence.
```bash
batpu2-speedy build -i <input.S> [options]
```

## File Formats

- **`.S`** - Assembly source files containing BatPU-2 assembly code
- **`.mc`** - Machine code files (intermediate binary format)
- **`.schem`** - Sponge Schematic files for Minecraft WorldEdit/Litematica

## BatPU-2 Architecture

The BatPU-2 is a redstone computer architecture created for Minecraft. This assembler supports the full BatPU-2 instruction set and generates compatible machine code.

For more information about the BatPU-2 architecture, see the [original video](https://www.youtube.com/watch?v=3gBZHXqnleU).

## Contributing

Please feel free to submit issues, feature requests, and pull requests.

## Links

- [BatPU-2 Video](https://www.youtube.com/watch?v=3gBZHXqnleU)
