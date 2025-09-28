// for las import, point cloud processing (and viz?)
use threecrate::{Error, Point3f, PointCloud};
use threecrate_io::read_point_cloud;

// for arg handling
use std::env;
// for local file reads
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("point quaffer v0.1.0");
        println!("  ->  provide a filename/path to a .laz to continue");
        return;
    }

    let filename = &args[1];

    println!("Opening point cloud .laz file at {filename}");

    let laz_open_result = process_laz(filename);
    if laz_open_result.is_err() {
        let err = laz_open_result.unwrap_err();
        println!("Error opening file! wtf???");
        dbg!(err);
    } else {
        println!("Successfully read file!");
        let point_cloud = laz_open_result.unwrap();
        let point_count = point_cloud.points.len();
        println!("yo got a point count {point_count}");
    }

    println!("all done! bye! :)");
}

fn process_laz(filename: &String) -> Result<PointCloud<Point3f>, Error> {
    let full_path_str = &("./".to_string() + filename);
    let path = Path::new(full_path_str);

    read_point_cloud(path)    
}
