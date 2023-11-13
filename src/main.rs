use std::{
    path::Path, fs, io::prelude::*
};



static CURRENT_DIR : &str = "." ;
static INPUT_DIR   : &str = "input" ;
static OUTPUT_DIR  : &str = "output" ;



fn main() {
    let inputDir = Path::new( CURRENT_DIR ).join( INPUT_DIR ) ;
    let outputDir = PrepareOutputDir() ;
    let paths = fs::read_dir( inputDir ).unwrap() ;
    for path in paths {
    println!("Hello, {}!", path.unwrap().path().display() );
    }
}



fn PrepareOutputDir(
)-> Box<Path>
{
    let outputDir = Path::new( CURRENT_DIR ).join( OUTPUT_DIR ) ;
    if outputDir.is_dir() {
        fs::remove_dir_all( &outputDir ) ;
    }
    fs::create_dir( &outputDir ) ;
    outputDir.into_boxed_path()
}
