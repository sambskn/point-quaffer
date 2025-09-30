// point cloud readin' son
use las::Reader;

// for arg handling
use std::env;
use std::fs::File;

// fgb!
use flatgeobuf::*;
use geozero::geojson::GeoJson;
use geozero::{ColumnValue, PropertyProcessor};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("point quaffer v0.1.0");
        println!("  ->  provide a filename/path to a .laz to continue");
        return Ok(());
    }

    let filename = &args[1];
    let outfile_path = format!("{}.fgb", filename.trim_end_matches(".laz"));

    let mut fgb = FgbWriter::create(outfile_path.as_str(), GeometryType::Point)?;
    fgb.add_column("fid", ColumnType::Long, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("intensity", ColumnType::Long, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("return_number", ColumnType::Long, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("number_of_returns", ColumnType::Long, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("scan_direction", ColumnType::String, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("classification", ColumnType::String, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("scan_angle", ColumnType::Double, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("point_source_id", ColumnType::Long, |_fbb, col| {
        col.nullable = false;
    });
    fgb.add_column("gps_time", ColumnType::Double, |_fbb, col| {
        col.nullable = false;
    });

    println!("Opening point cloud .laz file at {filename}");
    let mut reader = Reader::from_path(filename).expect("Couldn't open a reader");
    let mut i = 0;
    for point in reader.points() {
        let pnt = point.unwrap();
        let geom_str = format!(
            r#"{{"type": "Point", "coordinates": [{}, {}, {}]}}"#,
            pnt.x, pnt.y, pnt.z
        );
        let geom = GeoJson(&geom_str);
        fgb.add_feature_geom(geom, |feat| {
            feat.property(0, "fid", &ColumnValue::Long(i)).unwrap();
            feat.property(1, "intensity", &ColumnValue::Long(pnt.intensity as i64))
                .unwrap();
            feat.property(
                2,
                "return_number",
                &ColumnValue::Long(pnt.return_number as i64),
            )
            .unwrap();
            feat.property(
                3,
                "number_of_returns",
                &ColumnValue::Long(pnt.number_of_returns as i64),
            )
            .unwrap();
            feat.property(
                4,
                "scan_direction",
                &ColumnValue::String(match pnt.scan_direction {
                    las::point::ScanDirection::LeftToRight => "LeftToRight",
                    las::point::ScanDirection::RightToLeft => "RightToLeft",
                }),
            )
            .unwrap();
            feat.property(
                5,
                "classification",
                &ColumnValue::String(format!("{:?}", pnt.classification).as_str()),
            )
            .unwrap();
            feat.property(6, "scan_angle", &ColumnValue::Double(pnt.scan_angle as f64))
                .unwrap();
            feat.property(
                7,
                "point_source_id",
                &ColumnValue::Long(pnt.point_source_id as i64),
            )
            .unwrap();
            if let Some(gps) = pnt.gps_time {
                feat.property(8, "gps_time", &ColumnValue::Double(gps))
                    .unwrap();
            }
        })
        .ok();
        i += 1;
    }
    println!("total count {i}");
    let outfile = File::create(outfile_path)?;
    fgb.write(outfile)?;
    Ok(())
}
