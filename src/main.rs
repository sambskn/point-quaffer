// get our modules
mod laz_to_gpq;
use laz_to_gpq::read_laz_to_gpq;
mod read_parq;
use read_parq::read;

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
        // Only get points classified as ground
        #[arg(short, long)]
        filter_to_ground: bool,
        // Stop collecting points at this number if provided
        #[arg(short, long, default_value=None)]
        max_point_count: Option<i64>,
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
        ProcessType::LazImport {
            input,
            filter_to_ground,
            max_point_count,
        } => {
            let mut outfile_path = input.trim_end_matches(".laz").to_string();
            if *filter_to_ground {
                outfile_path += "_filter";
            }
            outfile_path = format!("{}.parquet", outfile_path);
            read_laz_to_gpq(
                input.to_string(),
                *filter_to_ground,
                *max_point_count,
                outfile_path,
            )
        }
        ProcessType::Viz { input } => {
            read(input);
            Ok(())
        }
    }
}
