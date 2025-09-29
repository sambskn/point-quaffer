// point cloud readin' son
use las::Reader;

// for arg handling
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("point quaffer v0.1.0");
        println!("  ->  provide a filename/path to a .laz to continue");
        return;
    }

    let filename = &args[1];

    println!("Opening point cloud .laz file at {filename}");
    let mut reader = Reader::from_path(filename).expect("Couldn't open a reader");
    let mut i = 0;
    for point in reader.points() {
        if i % 10000 == 0 {
            let pnt = point.unwrap();
            dbg!(pnt);
        } 
        i += 1;
    }
    println!("total count {i}");
}

