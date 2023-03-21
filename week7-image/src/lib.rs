//! An example of generating solar system using image crate
extern crate image;
extern crate num_complex;

pub fn draw_solar_system() {
    let imgx = 1000;
    let imgy = 1000;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Create the plot
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = x as f32 - imgx as f32 / 2.0;
            let cy = y as f32 - imgy as f32 / 2.0;
            let radius = (f32::powf((cx as f32), 2.0) + f32::powf((cy as f32), 2.0)).sqrt() / 1.5;
            
            let mut rgb = [0, 0, 0] as [u8; 3];
            if radius < 1.0 {
                // draw sun
                rgb = [255, 0, 0];
            } else if 3.5 <= radius && radius <= 4.5 {
                // draw mercury
                rgb = [0, 0, 255];
            } else if 6.5 <= radius && radius <= 7.5 {
                // draw venus
                rgb = [255, 0, 255];
            } else if 9.5 <= radius && radius <= 10.5 {
                // draw earth
                rgb = [0, 255, 255];
            } else if 14.5 <= radius && radius <= 15.5 {
                // draw mars
                rgb = [255, 0, 255];
            } else if 51.5 <= radius && radius <= 52.5 {
                // draw jupiter
                rgb = [255, 255, 0];
            } else if 95.5 <= radius && radius <= 96.5 {
                // draw saturn
                rgb = [0, 255, 255];
            } else if 191.5 <= radius && radius <= 192.5 {
                // draw uranus
                rgb = [255, 255, 255];
            } else if 299.5 <= radius && radius <= 300.5 {
                // draw neptune
                rgb = [0, 255, 0];
            } else {
                rgb = [0, 0, 0];
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb(rgb);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("solar_system.png").unwrap();
}
