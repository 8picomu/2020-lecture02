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

use std::u32;

use material::Material;
use ray::Ray;
use rtao::RTAO;
use scene::Scene;
use vec3::{Vec3, Vec3f, Color};

pub struct Raytracer<'a> {
    max_depth: u32,
    scene: &'a Scene,
}

impl<'a> Raytracer<'a> {
    pub fn new(max_depth: u32, scene: &'a Scene) -> Self {
        Raytracer { max_depth, scene }
    }

    pub fn raytrace(&self, camera_ray: Ray, ao_sampling_point: u32, index: u32) -> Color {
        if self.max_depth <= index {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(info) = self.scene.collision_detect(&camera_ray) {
            match info.target.get_material() {
                Material::Mirror => {
                    return self.raytrace(
                        Ray::new(
                            info.point,
                            Raytracer::reflect(camera_ray.direction * -1.0, info.normal)
                                .normalized(),
                        ),
                        ao_sampling_point,
                        index + 1,
                    );
                }
                Material::Glass => {
                    let is_inside = (camera_ray.direction * -1.0).dot(info.normal) < 0.0;

                    if !is_inside {
                        //球体のガラスが内側からか外側からかで屈折率が変化する
                        //他にも法線の向きが逆に
                        if let Some(direction) =
                            Raytracer::refract(camera_ray.direction * -1.0, info.normal, 1.0, 1.5)
                        {
                            return self.raytrace(
                                Ray::new(info.point, direction.normalized()),
                                ao_sampling_point,
                                index + 1,
                            );
                        }

                        return Vec3::new(0.0, 0.0, 0.0);
                    }

                    if let Some(direction) = Raytracer::refract(
                        camera_ray.direction * -1.0,
                        info.normal * -1.0,
                        1.5,
                        1.0,
                    ) {
                        return self.raytrace(
                            Ray::new(info.point, direction.normalized()),
                            ao_sampling_point,
                            index + 1,
                        );
                    }

                    return Vec3::new(0.0, 0.0, 0.0);
                }
                _ => (),
            }

            let directional_light_ray = Ray::new(info.point, self.scene.directional_light);
            let ray_info = self.scene.collision_detect(&directional_light_ray);

            let rtao = RTAO::new(100, 10.0);

            let kdao = info.target.get_kd() * 0.1 * (1.0 - rtao.rtao(self.scene, &info));

            if let Some(ray_info) = ray_info {
                if ray_info.target.get_material() == Material::Glass {
                    return info.target.get_kd()
                        * self.scene.directional_light.dot(info.normal).max(0.0)
                        + kdao;
                } else {
                    return kdao;
                }
            } else {
                return info.target.get_kd()
                    * self.scene.directional_light.dot(info.normal).max(0.0)
                    + kdao;
            }
        }

        Vec3::new(0.0, 0.0, 0.0)
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
}

pub fn gamma(k: f32) -> f32 {
    k.powf(1.0 / 2.2)
}
