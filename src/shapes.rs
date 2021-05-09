use crate::intersect_info::IntersectInfo;
use crate::material::Material;
use crate::ray::{Ray, TMAX, TMIN};
use crate::vec3::{Color, Vec3f};

use self::sphere::Sphere;
pub mod sphere;

use self::triangle::Triangle;
pub mod triangle;

use self::obj::Obj;
pub mod obj;

//できるだけ動的ディスパッチをしないようにするため
#[derive(Debug)]
pub enum Shapes {
    Sphere(Sphere),
    Triangle(Triangle),
    Obj(Obj),
}

const K_EPSILON: f32 = 1e-8;

impl Shapes {
    //collisiotn_detectはそれぞれが持つべき(?)
    pub fn collision_detect(&self, ray: &Ray) -> Option<IntersectInfo> {
        match self {
            Shapes::Sphere(sphere) => {
                let c_to_o = ray.origin - sphere.center_position;
                let b = ray.direction.dot(c_to_o);
                let c = c_to_o.sqr_magnitude() - num::pow(sphere.radius, 2);
                #[allow(non_snake_case)]
                let D = num::pow(b, 2) - c;

                if D < 0.0 {
                    return None;
                }

                let mut ans = -b - D.sqrt();
                if ans < TMIN || TMAX < ans {
                    ans = -b + D.sqrt();

                    if ans < TMIN || TMAX < ans {
                        return None;
                    }
                }

                let hit_position = ray.point_on_ray(ans);

                Some(IntersectInfo::new(
                    ans,
                    hit_position,
                    (hit_position - sphere.center_position).normalized(),
                    self,
                ))
            }
            Shapes::Triangle(triangle) => {
                let e1 = triangle.v1 - triangle.v0;
                let e2 = triangle.v2 - triangle.v0;

                let alpha = ray.direction.cross(e2);

                let det = e1.dot(alpha);
                if -K_EPSILON < det && det < K_EPSILON {
                    return None;
                }

                let inv_det = 1.0 / det;
                let r = ray.origin - triangle.v0;

                let u = alpha.dot(r) * inv_det;
                if !(0.0 ..=1.0).contains(&u) {
                    return None;
                }

                let beta = r.cross(e1);

                let v = ray.direction.dot(beta) * inv_det;
                if v < 0.0 || u + v > 1.0 {
                    return None;
                }

                let ans = e2.dot(beta) * inv_det;
                if !(TMIN..=TMAX).contains(&ans) {
                    return None;
                }

                let hit_position = ray.point_on_ray(ans);

                Some(IntersectInfo::new(
                    ans,
                    hit_position,
                    (e1.cross(e2)).normalized(),
                    self,
                ))
            }
            Shapes::Obj(_) => {
                //TODO: BVH
                None
            }
        }
    }

    // TODO: トレイトとしてまとめる
    //SphereやReactangleに対してShapeのようなトレイトを用意
    pub fn get_center_position(&mut self) -> Vec3f {
        match self {
            Shapes::Sphere(sphere) => sphere.center_position,
            Shapes::Triangle(triangle) => triangle.get_center_position(),
            Shapes::Obj(obj) => obj.center_position,
        }
    }

    pub fn get_material(&self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.material,
            Shapes::Triangle(triangle) => triangle.material,
            Shapes::Obj(obj) => obj.material,
        }
    }

    pub fn get_kd(&self) -> Color {
        match self {
            Shapes::Sphere(sphere) => sphere.kd,
            Shapes::Triangle(triangle) => triangle.kd,
            Shapes::Obj(obj) => obj.kd,
        }
    }
}
