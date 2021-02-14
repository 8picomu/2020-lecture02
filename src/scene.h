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

        bool hit = false;
        intersect_info info_iter;

        info.distance = ray.get_max();
        for(auto& sphere: spheres) {
            if(sphere.collision_detect(ray, info_iter) && info_iter.distance < info.distance) {
                hit = true;
                info = info_iter;
            }
        }

        return hit;
    }

    ~scene() {}
};


#endif //_CLASS_SCENE_