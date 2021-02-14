#ifndef _CLASS_VEC3_
#define _CLASS_VEC3_

#include<iostream>

template <typename T>
class vec3
{
protected:
    T x;
    T y;
    T z;

public:
    vec3() { x = y = z = 0; }

    vec3(T _x, T _y, T _z)
    {
        x = _x;
        y = _y;
        z = _z;
    }

    T get_x() const {
        return x;
    }

    T get_y() const {
        return y;
    }

    T get_z() const {
        return z;
    }

    vec3 operator * (const T target) const {
        return vec3(x * target, y * target, z * target);
    }

    vec3 operator * (vec3 target) const {
        return vec3(x * target.x, y * target.y, z * target.z);
    }

    vec3 operator + (vec3 target) const {
        return vec3(x + target.x, y + target.y, z + target.z);
    }

    vec3 operator - (vec3 target) const {
        return vec3(x - target.x, y - target.y, z - target.z);
    }

    T dot(const vec3 target) const {
        return ((x * target.x) +
                (y * target.y) +
                (z * target.z));
    }

    T me_dot() const {
        return ((x * x) +
                (y * y) +
                (z * z));
    }

    vec3 cross(vec3& target) const {
        return vec3(y * target.z - z * target.y,
                    z * target.x - x * target.z,
                    x * target.y - y * target.x
                    );
    }

    double magnitude() const {
        return std::sqrt(me_dot());
    }

    T sqr_magnitude() const {
        return me_dot();
    }

    vec3 normalized() {
        return *this * (T)(1 / magnitude());
    }

    bool eqValue(vec3& target) const {
        return (x == target.x) &&
               (y == target.y) && 
               (z == target.z);
    }

    void display() const {
        std::cout << "x:" << x << std::endl;
        std::cout << "y:" << y << std::endl;
        std::cout << "z:" << z << std::endl;
    }

    vec3 reflect(vec3& normal) const {
        return this->operator*(-1.0f) + normal * 2 * (this->dot(normal));
    }

    bool refract(vec3& normal, float ior1, float ior2, vec3& t) const {
        vec3 th = (this->operator-(normal * this->dot(normal))) * -(ior1 / ior2);
        const auto th_magnitude = th.sqr_magnitude();
        if (th_magnitude > 1.0) return false;

        t = normal * (-std::sqrt(1 - th_magnitude)) + th;

        return true;
    }

    ~vec3() {

    }
};

using vec3f = vec3<float>;

#endif