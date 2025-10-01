// get our modules
mod laz_to_gpq;
use laz_to_gpq::read_laz_to_gpq;
// for arg handling
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("point quaffer v0.2.0 (GeoArrow edition)");
        println!("  ->  provide a filename/path to a .laz to continue");
        return Ok(());
    }

    let filename = &args[1];
    let outfile_path = format!("{}.parquet", filename.trim_end_matches(".laz"));
    read_laz_to_gpq(filename.to_string(), outfile_path)
}
