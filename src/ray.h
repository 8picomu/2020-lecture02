#ifndef _CLASS_RAY_
#define _CLASS_RAY_

#include "./vec3.h"

//視野角からピンホールまでの距離を割り出す実装
class ray
{
private:
    vec3f origin;
    vec3f direction;
    static constexpr float intersection_distance_threshold_min = 1e-3f;
    static constexpr float intersection_distance_threshold_max = 10000;

public:
    ray() {}

    ray(vec3f& _origin, vec3f& _direction) {
        origin = _origin;
        direction = _direction.normalized();
     }

    vec3f return_t_point(float t) const { return origin + direction * t; }

    vec3f get_origin() const {
        return origin;
    }

    vec3f get_direction() const {
        return direction;
    }

    float get_min() const {
        return intersection_distance_threshold_min;
    }

    float get_max() const {
        return intersection_distance_threshold_max;
    }

    ~ray() {}
};

#endif // !_CLASS_RAY_