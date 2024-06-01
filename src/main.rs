use bytes::Buf;
use galileo_mvt::vector_tile::Tile as GalileoTile;
use geozero::mvt::Tile as GeoZeroTile;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use prost::Message;
use geozero::mvt::Message as GeoMessage;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Hello, world!");

    //read bus_tile.mvt file into bytes
    let mut f = File::open("bus_tile.mvt")?;
    let mut buffer: Vec<u8> = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let buffer = buffer;

    let start_galileo = Instant::now();
    for _ in 0..1000 {
        //println!("Decoding galileo tile");
        let tile = GalileoTile::decode(buffer.as_slice())?;
    }
    let duration_galileo = start_galileo.elapsed();
    println!("Time elapsed in galileo is: {:?}", duration_galileo);
    println!("Time per tile: {:?}", duration_galileo / 1000);

    let start_geozero = Instant::now();
    for _ in 0..1000 {
        //println!("Decoding geozero tile");
        let tile = GeoZeroTile::decode(buffer.as_slice())?;
    }
    let duration_geozero = start_geozero.elapsed();
    println!("Time elapsed in geozero is: {:?}", duration_geozero);
    println!("Time per tile: {:?}", duration_geozero / 1000);

    Ok(())
}
