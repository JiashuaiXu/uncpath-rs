mod convert;
mod errors;
mod mapping;

use clap::Parser;
use errors::Result;
use mapping::MappingTable;
use std::path::PathBuf;

/// Convert UNC paths to POSIX paths based on mapping configuration
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// UNC path to convert (e.g., \\server\share\path, //server/share/path, smb://server/share/path)
    #[arg(value_name = "PATH")]
    path: String,

    /// Add custom mapping in format host:share:mount_point
    #[arg(short, long, value_name = "MAPPING")]
    mapping: Vec<String>,

    /// Load mappings from JSON file
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Skip default mappings
    #[arg(long)]
    no_defaults: bool,

    /// List all configured mappings
    #[arg(short, long)]
    list: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Build mapping table
    let mut table = if args.no_defaults {
        MappingTable::new()
    } else {
        MappingTable::with_defaults()
    };

    // Load from environment variable
    table.load_from_env()?;

    // Load from file if specified
    if let Some(file_path) = &args.file {
        table.load_from_file(file_path)?;
    }

    // Add CLI mappings
    for mapping_str in &args.mapping {
        table.add_from_cli(mapping_str)?;
    }

    // List mappings if requested
    if args.list {
        println!("Configured mappings:");
        for mapping in table.get_mappings() {
            println!(
                "  \\\\{}\\{} -> {}",
                mapping.host, mapping.share, mapping.mount_point
            );
        }
        return Ok(());
    }

    // Convert the path
    let posix_path = convert::convert_to_posix(&args.path, &table)?;
    println!("{}", posix_path);

    Ok(())
}
