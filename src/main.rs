// get our modules
mod laz_to_gpq;
use laz_to_gpq::read_laz_to_gpq;

use clap::{Parser, Subcommand};

// Define args for CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct PointQuaffer {
    /// figure out the process to run
    #[command(subcommand)]
    process: ProcessType,
}

#[derive(Subcommand)]
enum ProcessType {
    // Importing .laz files to the geoparquet schema/file
    LazImport {
        // path to .laz file
        input: String,
    },
    // Visualizing an imported set of data
    Viz {
        // path to geoparquet file (created by laz-import)
        input: String,
    },
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let cli = PointQuaffer::parse();

    match &cli.process {
        ProcessType::LazImport { input } => {
            let outfile_path = format!("{}.parquet", input.trim_end_matches(".laz"));
            read_laz_to_gpq(input.to_string(), outfile_path)
        }
        ProcessType::Viz { input } => {
            println!("hey boss can we visualize {} yet?", input);
            println!("he said no");
            Ok(())
        }
    }
}
