// get our modules
#[cfg(feature = "laz_import")]
mod laz_to_gpq;

#[cfg(feature = "laz_import")]
use laz_to_gpq::read_laz_to_gpq;

#[cfg(feature = "parquet")]
mod read_parq;

#[cfg(feature = "parquet")]
use read_parq::read;
#[cfg(feature = "wasm_viz")]
mod bevy_viz;
#[cfg(feature = "wasm_viz")]
use bevy_viz::start_bevy;
#[cfg(feature = "wasm_viz")]
mod bevy_blob_loader_lib;
#[cfg(feature = "wasm_viz")]
mod bevy_blob_loader_path;
#[cfg(feature = "wasm_viz")]
mod bevy_blob_loader_source;
#[cfg(feature = "wasm_viz")]
mod bevy_web_file_drop;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};

// Define args for CLI
#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct PointQuaffer {
    /// figure out the process to run
    #[command(subcommand)]
    process: ProcessType,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum ProcessType {
    // Importing .laz files to the geoparquet schema/file
    #[cfg(feature = "laz_import")]
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
    // Reading an imported set of data
    #[cfg(feature = "parquet")]
    Read {
        // path to geoparquet file (created by laz-import)
        input: String,
    },
    Hello,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    #[cfg(feature = "wasm_viz")]
    {
        start_bevy();
        return Ok(());
    }
    #[cfg(feature = "cli")]
    {
        let cli = PointQuaffer::parse();

        match &cli.process {
            #[cfg(feature = "laz_import")]
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

            #[cfg(feature = "parquet")]
            ProcessType::Read { input } => {
                read(input);
                Ok(())
            }
            ProcessType::Hello => {
                println!("yo it's the point quaffer");
                Ok(())
            }
        }
    }
}
