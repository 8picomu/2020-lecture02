use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use num::Float;

use crate::vec3::*;

struct Image {
    width: usize,
    height: usize,
    canvas: Vec<f32>,
    canvasArraySize: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let arraySize = 3 * width * height;

        Image {
            width: width,
            height: height,
            canvas: Vec::with_capacity(arraySize as usize),
            canvasArraySize: arraySize,
        }
    }

    pub fn get_size(&self) -> (&usize, &usize) {
        (&self.width, &self.height)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: &Vec3<f32>) {
        let target_pixel_index = 3 * x + 3 * self.width * y;

        self.canvas[target_pixel_index] = rgb.get_x();
        self.canvas[target_pixel_index + 1] = rgb.get_y();
        self.canvas[target_pixel_index + 2] = rgb.get_z();
    }

    pub fn write_ppm(&self, output_name: &str) -> io::Result<()> {
        let mut f = File::create(output_name)?;
        let mut writer = BufWriter::new(f);

        writer.write_all(b"P3\r\n")?;
        writer.write_all(&format!("{} {}\r\n", self.width, self.height).as_bytes())?;
        writer.write_all(b"255\r\n")?;

        for j in 0..self.height {
            for i in 0..self.width {
                let index = 3 * i + self.width * j;
                let r = self.canvas[index];
                let g = self.canvas[index + 1];
                let b = self.canvas[index + 2];

                writer.write_all(&format!("{} ", r).as_bytes())?;
                writer.write_all(&format!("{} ", g).as_bytes())?;
                writer.write_all(&format!("{}\r\n", b).as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn gamma_set(&mut self) {
        for index in 0..self.canvasArraySize {
            self.canvas[index] = self.canvas[index].powf(1.0 / 2.2);
        }
    }
}
