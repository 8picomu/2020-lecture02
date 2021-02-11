extern crate image;

use core::f32;
use std::sync::{Arc, Mutex};
use std::thread;

use raytracer::image::*;
use raytracer::material::*;
use raytracer::pinhole_camera::*;
use raytracer::scene::*;
use raytracer::sphere::*;
use raytracer::vec3::*;
use raytracer::*;

fn main() {
    //image_test(1024, 512, "output.ppm");
    //pinhole_camera_test(1024, 512, "output.ppm");
    //sphere_test(1024, 512, "output.ppm");
    //scene_test(1024, 512, "output.ppm");
    //raytrace_test(1024, 512, "output.ppm");
    raytrace_test_with_image_crate(1024, 512, "output.png");
}

fn raytrace_test_with_image_crate(width: u32, height: u32, path: &str) {
    let image = image::ImageBuffer::new(width, height);

    let mut scene: Scene = Scene::new_without_spheres(Vec3::new(0.5, 1.0, 0.5).normalized());

    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Diffuce,
        Vec3::new(0.9, 0.9, 0.9),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 2.0, 3.0),
        1.0,
        Material::Glass,
        Vec3::new(0.0, 0.0, 0.0),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, 1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(1.0, 0.0, 0.0),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.0, 1.0, 0.0),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.0, 0.0, 1.0),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-2.0, 2.0, 1.0),
        1.0,
        Material::Mirror,
        Vec3::new(1.0, 1.0, 1.0),
    ));

    let camera = Arc::new(PinholeCamera::new(
        Vec3::new(0.0, 2.0, 8.0),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
    ));
    let image = Arc::new(Mutex::new(image));
    let scene = Arc::new(scene);
    let mut handles = vec![];

    for j in 0..height {
        let (image, camera, scene) = (image.clone(), camera.clone(), scene.clone());
        handles.push(thread::spawn(move || {
            for i in 0..width {
                let u = (2.0 * i as f32 - width as f32) / width as f32;
                let v = (2.0 * j as f32 - height as f32) / width as f32;

                let ray = camera.make_ray_to_pinhole(u, v);
                let raytracer = Raytracer::new(100, &scene);
                
                let kd = raytracer.raytrace(ray, 0);
                
                let r = num::clamp(kd.x.powf(1.0 / 2.2) * 255.0, 0.0, 255.0) as u8;
                let g = num::clamp(kd.y.powf(1.0 / 2.2) * 255.0, 0.0, 255.0) as u8;
                let b = num::clamp(kd.z.powf(1.0 / 2.2) * 255.0, 0.0, 255.0) as u8;
                
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

fn raytrace_test(width: usize, height: usize, path: &str) {
    let image = Image::new(width, height);
    let canvas_size = image.get_size();

    let image = Arc::new(Mutex::new(image));

    let camera = Arc::new(PinholeCamera::new(
        Vec3::new(0.0, 2.0, 8.0),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
    ));

    let mut scene: Scene = Scene::new_without_spheres(Vec3::new(0.5, 1.0, 0.5).normalized());

    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Diffuce,
        Vec3::new(0.9, 0.9, 0.9),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 2.0, 3.0),
        1.0,
        Material::Glass,
        Vec3::new(0.0, 0.0, 0.0),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, 1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(1.0, 0.0, 0.0),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.0, 1.0, 0.0),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.0, 0.0, 1.0),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-2.0, 2.0, 1.0),
        1.0,
        Material::Mirror,
        Vec3::new(1.0, 1.0, 1.0),
    ));

    let scene = Arc::new(scene);
    let mut handles = vec![];

    for j in 0..canvas_size.1 {
        let (image, camera, scene) = (image.clone(), camera.clone(), scene.clone());
        handles.push(thread::spawn(move || {
            for i in 0..canvas_size.0 {
                let u = (2.0 * i as f32 - canvas_size.0 as f32) / canvas_size.0 as f32;
                let v = (2.0 * j as f32 - canvas_size.1 as f32) / canvas_size.0 as f32;

                let ray = camera.make_ray_to_pinhole(u, v);
                let raytracer = Raytracer::new(100, &scene);

                image
                    .lock()
                    .unwrap()
                    .set_pixel(i, j, raytracer.raytrace(ray, 0));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut image = image.lock().unwrap();

    image.gamma_set();

    image.write_ppm(path).expect("failed to write ppm");
}

fn scene_test(width: usize, height: usize, path: &str) {
    let mut image = Image::new(width, height);
    let canvas_size = image.get_size();

    let camera = PinholeCamera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), 1.0);

    let mut scene: Scene = Scene::new_without_spheres(Vec3::new(0.5, 1.0, 0.5).normalized());
    scene.add_sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, 1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(1.0, 1.0, 1.0),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Diffuce,
        Vec3::new(1.0, 1.0, 1.0),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(1.0, 1.0, 1.0),
    ));

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            let u = (2.0 * i as f32 - canvas_size.0 as f32) / canvas_size.0 as f32;
            let v = (2.0 * j as f32 - canvas_size.1 as f32) / canvas_size.0 as f32;

            let ray = camera.make_ray_to_pinhole(u, v);
            if let Some(info) = scene.collision_detect(&ray) {
                image.set_pixel(i, j, info.normal * 0.5 + Vec3::new(0.5, 0.5, 0.5));
            } else {
                image.set_pixel(i, j, Vec3::new(0.0, 0.0, 0.0));
            }
        }
    }

    image.write_ppm(path).expect("failed to write ppm");
}

fn sphere_test(width: usize, height: usize, path: &str) {
    let mut image = Image::new(width, height);
    let canvas_size = image.get_size();

    let camera = PinholeCamera::new(Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, -1.0), 1.0);

    let sphere = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Diffuce,
        Vec3::new(1.0, 1.0, 1.0),
    );

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            let u = (2.0 * i as f32 - canvas_size.0 as f32) / canvas_size.0 as f32;
            let v = (2.0 * j as f32 - canvas_size.1 as f32) / canvas_size.0 as f32;

            let ray = camera.make_ray_to_pinhole(u, v);

            if let Some(info) = sphere.collision_detect(ray) {
                image.set_pixel(i, j, info.normal * 0.5 + Vec3::new(0.5, 0.5, 0.5));
            } else {
                image.set_pixel(i, j, Vec3::new(0.0, 0.0, 0.0));
            }
        }
    }

    image.write_ppm(path).expect("failed to write ppm");
}

//(2.0f * i - width) / widthは(i - width / 2.0f) / widthでも問題ない？
//widthが512の場合* 2.0f - widthで-512から512を取るようになるそれを512で割って[-1, 1]
fn pinhole_camera_test(width: usize, height: usize, path: &str) {
    let mut image = Image::new(width, height);
    let canvas_size = image.get_size();

    let camera = PinholeCamera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 1.0);

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            let u = (2.0 * i as f32 - canvas_size.0 as f32) / canvas_size.0 as f32;
            let v = (2.0 * j as f32 - canvas_size.1 as f32) / canvas_size.0 as f32;
            let ray = camera.make_ray_to_pinhole(u, v);
            //println!("u: {}, v: {}", u, v);

            image.set_pixel(i, j, (ray.direction + Vec3::new(1.0, 1.0, 1.0)) * 0.5);
        }
    }
    image.write_ppm(path).expect("failed to write ppm");
}

fn image_test(width: usize, height: usize, path: &str) {
    let mut image = Image::new(width, height);
    let canvas_size = image.get_size();

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            image.set_pixel(
                i,
                j,
                Vec3::new(
                    i as f32 / canvas_size.0 as f32,
                    j as f32 / canvas_size.1 as f32,
                    1.0,
                ),
            );
        }
    }
    image.write_ppm(path).expect("failed to write ppm");
}
