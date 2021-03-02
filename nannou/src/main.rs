use nannou::{prelude::*};

#[derive(Debug, Clone, Copy)]
struct Pixel {
    w: f32,
    h: f32,
    x: f32,
    y: f32,
}
impl Pixel {
    fn new( w: f32, h: f32, x: f32, y: f32 ) -> Pixel {
        Pixel {
            w,
            h,
            x,
            y,
        }
    }
    fn coord(self) -> (f32, f32) {
        (self.x * self.w,  self.y * self.h)
    }
}

fn main() {
    nannou::sketch(view).run();
}
fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    // set background to white
    draw.background().color(WHITE);
    // size of a pixel
    let pxl_size:(f32, f32) = (3.0, 3.0);
    // map size
    let map:(i32,i32) = (200, 200);
    // nodes
    let node_amount = 40;
    let mut nodes:Vec<(f32,f32)> = vec![];
    //?let mut closest_point:(f32,f32) = (-1.0,-1.0);
    // draw a pixel for every x and y in the map size
    if &frame.nth() < &1 {
        for _ in 0..node_amount {
            let x: i32 = (random::<f32>() * map.0 as f32).floor() as i32;
            let y: i32 = (random::<f32>() * map.1 as f32).floor() as i32;
            nodes.push((x as f32, y as f32));
        }
        for x in 0..map.0 {
            for y in 0..map.1 {
                let mut dist:f32 = -1.0;
                for i in 0..nodes.len() {
                    if dist == -1.0 as f32 {
                        dist = distance(nodes[i], (x as f32, y as f32));
                    }
                    else if dist > distance((x as f32, y as f32), nodes[i]) {
                        dist = distance(nodes[i], (x as f32, y as f32));
                    }
                }
                let level = dist / 60.0;
                let level = (level - 1.0).abs();
                let mut pxl_rgb = (0.0, 0.0, 0.0);
                
                if level < 0.7 { 
                    // ocean
                    pxl_rgb = (0.086 - level, 0.573 - level, 0.969 - level);
                }
                else if level < 0.8 {
                    // land
                    pxl_rgb = (0.3 + 0.0 - level, 0.3 + 0.859 - level, 0.3 + 0.226 - level);
                }
                else if level < 0.95 {
                    pxl_rgb = (0.35 + 0.0 - (level + (1.0-level)/4.0), 0.35 + 0.859 - (level + (1.0-level)/4.0), 0.35 + 0.226 - (level + (1.0-level)/4.0));
                }
                else {
                    let level: f32 = 0.95;
                    pxl_rgb = (0.35 + 0.0 - (level + (1.0-level)/4.0), 0.35 + 0.859 - (level + (1.0-level)/4.0), 0.35 + 0.226 - (level + (1.0-level)/4.0));
                }
                // new pixel with set size and location from the loop
                let pixel = Pixel::new(pxl_size.0, pxl_size.1, x as f32 - (map.0 / 2) as f32,y as f32 - (map.1 / 2) as f32);
                // get the pixel's coord compared to the size as a tuple from the pixel initialized from before
                let pixel_coord = &pixel.coord();
                // using the previous info, draw the pixel
                draw.rect().color(rgb(pxl_rgb.0 + 0.2, pxl_rgb.1 + 0.2, pxl_rgb.2 + 0.2)) 
                    .w(pixel.w).h(pixel.h) 
                    .x(pixel_coord.0).y(pixel_coord.1);
            }
        }
        // Write to the window frame.
        println!("frame: {}", &frame.nth());
        draw.to_frame(app, &frame).unwrap();
    }
}
fn distance(pt1:(f32,f32),  pt2:(f32,f32)) -> f32 {
    ((pt2.0 - pt1.0).powi(2) + (pt2.1 - pt1.1).powi(2)).sqrt()
}