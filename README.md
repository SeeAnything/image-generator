# Image Generators

> Image generators for the See Anything project ([SeeAnything.org](http://seeanything.org/))

## Background

Start with a black image and increment each pixel until the image becomes white. Then you will have created every image possible within that frame. 

The See Anything project is an exercise in (combinatorics)[https://en.wikipedia.org/wiki/Combinatorics] that aims to use different techniques to express as many images as possible through image generator algorithms.

## Generators

The current implementation has three image generators:

* **Linear** - 320x240 raster with 8-bit greyscale pixels, linearly generating images by incrementing each pixel until the entire raster is filled. This technique will take trillions of years to complete. Number of possible images is 256<sup>76,800</sup>.

* **Linear 1-Bit** 320x240 raster with 1-bit pixels, where each pixel is either black or white. Increments in the same way as the Linear Generator but will complete sooner as it has a reduced number set. Number of possible images is 2<sup>76,800</sup>.

* **Random 8-bit**  320x240 raster with 8-bit greyscale pixels, with each pixel's value set to a pseudo-random number from 0 to 255. Images are not based on a particular set and may repeat. Number of possible images is 256<sup>76,800</sup>.

## How it works

The image generator code runs on a server, with each generator working on its own thread and producing one image per generator at a time. When a generator creates a new image, the previous image is overwritten. These images are served on the webpages at [SeeAnything.org](http://seeanything.org/).

## Building

* Install (Rust)[https://www.rust-lang.org/]
* `cd` into the project directory
* Change the file path constants in `main()` as needed 
* Run `cargo build` to compile without running (optional)
* Run `cargo run` to run the code (Press Ctrl+C to stop)

## Contributing

Both the (SeeAnything website)[https://github.com/SeeAnything/seeanything.org] and the image generators are open for pull requests.