# point-quaffer
### "its the quaffiest!"

rust cli that is for playing around with point cloud data and geoarrow/geoparquet

im inherently paranoid of every processed raster GeoTIFF I get from a surveyor and want to better understand what goes into working with the point cloud data they actually collect

## goals and stuff I wanna try
- [x] Parse .LAZ point cloud data from a standardized source (USGS data)
  - [ ] Run some algos on the collected points to make a 'simplified' surface
    - [ ] Downsample points (somehow do it smartly?)
    - [ ] Mesh construction? (poisson?)
  - [x] Save parsed points as a GeoParquet file that formatted correctly
- [ ] Easily visualize point cloud data/mesh/some subset of this data
  - [x] Read points from created GeoParquet files
  - [ ] Visualize locally? Somehow? ([bevy_pointcloud](https://github.com/rlamarche/bevy_pointcloud) maybe?)
- [ ] Create proof of concept for streaming from GeoParquet to a client 
- [ ] Create release binary version of this crate? If actually helpful lol

## running/developing locally
1. Install [devenv](https://devenv.sh/getting-started/) if not already installed.
2. Run `devenv shell` to get a env with all the goodies ready to go (gdal stuff, ya boy's fav ide and command prompt, etc)
3. Test with `cargo r --`, ideally the CLI help commands and code itself can lead you from there, godspeed.

