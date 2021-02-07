#ifndef _CLASS_PINHOLE_CAMERA_
#define _CLASS_PINHOLE_CAMERA_

#include "./vec3.h"
#include "./ray.h"

class pinhole_camera
{
private:
    vec3f position;
    vec3f forward_direction;
    vec3f right_direction;
    vec3f up_direction;
    static constexpr float distance_to_pinhole = 1;
public:
    pinhole_camera() {}

    pinhole_camera(vec3f& _position, vec3f& _forward_direction) {
        position = _position;
        forward_direction = _forward_direction.normalized();
        right_direction = forward_direction.cross(vec3f(0, 1, 0)).normalized();
        up_direction = right_direction.cross(forward_direction).normalized();
    }
    

    ray pinhole_ray(float u, float v) {
        vec3f ray_origin = position + right_direction * u + up_direction * v;
        vec3f ray_direction = (position + forward_direction * distance_to_pinhole) - ray_origin;

        return ray(ray_origin, ray_direction.normalized());
    }

    ~pinhole_camera() {}
};

#endif // !_CLASS_PINHOLE_CAMERA_