use std::{
    path::Path, fs, io::prelude::*
};



static CURRENT_DIR : &str = "." ;
static INPUT_DIR   : &str = "input" ;



fn main() {
    let inputDir = Path::new( CURRENT_DIR ).join( INPUT_DIR ) ;
    let paths = fs::read_dir( inputDir ).unwrap() ;
    for path in paths {
    println!("Hello, {}!", path.unwrap().path().display() );
    }
}
