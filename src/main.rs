use clap::{Parser, Subcommand};
use std::path::Path;
use std::process;

mod assembler;
mod schematic;
mod symbols;

use assembler::assemble;
use schematic::make_schematic;

#[derive(Parser)]
#[command(name = "batpu2-speedy")]
#[command(about = "A BatPU-2 assembler that can generate machine code and Minecraft schematics")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Assemble assembly file to machine code")]
    Assemble {
        // Input assembly file
        #[arg(short, long)]
        input: String,
        // Output machine code file
        #[arg(short, long)]
        output: Option<String>,
    },
    #[command(about = "Converts machine code to Minecraft schematic")]
    Schematic {
        // Input machine code file
        #[arg(short, long)]
        input: String,
        // Output schematic file
        #[arg(short, long)]
        output: Option<String>,
    },
    #[command(about = "Assemble and generate schematic in one step")]
    Build {
        // Input assembly file
        #[arg(short, long)]
        input: String,
        // Output machine code file (optional)
        #[arg(short, long)]
        machine_code: Option<String>,
        // Output schematic file (optional)
        #[arg(short, long)]
        schematic: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble { input, output } => {
            let output = output.unwrap_or_else(|| {
                let base = Path::new(&input).file_stem().unwrap().to_str().unwrap();
                format!("{}.mc", base)
            });
            if let Err(e) = assemble(&input, &output) {
                eprintln!("Assembly failed: {}", e);
                process::exit(1);
            }
            println!("Successfully assembled {} to {}", input, output);
        }
        Commands::Schematic { input, output } => {
            let output = output.unwrap_or_else(|| {
                let base = Path::new(&input).file_stem().unwrap().to_str().unwrap();
                format!("{}.schem", base)
            });
            if let Err(e) = make_schematic(&input, &output) {
                eprintln!("Schematic generation failed: {}", e);
                process::exit(1);
            }
            println!("Successfully generated schematic {} from {}", output, input);
        }
        Commands::Build {
            input,
            machine_code,
            schematic,
        } => {
            let base = Path::new(&input).file_stem().unwrap().to_str().unwrap();
            let mc_file = machine_code.unwrap_or_else(|| format!("{}.mc", base));
            let schem_file = schematic.unwrap_or_else(|| format!("{}.schem", base));

            // Assemble into .mc
            if let Err(e) = assemble(&input, &mc_file) {
                eprintln!("Assembly failed: {}", e);
                process::exit(1);
            }
            println!("Successfully assembled {} to {}", input, mc_file);

            // Generate schematic from .mc
            if let Err(e) = make_schematic(&mc_file, &schem_file) {
                eprintln!("Schematic generation failed: {}", e);
                process::exit(1);
            }
            println!("Successfully generated schematic {}", schem_file);

            // Remove the intermediate .mc file
            if let Err(e) = std::fs::remove_file(&mc_file) {
                eprintln!(
                    "Warning: failed to remove intermediate file {}: {}",
                    mc_file, e
                );
            } else {
                println!("Cleaned up intermediate file {}", mc_file);
            }
        }
    }
}
