use clap::{Parser, Subcommand};

mod compute;

#[derive(Parser)]
#[command(name = "WGPU")]
#[command(version = "0.1.0")]
#[command(about = "Learn WGPU by examples with compute and graphics modules", long_about = None)]
struct Cli {
    #[command(subcommand)]
    module: Modules,
}

#[derive(Subcommand)]
enum Modules {
    /// WGPU compute examples
    Compute {
        #[command(subcommand)]
        command: ComputeCommands,
    },

    /// WGPU graphics examples
    Graphics {
        #[command(subcommand)]
        command: GraphicsCommands,
    }
}

#[derive(Subcommand)]
enum ComputeCommands {
    /// Collatz conjecture
    Collatz,
}


#[derive(Subcommand)]
enum GraphicsCommands {
    /// Run boid simulation
    Boid,
}

fn main() {
    let cli = Cli::parse();

    match cli.module {
        Modules::Compute { command } => match command {
            ComputeCommands::Collatz => compute::collatz::main(),
        },
        Modules::Graphics { command } => match command {
            _ => panic!("WGPU graphics examples coming soon!")
        }
    }
}