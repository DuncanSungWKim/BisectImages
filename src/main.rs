use std::{
    path::Path, fs, io::prelude::*
};

use image::* ;
use rayon::prelude::* ;



const CURRENT_DIR : &str = "." ;
const INPUT_DIR   : &str = "input" ;
const OUTPUT_DIR  : &str = "output" ;



fn main() {
    println!( "Starting to bisect images ..." ) ;
    let inputDir = Path::new( CURRENT_DIR ).join( INPUT_DIR ) ;
    let outputDir = PrepareOutputDir() ;
    let paths = fs::read_dir( inputDir ).unwrap() ;
    paths.par_bridge().for_each( |path|{
        BisectImage( &path.unwrap().path(), &outputDir ) ;
    } );
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



fn BisectImage(
    imageFilePath : &Path
,   outputDir : &Path
)
{
    let mut img = open( imageFilePath ).unwrap() ;
    let width  = img.width() ;
    let height = img.height() ;
    let leftImage  = imageops::crop( &mut img,       0, 0, width/2, height ).to_image() ;
    let rightImage = imageops::crop( &mut img, width/2, 0, width  , height ).to_image() ;
    let fileNameStem = imageFilePath.file_stem().unwrap().to_str().unwrap() ;
    let leftFilePath  = outputDir.join( fileNameStem.to_owned() + "-L.png" ) ;
    let rightFilePath = outputDir.join( fileNameStem.to_owned() + "-R.png" ) ;
    leftImage.save_with_format(  &leftFilePath,  ImageFormat::Png ) ;
    rightImage.save_with_format( &rightFilePath, ImageFormat::Png ) ;
    println!( "bisected {}", imageFilePath.file_name().unwrap().to_str().unwrap() ) ;
}
