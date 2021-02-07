#ifndef _CLASS_SCENE_
#define _CLASS_SCENE_

#include <vector>
#include "./vec3.h"
#include "./sphere.h"

class scene
{
private:
    std::vector<sphere> spheres;
public:
    scene() {}

    scene(std::vector<sphere> _spheres) {
        spheres = _spheres;
    }

    void add_sphere(sphere sph) {
        spheres.push_back(sph);
    }

    bool collisions_detect(ray ray, intersect_info& info) {
        std::vector<float> distances;
        std::vector<sphere> true_sphere;

        for(int i = 0; i < spheres.size(); i++) {

            auto b = ray.get_direction().dot(ray.get_origin() - spheres[i].point);
            auto c = (ray.get_origin() - spheres[i].point).sqr_magnitude() - powf(spheres[i].radius, 2);
            auto D = powf(b, 2) - c;

            if (D < 0) continue;

            const auto ans0 = -b - std::sqrt(D);
            const auto ans1 = -b + std::sqrt(D);

            auto ans = ans0;
            if(ans < ray.get_min() || ans > ray.get_max()) {
                ans = ans1;

                if(ans < ray.get_min() || ans > ray.get_max()) {
                    continue;
                }
            }

            distances.push_back(ans);
            true_sphere.push_back(spheres[i]);
        }

        if(true_sphere.size() <= 0) return false;

        auto ans = ray.get_max();
        for(int i = 0; i < true_sphere.size(); i++) {
            if(distances[i] <= ans) {
                ans = distances[i];
                info.distance = ans;
                info.point = ray.return_t_point(ans);
                info.normal = (info.point - true_sphere[i].point).normalized();
                info.sph = &true_sphere[i];
            }
        }

        return true;
    }

    ~scene() {}
};


#endif //_CLASS_SCENE_