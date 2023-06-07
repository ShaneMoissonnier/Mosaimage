<div align="center">
	<br>
	<br>
	<a href="https://github.com/ShaneMoissonnier/Mosaimage/blob/main/assets/logo.png"><img src="assets/logo.png" width="400" height="300"></a>
	<h1>Mosaimage</h1>
	<p>
	<p>📷 A mosaic generator, images built from thousands of other tiny images...</p>
	</p>
	<br>
</div>

**Mosaimage** is a fun project that generates an image using thousands of other tiny images. It's implemented in Rust and serves as a personal learning project to improve my knowledge of the language. The project is purely for fun and experimental purposes.

## How does it works?

**Mosaimage** takes an input image and generates a mosaic version of it by replacing each pixel of the original image with a small image tile from a collection of images. These image tiles can be anything - from random images to specific themed images.

The process involves the following steps:

1. **Image Processing**: The original input image is loaded and processed to prepare it for the mosaic creation.
2. **Tile Selection**: A collection of small image tiles is selected to create the mosaic. These tiles can be sourced from a predefined set or generated dynamically.
3. **Mosaic Generation**: Each pixel of the input image is analyzed, and the corresponding tile is selected from the collection based on color similarity.
4. **Mosaic Assembly**: The selected tiles are placed together to form the final mosaic image.

## Usage

To use **Mosaimage**, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/ShaneMoissonnier/Mosaimage.git
   ```

2. Build the project using Rust's package manager, Cargo:

   ```
   cd mosaimage
   cargo build --release
   ```

3. Prepare your input image and the collection of image tiles. Place them in the appropriate directories.

4. Run the program:

   ```
   cargo run --release -- -i /path/to/input/image.png -o /path/to/output/image.png -s /path/to/collection/of/images/
   ```

   Replace `/path/to/input/image.png` with the path to your desired input image, `/path/to/output/image.png` with the path of the result image and `/path/to/collection/of/images/` with the path of the folder containing the image collection to be used for the mosaic.

## Future Development

While Mosaimage is primarily a personal learning project, I may continue to work on it in the future. Possible areas for improvement and expansion include:

- **Refactoring**: The codebase may undergo refactoring to improve readability, maintainability, and adhere to best practices.
- **Optimization**: The performance of the mosaic generation algorithm could be optimized to handle larger images or improve efficiency.
- **User Interface**: Adding a user-friendly interface, such as a web-based interface or a graphical user interface (GUI), could make the project more accessible and enjoyable to use.
- **Additional Features**: I may explore adding additional features, such as image manipulation options, customization settings, or advanced mosaic generation algorithms.

## Contributing

As this project is primarily for personal learning and fun, contributions are not actively sought. However, if you have any suggestions, bug reports, or improvements you'd like to share, please feel free to open an issue or submit a pull request.

## License

Mosaimage is released under the [MIT License](https://github.com/ShaneMoissonnier/Mosaimage/blob/main/LICENSE). Feel free to modify and use the code as per the terms of the license.

## Acknowledgments

I would like to acknowledge the Rust community and its valuable resources, including documentation, tutorials, and open-source libraries, which have contributed to my learning process during the development of this project.