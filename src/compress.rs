

extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::write::DeflateEncoder;
use std::io::prelude::*;


pub fn gzip(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {

    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    if let Err(error) = e.write_all(&data) {
        return Err(error);
    }
    e.finish()

}


pub fn deflate(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {

    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());

    if let Err(error) = e.write_all(&data) {
        return Err(error);
    }
    e.finish()

}


