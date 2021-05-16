pub mod image;
pub mod intersect_info;
pub mod material;
pub mod pinhole_camera;
pub mod ray;
pub mod rtao;
pub mod scene;
pub mod shapes;
pub mod vec3;

#[cfg(test)]
mod tests;

use core::f32;
use std::f32::consts::PI;
use std::rc::Rc;
use std::u32;

use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use ray::Ray;
use scene::Scene;
use shapes::bsdf::BSDF;
use vec3::{Color, Vec3, Vec3f};

pub struct Raytracer<'a> {
    max_depth: u32,
    scene: &'a Scene,
}

impl<'a> Raytracer<'a> {
    pub fn new(max_depth: u32, scene: &'a Scene) -> Self {
        Raytracer {
            max_depth,
            scene,
        }
    }
    
    pub fn pathtrace(&self, ray: Ray, index: u32, p: f32, sample: u32) -> Color {
        let mut result: Vec3f = Vec3f::from(0.0);
        let mut rng = thread_rng();
        let ray = Rc::new(ray);
        for _ in 0..sample {
            result = result + self.trace(ray.clone(), index, p, &mut rng, Vec3f::from(1.0));
        }

        result / sample as f32
    }

    fn trace(&self, ray: Rc<Ray>, index: u32, p: f32, rng: &mut ThreadRng, throughput: Vec3f) -> Color {
        if self.max_depth <= index {
            return Vec3::from(0.0);
        }
        
        if russian_roulette(p, rng) {
            return Vec3::from(0.0);
        }

        let throughput = throughput / p;
    
        if let Some(info) = self.scene.collision_detect(&ray) {
            let normal = info.normal;

            let (v2, v3) = normal.make_basis();

            let (bsdf, target_direction, pdf) = match info.target.get_bsdf() {
                BSDF::Lambert(lambert) => lambert.sample(rng),
            };

            let direction = Raytracer::local_to_world(
                target_direction,
                v2,
                normal,
                v3,
            );

            let cos = direction.dot(normal).max(0.0);
            
            let throughput = throughput * (bsdf * cos / pdf);

            return self.trace(
                Rc::new(Ray::new(info.point, direction)),
                index + 1,
                p,
                rng,
                throughput,
            );
        }

        Vec3f::from(1.0) * throughput
    }

    fn reflect(vec: Vec3f, normal_of_point: Vec3f) -> Vec3f {
        vec * -1.0 + normal_of_point * vec.dot(normal_of_point) * 2.0
    }

    //ベクトルを生で考えずに横成分と縦成分に分割して考えてる
    //https://i.imgur.com/vD5gz5h.png
    fn refract(in_vec: Vec3f, normal_of_point: Vec3f, in_ior: f32, out_ior: f32) -> Option<Vec3f> {
        let cos1 = in_vec.dot(normal_of_point);
        let in_vech = in_vec - normal_of_point * cos1;

        let out_vech = in_vech * -(in_ior / out_ior);

        if out_vech.magnitude() > 1.0 {
            return None;
        }

        let cos2 = (1.0 - out_vech.sqr_magnitude()).sqrt();
        let out_vecp = normal_of_point * -cos2;

        Some(out_vech + out_vecp)
    }

    fn make_ray_direction_with_important_sampling(u: f32, v: f32) -> (Vec3f, f32) {
        let theta = (1. / 2.) * (1. - 2. * u).clamp(-1.0, 1.0).acos();
        let phi = 2.0 * PI * v;

        let pdf: f32 = theta.cos() / PI;

        (Vec3::new(phi.cos() * theta.sin(), theta.cos(), phi.sin() * theta.sin()), pdf)
    }

    fn local_to_world(direction: Vec3f, lx: Vec3f, ly: Vec3f, lz: Vec3f) -> Vec3f {
        lx * direction.x + ly * direction.y + lz * direction.z
    }
}

pub fn gamma(k: f32) -> f32 {
    k.powf(1.0 / 2.2)
}

fn russian_roulette(p: f32, rng: &mut ThreadRng) -> bool {
    let u = rng.gen_range(0.0..=1.0);

    u > p
}
