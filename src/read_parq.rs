use std::fs::File;

use arrow_array::Float64Array;
use arrow_array::RecordBatch;
use arrow_array::RecordBatchReader;
use arrow_array::StringArray;
use arrow_array::StructArray;
use arrow_schema::ArrowError;
use geoparquet::reader::{GeoParquetReaderBuilder, GeoParquetRecordBatchReader};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

const BATCH_ROW_SIZE: usize = 65536; // this was the val in the example
const DEBUG_PRINT_FREQ: usize = 10000; // print debug record info every <this num> of rows

pub fn read(filepath: &String) {
    println!("Opening parquet file at {}...", filepath);
    let file = File::open(filepath).unwrap();
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    println!("Setting up geoparquet reader...");
    let geoparquet_metadata = builder.geoparquet_metadata().unwrap().unwrap();
    let geoarrow_schema = builder
        .geoarrow_schema(&geoparquet_metadata, true, Default::default())
        .unwrap();
    println!("Start reader...");
    let parquet_reader = builder.with_batch_size(BATCH_ROW_SIZE).build().unwrap();
    let geoparquet_reader =
        GeoParquetRecordBatchReader::try_new(parquet_reader, geoarrow_schema).unwrap();
    let schema = geoparquet_reader.schema();
    println!("Schema info:\n{}", schema);
    println!("Starting batch parsing...");
    let batches = geoparquet_reader
        .collect::<Result<Vec<RecordBatch>, ArrowError>>()
        .unwrap();
    let batch_count = batches.len();
    for (batch_i, batch) in batches.iter().enumerate() {
        // Get the xy column as a StructArray
        let xy_col_arr_ref = batch.column_by_name("xy").unwrap();
        let xy_struct = xy_col_arr_ref
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();

        // Get x and y arrays from the struct
        let x_array = xy_struct
            .column_by_name("x")
            .unwrap()
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();
        let y_array = xy_struct
            .column_by_name("y")
            .unwrap()
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();
        // Grab z column directly
        let z_array = batch
            .column_by_name("z")
            .unwrap()
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();
        // Grab point classifaction too
        let class_array = batch
            .column_by_name("classification")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        let row_count = if batch_i < batch_count - 1 {
            BATCH_ROW_SIZE
        } else {
            batch.num_rows() // last batch may not be full
        };

        for i in 0..row_count {
            // Print some sample rows
            if i % DEBUG_PRINT_FREQ == 0 {
                let x = x_array.value(i);
                let y = y_array.value(i);
                let z = z_array.value(i);
                let class = class_array.value(i);
                println!("Row {}: x={}, y={}, z={} - {}", i, x, y, z, class);
            }
        }
    }
    println!("Done!");
}
