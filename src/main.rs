use image::ImageBuffer;


/* 
 * Returns a 512 x 512 pixel RgbImage of the Mandelbrot set.
 * Use x in [-2.5, 1] and y in [-1, 1].
 * Translation of the pseudocode at https://en.wikipedia.org/wiki/Mandelbrot_set.
 */

fn find_mandelbrot(x0:f64, y0:f64) -> f64{
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let mut iter_: f64 = 0.0;
    let max_iter: f64 = 1000.0; 
    while x*x + y*y <= 4.0  && iter_ < max_iter{
        let xtemp: f64 = x*x - y*y + x0;
        y = 2.0*x*y + y0;
        x = xtemp;
        iter_ += 1.0f64;
    }
    iter_ / 1000.0f64 * 255.0f64
}

// scale x coordinate
fn x_scaled(x: u32, width: u32) -> f64{
    x as f64 / width as f64 * 3.5f64 - 2.5f64
}

// scale y coordinate
fn y_scaled(y: u32, height: u32) -> f64{
    y as f64 / height as f64 * 2.0f64 - 1.0f64

}

fn main() {
    let mut image = ImageBuffer::new(512, 512);
    let (width, height) = image.dimensions();
    for x in 0..width{ // for row in rows
        for y in 0..height{ // for pixel in row
            let x0 = x_scaled(x, width);
            let y0 = y_scaled(y, height);
            let n: f64 = find_mandelbrot(x0, y0);
            let pixel = image.get_pixel_mut(x, y);
            *pixel = image::Rgb([n.floor() as u8, n.floor() as u8, n.floor() as u8]);
        }
    }

    image.save("mandelbrot.png").unwrap()
}
