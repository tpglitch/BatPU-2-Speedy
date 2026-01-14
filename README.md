# BatPU-2-Speedy

A high-performance Rust-based assembler for the [BatPU-2 redstone computer](https://www.youtube.com/watch?v=3gBZHXqnleU).

## Overview

BatPU-2-Speedy streamlines the development workflow for BatPU-2 programs by providing fast assembly, machine code generation, and Minecraft schematic creation. Built in Rust for maximum performance and reliability.

## Features

- **Assembly**: Compile `.s` assembly files to `.mc` machine code files
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
batpu2-speedy assemble -i input.s -o output.mc

# Generate a schematic from machine code
batpu2-speedy schematic -i input.mc -o output.schem

# Build everything at once
batpu2-speedy build -i input.s -o program.schem
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

- **`.s`** - Assembly source files containing BatPU-2 assembly code (or `.S`, `.as`, `.asm`, etc.)
- **`.mc`** - Machine code files (intermediate binary format)
- **`.schem`** - Sponge Schematic files for Minecraft WorldEdit/Litematica

## Assembly Language Syntax

BatPU-2-Speedy supports both legacy and modern assembly syntax:

### Modern Syntax Features

- **Labels with colons**: `main:` or `loop:` (instead of `.main` or `.loop`)
- **Semicolon comments**: `;` in addition to `//` and `#`
- **Comma separators**: `add r1, r2, r3` (optional, for readability)
- **Data directives**:
  - `.db` / `.byte` - Define bytes
  - `.dw` / `.word` - Define 16-bit words
  - `.ascii` - ASCII string (no null terminator)
  - `.asciz` / `.string` - ASCII string with null terminator
- **String literals in data**: `.db "Hello"` expands to individual bytes
- **Multiple define styles**: `define`, `.equ`, or `.define`
- **Hex and binary numbers**: `0xFF`, `0b11111111`

### Example: Modern Syntax

```assembly
; Hello World using modern syntax
main:
    ldi r15, clear_chars_buffer
    str r15, r0

    ldi r1, message         ; Pointer to message
    ldi r2, 10              ; Message length
    ldi r15, write_char

print_loop:
    lod r1, r14, 0
    str r15, r14
    inc r1
    dec r2
    brh nz, print_loop

    ldi r15, buffer_chars
    str r15, r0
    hlt

message:
    .db 7, 4, 11, 11, 14    ; "HELLO"
```

### Legacy Syntax

The assembler remains fully backward compatible with the original syntax:

```assembly
// Hello World using legacy syntax
.main
    LDI r15 write_char
    LDI r14 "H"
    STR r15 r14
    LDI r14 "E"
    STR r15 r14
    // ... etc
    HLT
```

### Supported Instructions

All BatPU-2 instructions are supported:
- Arithmetic: `ADD`, `SUB`, `ADI`
- Logic: `NOR`, `AND`, `XOR`, `RSH`
- Memory: `LDI`, `LOD`, `STR`
- Control: `JMP`, `BRH`, `CAL`, `RET`
- Special: `NOP`, `HLT`

Pseudo-instructions:
- `CMP` - Compare (SUB with r0 destination)
- `MOV` - Move (ADD with r0)
- `LSH` - Left shift
- `INC` / `DEC` - Increment/decrement
- `NOT` - Bitwise NOT
- `NEG` - Negate

## BatPU-2 Architecture

The BatPU-2 is a redstone computer architecture created for Minecraft. This assembler supports the full BatPU-2 instruction set and generates compatible machine code.

For more information about the BatPU-2 architecture, see the [original video](https://www.youtube.com/watch?v=3gBZHXqnleU).

## Contributing

Please feel free to submit issues, feature requests, and pull requests.

## License
This project is licensed under the MIT license. Read the [LICENSE](./LICENSE) file for more information.

## Links

- [BatPU-2 Video](https://www.youtube.com/watch?v=3gBZHXqnleU)
