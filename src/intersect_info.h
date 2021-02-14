#ifndef _CLASS_INTERSECT_INFO_
#define _CLASS_INTERSECT_INFO_

#include "./vec3.h"

class sphere;
struct intersect_info
{
    /* data */
    float distance;
    vec3f point;
    vec3f normal;
    const sphere* sph;
};

#endif // _CLASS_INTERSECT_INFO_