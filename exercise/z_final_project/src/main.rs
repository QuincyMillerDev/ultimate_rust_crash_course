// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let app = App::new("final_project")
        .version("1.0")
        .about("A simple app that processes images")
        .subcommand(SubCommand::with_name("blur")
            .about("Blur the image")
            .arg(Arg::with_name("infile")
                .help("input file to blur")
                .required(true)
                .index(1))
            .arg(Arg::with_name("outfile")
                .help("file to save the blur image")
                .required(true)
                .index(2))
            .arg(Arg::with_name("amount")
                .help("amount to blur")
                .required(true)
                .index(3)))
        .subcommand(SubCommand::with_name("brighten")
            .about("Brighten the image")
            .arg(Arg::with_name("infile")
                .help("input file to brighten")
                .required(true)
                .index(1))
            .arg(Arg::with_name("outfile")
                .help("file to save the brighten image")
                .required(true)
                .index(2))
            .arg(Arg::with_name("amount")
                .help("amount to brighten")
                .required(true)
                .index(3)))
        .subcommand(SubCommand::with_name("crop")
            .about("Crop the image")
            .arg(Arg::with_name("infile")
                .help("input file to crop")
                .required(true)
                .index(1))
            .arg(Arg::with_name("outfile")
                .help("file to save the crop image")
                .required(true)
                .index(2))
            .arg(Arg::with_name("x")
                .help("x coordinate")
                .required(true)
                .index(3))
            .arg(Arg::with_name("y")
                .help("y coordinate")
                .required(true)
                .index(4))
            .arg(Arg::with_name("width")
                .help("width")
                .required(true)
                .index(5))
            .arg(Arg::with_name("height")
                .help("height")
                .required(true)
                .index(6)))
        .subcommand(SubCommand::with_name("rotate")
            .about("Rotate the image")
            .arg(Arg::with_name("infile")
                .help("input file to rotate")
                .required(true)
                .index(1))
            .arg(Arg::with_name("outfile")
                .help("file to save the rotate image")
                .required(true)
                .index(2))
            .arg(Arg::with_name("amount")
                .help("amount to rotate, 90, 180, 270 degrees")
                .required(true)
                .index(3)))
        .subcommand(SubCommand::with_name("invert")
            .about("Invert the image")
            .arg(Arg::with_name("infile")
                .help("input file to invert")
                .required(true)
                .index(1))
            .arg(Arg::with_name("outfile")
                .help("file to save the invert image")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("grayscale")
            .about("Grayscale the image")
            .arg(Arg::with_name("infile")
                .help("input file to grayscale")
                .required(true)
                .index(1))
            .arg(Arg::with_name("outfile")
                .help("file to save the grayscale image")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("generate")
            .about("Generate a solid color image")
            .arg(Arg::with_name("outfile")
                .help("file to save the generated image")
                .required(true)
                .index(1))
            .arg(Arg::with_name("color")
                .help("color to generate, e.g. 255 0 0")
                .required(true)
                .index(2)
                .multiple(true)))
        .subcommand(SubCommand::with_name("fractal")
            .about("Generate a fractal image")
            .arg(Arg::with_name("outfile")
                .help("file to save the fractal image")
                .required(true)
                .index(1)))
        .get_matches();

    if let Some(matches) = app.subcommand_matches("blur") {
        let infile = matches.value_of("infile").unwrap();
        let outfile = matches.value_of("outfile").unwrap();
        let amount: f32 = matches.value_of("amount").unwrap().parse().expect("Failed to parse a number");
        blur(infile.to_string(), outfile.to_string(), amount);
    } else if let Some(matches) = app.subcommand_matches("brighten") {
        let infile = matches.value_of("infile").unwrap();
        let outfile = matches.value_of("outfile").unwrap();
        let amount: i32 = matches.value_of("amount").unwrap().parse().expect("Failed to parse a number");
        brighten(infile.to_string(), outfile.to_string(), amount);
    } else {
        print_usage_and_exit();
    }
}


fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, amount: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (a f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(amount);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, amount: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(amount);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x : u32, y : u32, width : u32, height : u32) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    let img2 = img.crop(x, y, width, height);
    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, amount: u32) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    let img2 = img.clone();
    if amount == 90 {
        img2.rotate90();
    } else if amount == 180 {
        img2.rotate180();
    } else if amount == 270 {
        img2.rotate270();
    }

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    let mut img2 = img.clone();
    img2.invert();

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();
    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, color: [u8; 3]) {
    // Create an ImageBuffer -- see fractal() for an example
    let mut imgbuf = image::ImageBuffer::new(800, 800);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    // Set the image to some solid color. -- see fractal() for an example
        *pixel = image::Rgb(color);
    }
    imgbuf.save(outfile).expect("Failed writing OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
