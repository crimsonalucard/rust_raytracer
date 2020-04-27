use crate::utils::math::tuple::{Environment, Projectile, tick, point, vector};
use crate::utils::color::{canvas, Color, canvas_to_ppm, write_to_canvas};

mod utils;

fn main() {
    let height = 500;
    let width = 500;
    let mut p = Projectile {
        position: point(0.0, 0.0, 0.0),
        velocity: vector(1.0, 4.0, 0.0),
    };
    let e = Environment {
        gravity: vector(0.0, -0.008, 0.0),
        wind: vector(0.0, 0.0, 0.0),
    };

    let filename = "test.ppm";
    let mut c = canvas(width, height, Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    });

    for _ in 0..500 {
        print!("{}, {}\n", p.position.value[0], p.position.value[1]);

        if p.position.value[1] > 0.0 && p.position.value[1] < height as f64 {
            // print!("{}, {}\n", p.position.value[0],   p.position.value[1]);
            write_to_canvas(p.position.value[0] as usize, (height - p.position.value[1] as usize) as usize, &mut c, Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            });
        }
        p = tick(&e, &p);
    }
    canvas_to_ppm(&c, filename);
}
