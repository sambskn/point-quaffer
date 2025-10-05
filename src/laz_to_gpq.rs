use geoarrow::error::{GeoArrowError, GeoArrowResult};
use geoarrow_schema::crs::CrsTransform;
// point cloud readin' son
use las::point::Classification;
use las::{LazParallelism, Reader, ReaderOptions};
use serde_json::Value;
use std::collections::HashMap;
// opening/closing files
use std::fs::File;
use std::io::BufReader;
// geoarrow!
use arrow_array::{ArrayRef, Float64Array, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use geoarrow::array::PointBuilder;
use geoarrow::datatypes::{Crs, Dimension, PointType};
use geoarrow_array::GeoArrowArray;
use std::sync::Arc;
// geoparquet writer
use geoparquet::writer::{GeoParquetRecordBatchEncoder, GeoParquetWriterOptionsBuilder};
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use gdal::spatial_ref::SpatialRef;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct WKTStringTransform {
    wkt_str: String,
    proj_str: String,
}

impl WKTStringTransform {
    pub fn new(wkt_str: String) -> Self {
        let cleaned_wkt = wkt_str.trim_ascii().trim_end_matches('\0').to_string();
        let spatial_ref = SpatialRef::from_wkt(&cleaned_wkt).expect("ruh ruh gdal broke");
        let proj_str = spatial_ref
            .to_projjson()
            .expect("oops couldn't make a projjson str");
        WKTStringTransform { wkt_str, proj_str }
    }
}
impl CrsTransform for WKTStringTransform {
    fn _convert_to_projjson(
        &self,
        _crs: &Crs,
    ) -> std::result::Result<Option<serde_json::Value>, GeoArrowError> {
        let json_value: Value =
            serde_json::from_str(&self.proj_str).expect("couldn't parse json str for proj");
        Ok(Some(json_value))
    }

    fn _convert_to_wkt(&self, _crs: &Crs) -> GeoArrowResult<Option<String>> {
        Ok(Some(self.wkt_str.clone()))
    }

    fn extract_projjson(&self, _crs: &Crs) -> geoarrow::error::GeoArrowResult<Option<Value>> {
        let json_value: Value =
            serde_json::from_str(&self.proj_str).expect("couldn't parse json str for proj");
        Ok(Some(json_value))
    }

    fn extract_wkt(&self, _crs: &Crs) -> geoarrow::error::GeoArrowResult<Option<String>> {
        Ok(Some(self.wkt_str.clone()))
    }
}

// open up a point cloud .laz file
// (testing with USGS data)
// and dump it into a geoparquet file
pub fn read_laz_to_gpq(
    filename: String,
    filter_to_ground: bool,
    max_points: Option<i64>,
    outfile_path: String,
) -> Result<()> {
    println!("Opening point cloud .laz file at {filename}");
    let file = File::open(filename).unwrap();
    let options = ReaderOptions::default();
    options.with_laz_parallelism(LazParallelism::Yes);
    let mut reader = Reader::with_options(BufReader::new(file), options)?;
    let mut wkt_transform = None;
    let header = reader.header();
    // try to grab the wkt string verison of the CRS
    // (according to LAZ spec CRS can either be WKT or 'GeoTIFF' based -
    // so far all the USGS data sampled has has WKT)
    if header.has_wkt_crs() {
        let header_vlrs = header.vlrs();
        for vlr in header_vlrs {
            if vlr.description.contains("WKT") {
                println!("found a WKT header!");
                let data = vlr.data.clone();
                let parsed_wkt_string = String::from_utf8(data).unwrap();
                wkt_transform = Some(WKTStringTransform::new(parsed_wkt_string));
            }
        }
    }
    // Vectors to accumulate point data
    let mut x_coords: Vec<f64> = Vec::new();
    let mut y_coords: Vec<f64> = Vec::new();
    let mut z_coords: Vec<f64> = Vec::new();
    let mut fids: Vec<i64> = Vec::new();
    let mut intensities: Vec<i64> = Vec::new();
    let mut return_numbers: Vec<i64> = Vec::new();
    let mut number_of_returns: Vec<i64> = Vec::new();
    let mut scan_directions: Vec<String> = Vec::new();
    let mut classifications: Vec<String> = Vec::new();
    let mut scan_angles: Vec<f64> = Vec::new();
    let mut point_source_ids: Vec<i64> = Vec::new();
    let mut gps_times: Vec<Option<f64>> = Vec::new();

    let has_max = max_points.is_some();
    let mut i: i64 = 0;
    for point in reader.points() {
        if has_max && i > max_points.unwrap() {
            println!("Hit limit for max number of points, continuing...");
            break;
        }
        let pnt = point?;
        // if filter flag was provided, and point isn't ground, dip early
        if filter_to_ground && pnt.classification != Classification::Ground {
            continue;
        }
        x_coords.push(pnt.x);
        y_coords.push(pnt.y);
        z_coords.push(pnt.z);
        fids.push(i);
        intensities.push(pnt.intensity as i64);
        return_numbers.push(pnt.return_number as i64);
        number_of_returns.push(pnt.number_of_returns as i64);
        scan_directions.push(match pnt.scan_direction {
            las::point::ScanDirection::LeftToRight => "LeftToRight".to_string(),
            las::point::ScanDirection::RightToLeft => "RightToLeft".to_string(),
        });
        classifications.push(format!("{:?}", pnt.classification));
        scan_angles.push(pnt.scan_angle as f64);
        point_source_ids.push(pnt.point_source_id as i64);
        gps_times.push(pnt.gps_time);

        i += 1;
    }

    println!("total count {i}");
    println!("Building GeoArrow arrays...");

    // Build the PointArray
    let point_type = PointType::new(Dimension::XY, Default::default());
    let data_type_point = point_type.clone().data_type();
    let mut point_builder = PointBuilder::new(point_type);
    point_builder.reserve(i as usize);

    for idx in 0..i as usize {
        point_builder.push_point(Some(&geo::Point::new(x_coords[idx], y_coords[idx])));
    }

    let point_array = point_builder.finish();
    // Create metadata arrays
    let fid_array = Int64Array::from(fids);
    let z_array = Float64Array::from(z_coords);
    let intensity_array = Int64Array::from(intensities);
    let return_number_array = Int64Array::from(return_numbers);
    let number_of_returns_array = Int64Array::from(number_of_returns);
    let scan_direction_array = StringArray::from(scan_directions);
    let classification_array = StringArray::from(classifications);
    let scan_angle_array = Float64Array::from(scan_angles);
    let point_source_id_array = Int64Array::from(point_source_ids);
    let gps_time_array = Float64Array::from(gps_times);

    let mut metadata = HashMap::new();
    metadata.insert(
        "ARROW:extension:name".to_string(),
        "geoarrow.point".to_string(),
    );
    metadata.insert("ARROW:extension:metadata".to_string(), "{}".to_string());
    let geometry_field = Field::new("xy", data_type_point, false).with_metadata(metadata);

    let schema = Schema::new(vec![
        geometry_field,
        Field::new("fid", DataType::Int64, false),
        Field::new("z", DataType::Float64, false),
        Field::new("intensity", DataType::Int64, false),
        Field::new("return_number", DataType::Int64, false),
        Field::new("number_of_returns", DataType::Int64, false),
        Field::new("scan_direction", DataType::Utf8, false),
        Field::new("classification", DataType::Utf8, false),
        Field::new("scan_angle", DataType::Float64, false),
        Field::new("point_source_id", DataType::Int64, false),
        Field::new("gps_time", DataType::Float64, true),
    ]);

    let points_arr_ref: ArrayRef = point_array.into_array_ref();
    // Create RecordBatch
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            points_arr_ref,
            Arc::new(fid_array),
            Arc::new(z_array),
            Arc::new(intensity_array),
            Arc::new(return_number_array),
            Arc::new(number_of_returns_array),
            Arc::new(scan_direction_array),
            Arc::new(classification_array),
            Arc::new(scan_angle_array),
            Arc::new(point_source_id_array),
            Arc::new(gps_time_array),
        ],
    )?;

    println!("Writing GeoParquet to {outfile_path}...");

    let options = if wkt_transform.is_some() {
        // build with CRS info
        GeoParquetWriterOptionsBuilder::default()
            .set_primary_column("xy".to_string())
            .set_crs_transform(Box::new(wkt_transform.unwrap()))
            .build()
    } else {
        // build w/o crs info (assumes WGS84)
        GeoParquetWriterOptionsBuilder::default()
            .set_primary_column("xy".to_string())
            .build()
    };

    let mut gpq_encoder = GeoParquetRecordBatchEncoder::try_new(&schema, &options)?;

    // Create Parquet writer with the target schema from the encoder
    let file = File::create(&outfile_path)?;
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let mut parquet_writer = ArrowWriter::try_new(file, gpq_encoder.target_schema(), Some(props))?;

    // Encode and write the batch
    let encoded_batch = gpq_encoder.encode_record_batch(&batch)?;
    parquet_writer.write(&encoded_batch)?;

    // Add GeoParquet metadata and finish
    let kv_metadata = gpq_encoder.into_keyvalue()?;
    parquet_writer.append_key_value_metadata(kv_metadata);
    parquet_writer.close()?;

    println!("Done! Wrote {i} points to {outfile_path}");

    Ok(())
}
