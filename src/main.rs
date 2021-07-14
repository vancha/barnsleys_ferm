use rand::Rng;
//use num_traits::pow::Pow;


///sets the width of the resulting generated image
const IMG_WIDTH: u32 = 1920;
///sets the height of the generated image
const IMG_HEIGHT: u32 = 1080;
///decides on the number of points generated to create the plot in the image
const NUMBER_OF_POINTS: u32 = 100000;
///the fraction used in the chaos game algorithm
const FRACTION: f32 = 1. / 2.;
///the number of sides of the polygon to generate the fractal of
const NR_SIDES: i32 = 5;

fn main() {
    let mut img_buffer = image::ImageBuffer::new(IMG_WIDTH + 1, IMG_HEIGHT + 1);
    let mut random_numer_generator = rand::thread_rng();
    
    let mut x:f32 = 0.;
    let mut y:f32 = 0.;
    for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([255u8, 255u8, 255u8]);
    }

    for _ in 0..NUMBER_OF_POINTS {
          img_buffer.put_pixel((72. * x + (IMG_WIDTH as f32/ 2.)) as u32, (72. * y+ 200.) as u32, image::Rgb([0u8,200u8,0u8]));
         
          let xn = x;
          let yn = y;
          match random_numer_generator.gen_range(0..101) {
          r if r < 1  => {
              x = 0.;
              y = 0.16 * y;
          },
          r if r > 1 && r < 86 => {
              x = 0.85 * xn + 0.04 * yn;
              y = -0.04 * xn + 0.85 * yn + 1.6;
          },
          r if r < 93 && r > 86 => {
              x = 0.20 * xn - 0.26 * yn;
              y = 0.23 * xn + 0.22 * yn + 1.6;
          },
          _ => {
              x = -0.15 * xn + 0.28 * yn;
              y = 0.26 * xn + 0.24 * yn + 0.44
          }
          }
    }
    //save the image to disk as a png
    img_buffer.save("fractal.png").unwrap();
}
