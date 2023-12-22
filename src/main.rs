use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::{Duration, Instant};

pub const WIDTH: usize = 800; // screen width
pub const HEIGHT: usize = 600; // screen height
const FPS: u32 = 120; // maximum FPS


mod draw {
    use crate::{WIDTH, HEIGHT};
    use image::GenericImageView;
    //#[allow(dead_code)]

    // Funcs - - -

    fn draw_to_buffer(buffer: &mut Vec<u32>, index: usize, color: u32) {
        if index as usize > buffer.len() {
            panic!("Buffer too small for index! Index: {}, Buffer: {}", index, buffer.len())
        }
        buffer[index] = color;
    }

    #[allow(dead_code)]
    pub fn square(buffer: &mut Vec<u32>, start_x: i32, start_y: i32, sq_width: i32, sq_height: i32, color: u32) {
        for y in 0..sq_height {
            for x in 0..sq_width {
                let index = (start_y + y) * WIDTH as i32 + start_x + x;
                
                draw_to_buffer(buffer, index as usize, color)
            }
        }
    }

    fn coordinates_to_index(x: usize, y: usize) -> usize {
        return y * WIDTH + x
    }

    fn bresenham_low(buffer: &mut Vec<u32>, x0: i32, x1: i32, y0: i32, y1: i32, color: u32) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy
        }
        let mut d = (2 * dy) - dx;
        let mut y = y0;
    
        for x in x0..=x1 {
            draw_to_buffer(buffer, coordinates_to_index(x as usize, y as usize), color);
            if d > 0 {
                y = y + yi;
                d = d + (2 * (dy - dx))
            } else {
                d = d + 2*dy;
                }
            }
        }

    fn bresenham_high(buffer: &mut Vec<u32>, x0: i32, x1: i32, y0: i32, y1: i32, color: u32) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx
        }
        let mut d = (2 * dx) - dy;
        let mut x = x0;

        for y in y0..y1 {
            draw_to_buffer(buffer, coordinates_to_index(x as usize, y as usize), color);
            if d > 0 {
                x = x + xi;
                d = d + (2 * (dx - dy))
            } else {
                d = d + 2*dx
            }
        }
    }

    #[allow(dead_code)]
    pub fn line(buffer: &mut Vec<u32>, x0: i32, x1: i32, y0: i32, y1: i32, color: u32) {
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                bresenham_low(buffer, x0 as i32, x1 as i32, y0 as i32, y1 as i32, color)
            } else {
                bresenham_low(buffer, x0 as i32, x1 as i32, y0 as i32, y1 as i32, color)
            }
        } else {
            if y0 > y1 {
                bresenham_high(buffer, x0 as i32, x1 as i32, y0 as i32, y1 as i32, color)
            } else {
                bresenham_high(buffer, x0 as i32, x1 as i32, y0 as i32, y1 as i32, color)
            }
        }
    }

    #[allow(dead_code)]
    pub fn image(buffer: &mut Vec<u32>, path: &str, x: usize, y: usize) {
        let img = image::open(path).expect("Failed to open image");

        // Ensure image fits within the buffer
        if x + img.width() as usize > WIDTH || y + img.height() as usize > HEIGHT {
            return;
        }
    
        for i in 0..img.height() {
            for j in 0..img.width() {
                let pixel = img.get_pixel(j, i);
                let index_buffer = (y + i as usize) * WIDTH + (x + j as usize);
                // Convert RGB values to u32 (ARGB format)
                draw_to_buffer(buffer, index_buffer, ((pixel[3] as u32) << 24)
                    | ((pixel[0] as u32) << 16)
                    | ((pixel[1] as u32) << 8)
                    | pixel[2] as u32)
            }
        }

}

}

fn main() {
    // create rng instance
    let mut rng = rand::thread_rng();

    // fps logic variables
    let frame_time = Duration::from_secs(1) / FPS;
    let mut last_frame_time = Instant::now();

    // Create a window
    let mut window = Window::new(
        "Rust Pixel Drawing",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    // Buffer to store pixel data
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Clear the buffer (optional)
        buffer.iter_mut().for_each(|pixel| *pixel = 0);

        // Test code for performance or something
        for _ in 0..1000 {
            draw::line(&mut buffer, rng.gen_range(0..WIDTH) as i32, rng.gen_range(0..WIDTH) as i32, rng.gen_range(0..HEIGHT) as i32, rng.gen_range(0..HEIGHT) as i32, 0xFFFF0000);
        } 

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                println!("Window update failed: {}", e);
            });

        // FPS lock
        let elapsed_time = Instant::now().duration_since(last_frame_time);
        if elapsed_time < frame_time {
            let sleep_time = frame_time - elapsed_time;
            std::thread::sleep(sleep_time); 
        }

        last_frame_time = Instant::now();

    }
}
