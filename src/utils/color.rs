use crate::utils::math::tuple::{Tuple4};
use std::ops::{Add, Neg, Mul, Sub};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
}

pub struct Canvas {
    pub x: usize,
    pub y: usize,
    pub canvas: Vec<Vec<Color>>,
}

pub fn canvas(x: usize, y: usize, default_color: Color) -> Canvas {
    let mut acc: Vec<Vec<Color>> = Vec::new();
    for _ in 0..y {
        let mut row: Vec<Color> = Vec::new();
        for _ in 0..x {
            row.push(default_color);
        }
        acc.push(row)
    }

    Canvas {
        x,
        y,
        canvas: acc,
    }
}

pub fn write_to_canvas(x: usize, y: usize, canvas: &mut Canvas, color: Color) {
    canvas.canvas[y][x] = color;
}


pub fn canvas_to_ppm(c: &Canvas, filename: &str) {
    let mut file = match File::create(filename) {
        Ok(filename) => filename,
        Err(E) => panic!(E)
    };
    let color_max = 255;
    write!(file, "P3\n");
    write!(file, "{} {}\n", c.x, c.y);
    write!(file, "{}\n", color_max);
    for r_index in 0..c.y {
        for l_index in 0..c.x {
            write!(file, "{} {} {}\n",
                   c.canvas[r_index][l_index].red * (color_max as f64),
                   c.canvas[r_index][l_index].green * (color_max as f64),
                   c.canvas[r_index][l_index].blue * (color_max as f64));
        }
    }
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color {
        red: r,
        green: g,
        blue: b,
    }
}

fn color_to_tuple(c: Color) -> Tuple4 {
    Tuple4 {
        value: [c.red, c.green, c.blue, 0.0]
    }
}

fn tuple_to_color(t: Tuple4) -> Color {
    color(t[0], t[1], t[2])
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        tuple_to_color(color_to_tuple(self) + color_to_tuple(rhs))
    }
}

impl Neg for Color {
    type Output = Color;
    fn neg(self) -> Self::Output {
        tuple_to_color(-color_to_tuple(self))
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        tuple_to_color(color_to_tuple(self) * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}



