#ifndef _CLASS_SPHERE_
#define _CLASS_SPHERE_

#include "./vec3.h"
#include "./ray.h"
#include "./intersect_info.h"

enum class sphere_material { mirror, glass, diffuse };

class sphere
{
public:
    vec3f point;
    float radius;
    sphere_material material;
    vec3f rgb;

    sphere(vec3f _point, float _radius, sphere_material _material, vec3f _rgb) {
        point = _point;
        radius = _radius;
        material = _material;
        rgb = _rgb;
    }

    bool collision_detect(ray ray, intersect_info& info)  {
        auto b = ray.get_direction().dot(ray.get_origin() - point);
        auto c = (ray.get_origin() - point).sqr_magnitude() - powf(radius, 2);
        auto D = powf(b, 2) - c;

        if (D < 0) return false;

        const auto ans0 = -b - std::sqrt(D);
        const auto ans1 = -b + std::sqrt(D);

        //許容交差
        auto ans = ans0;
        if(ans < ray.get_min() || ans > ray.get_max()) {
            ans = ans1;

            if(ans < ray.get_min() || ans > ray.get_max()) {
                return false;
            }
        }

        info.distance = ans;
        info.point = ray.return_t_point(ans);
        info.normal = (info.point - point).normalized();
        info.sph = this;

        return true;
    }

    ~sphere() {}
};

void display(intersect_info& info) {
    std::cout << info.distance << std::endl;
    std::cout << info.point.get_x() << " " << info.point.get_y() << " " << info.point.get_z() << std::endl;
    std::cout << info.normal.get_x() << " " << info.normal.get_y() << " " << info.normal.get_z() << std::endl;
    std::cout << info.sph->radius << std::endl;
}

#endif // _CLASS_SPHERE_