#ifndef _CLASS_INTERSECT_INFO_
#define _CLASS_INTERSECT_INFO_

#include "./vec3.h"
#include "./sphere.h"

class sphere;

struct intersect_info
{
    /* data */
    float distance;
    vec3f point;
    vec3f normal;
    sphere* sph;
};

#endif // _CLASS_INTERSECT_INFO_