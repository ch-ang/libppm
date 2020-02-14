use std::fmt; // Import `fmt`

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Copy, Clone)]

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    
    // Default constructor
    pub fn empty_color() -> Pixel{
        Pixel{
        r : 0,
        g : 0, 
        b : 0
        }
    }
    
    // Constructor
    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn red(&self) -> u8 {
        return self.r;
    }

    pub fn green(&self) -> u8 {
        return self.r;
    }

    pub fn blue(&self) -> u8 {
        return self.r;
    }

    pub fn fmt(&self, rgb: &mut fmt::Formatter) -> fmt::Result {
        write!(
            rgb, 
            "red: {}, gree: {}, blue: {}", 
            self.r, self.g, self.b)
    }

    pub fn invert(&mut self)-> Pixel{
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
        let pixInvert = Pixel::new(self.r, self.g, self.b);
        return pixInvert
    }

    pub fn eq(&self, other: Pixel) -> bool {
        if self.r == other.r && self.g == other.g && self.b == other.b {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn to_grayscale(&mut self) -> Pixel{
        let average = self.r / 3 + self.g / 3 + self.b / 3;
        self.r = average;
        self.g = average;
        self.b = average;
        let pix_togray = Pixel::new(self.r, self.g, self.b);
        return pix_togray

    }
}

//*********************************************************************************


impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}


//*********************************************************************************

struct Image {
//An image is reprensented by a pixels matrix

    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image{


    //Default Constructor
    pub fn empty_color() -> Image {
        Image {width : 0, height : 0, pixels : Vec::new()}
        }

    //Constructor
    pub fn new(w:u32, h:u32, p:Vec<Pixel>) -> Image {
    Image {width : w, height : h, pixels : p}
    }

    //Image variables accesseurs

    pub fn width(&self) -> u32 {
    return self.width;
    }
    
    pub fn height(&self) -> u32 {
    return self.height;
    }
    
    pub fn pixels(&self) -> &Vec<Pixel> {
    return &self.pixels;
    }

    pub fn new_with_file(filename: &Path) -> Option<Image> {

        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut pixels: Vec<Pixel> = Vec::new();
        let file = File::open(filename).expect("Unable to open the File");
        let mut contents = String::new();
        let read_file = file.read(&mut contents);

        for (i, line) in read_file.lines().enumerate().by_ref() {
            // Treatment for the first line, if not P3 it's not a RGB picture => exit.
            if i == 0 {
                if &line.unwrap() != "P3" {
                    return None;
                }
                // The second line is the dimensions of the picture.
            } else if i == 1 {
                let list_num: Vec<u32> = Vec::new();
                width = list_num[0];
                height = list_num[1];
            }
        }
        return Some(Image::new(width, height, pixels));
    }

    pub fn invert(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i].r = 255 - self.pixels[i].red()();
            self.pixels[i].g = 255 - self.pixels[i].green();
            self.pixels[i].b = 255 - self.pixels[i].blue();
        }
    }

    /*
    pub fn invert_image(img : &Image) -> Image{
        let mut inv: Vec<Pixel> = Vec::new();
        for c in &img.pixels {
            inv.push(c.invert());
            }

        return Image::new(img.width, img.height, inv);

    }*/

    
}

//*********************************************************************************


//When you run your tests with the cargo test command, Rust builds a test runner 
//binary that runs the functions annotated with the test attribute and reports on
// whether each test function passes or fails.

#[cfg(test)]
mod tests {
    //Test for Pixel structure's functions

    //Test ok if the two pixels vector have differents values
    #[test]
    #[should_panic]
    fn test_struct_pixel() {
        let v = super::Pixel::new(255, 0, 0);
        let p = super::Pixel::new(0, 255, 255);
        assert_eq!(v,p)
        
    }

    //Test ok if the the first vector is the invert of the second one
    #[test]
    fn test_invert_pixel() {
        let v = super::Pixel::new(255, 0, 0);
        let mut p = super::Pixel::new(0, 255, 255);
        
        assert_eq!(v, p.invert())
    }

    //Test ok if the the two pixels vectors are equals
    #[test]
    fn test_eq_pixel(){
        let v = super::Pixel::new(255, 0, 0);
        let p = super::Pixel::new(255, 0, 0);

        assert!(v.eq(p))

    }

    //Test ok if the the two pixels vectors are different
    #[test]
    #[should_panic]
    fn test_noteq_pixel(){
        let v = super::Pixel::new(0, 255, 255);
        let p = super::Pixel::new(255, 0, 0);

        assert!(v.eq(p))

    }

    //test ok if the gray colors of the first pixel vector is identique to the second one values
    #[test]
    fn test_to_grayscale(){

        let mut v = super::Pixel::new(0, 255, 255);
        let p = super::Pixel::new(170, 170, 170); //Average of v = (0/3)+(255/3)+(255/3) = 170

        assert_eq!(v.to_grayscale(), p)
    }






    //Test for Image structure's functions

    

    
}