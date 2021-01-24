#ifndef _CLASS_VEC3_
#define _CLASS_VEC3_

template <typename T>
class vec3
{
protected:
    T x;
    T y;
    T z;

public:
    vec3(T _x, T _y, T _z)
    {
        x = _x;
        y = _y;
        z = _z;
    }

    T get_x() {
        return x;
    }

    T get_y() {
        return y;
    }

    T get_z() {
        return z;
    }

    vec3 operator * (T target) {
        return vec3(x * target, y * target, z * target);
    }

    vec3 operator * (vec3 target) {
        return vec3(x * target.x, y * target.y, z * target.z);
    }

    vec3 operator + (vec3 target) {
        return vec3(x + target.x, y + target.y, z + target.z);
    }

    vec3 operator - (vec3 target) {
        return vec3(x - target.x, y - target.y, z - target.z);
    }

    T dot(vec3 target) {
        return ((x * target.x) +
                (y * target.y) +
                (z * target.z));
    }

    vec3 cross(vec3 target) {
        return vec3(y * target.z - z * target.y,
                    z * target.x - x * target.z,
                    x * target.y - y * target.x
                    );
    }

    double magnitude() {
        return std::sqrt(dot(this));
    }

    T sqr_magnitude() {
        return dot(this);
    }

    vec3 normalized() {
        return (1 / sqrMagnitude) * this;
    }

    bool eqValue(vec3 target) {
        return (x == target.x) &&
               (y == target.y) && 
               (z == target.z);
    }

    ~vec3() {

    }
};

using vec3f = vec3<float>;

#endif