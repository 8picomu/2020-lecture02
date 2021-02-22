extern crate image;

use core::f32;
use std::sync::{Arc, Mutex};
use std::thread;

use rand::{thread_rng, Rng};
use raytracer::material::*;
use raytracer::pinhole_camera::*;
use raytracer::scene::*;
use raytracer::sphere::*;
use raytracer::vec3::*;
use raytracer::*;

fn main() {
    raytrace_ssaa(512, 512, "output.png", 16);
}

fn raytrace_ssaa(width: u32, height: u32, path: &str, sampling_point: u32) {
    let image = image::ImageBuffer::new(width, height);

    let mut scene: Scene = Scene::new_without_spheres(Vec3::new(0.5, 1.0, 0.5).normalized());

    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Diffuce,
        Vec3::new(0.9, 0.9, 0.9),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(3.0, 1.0, 2.0),
        1.0,
        Material::Glass,
        Vec3::new(0.0, 0.0, 0.0),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-2.0, 0.0, 1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.8, 0.2, 0.2),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.2, 0.8, 0.2),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(2.0, 0.0, -1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.2, 0.2, 0.8),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-2.0, 3.0, 1.0),
        1.0,
        Material::Mirror,
        Vec3::new(1.0, 1.0, 1.0),
    ));

    let camera = Arc::new(PinholeCamera::new(
        Vec3::new(4.0, 1.0, 7.0),
         Vec3::from(0.0) - Vec3::new(4.0, 1.0, 7.0).normalized(),
        1.0,
    ));
    let image = Arc::new(Mutex::new(image));
    let scene = Arc::new(scene);
    let mut handles = vec![];

    for j in 0..height {
        let (image, camera, scene) = (image.clone(), camera.clone(), scene.clone());
        handles.push(thread::spawn(move || {
            for i in 0..width {
                let mut rng = thread_rng();
  
                let kd: Color = (0..sampling_point).map(|_| {
                    let u = (2.0 * (i as f32 + rng.gen_range(0.0..1.0)) - width as f32) / height as f32;
                    let v = (2.0 * (j as f32 + rng.gen_range(0.0..1.0)) - height as f32) / height as f32;

                    let ray = camera.make_ray_to_pinhole(u, v);
                    let raytracer = Raytracer::new(100, &scene);
                    raytracer.raytrace(ray, 0)
                }).collect::<Vec<_>>().into_iter().fold(Vec3::from(0.0), |sum, color| sum + color) / sampling_point as f32;

                let kd = Vec3::new(gamma(kd.x), gamma(kd.y), gamma(kd.z));
                
                let r = num::clamp(kd.x * 255.0, 0.0, 255.0) as u8;
                let g = num::clamp(kd.y * 255.0, 0.0, 255.0) as u8;
                let b = num::clamp(kd.z * 255.0, 0.0, 255.0) as u8;

                let mut image = image.lock().unwrap();
                let pixel = image.get_pixel_mut(i, j); 
                *pixel = image::Rgb([r, g, b]);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let image = image.lock().unwrap();
    image.save(path).unwrap();
}
