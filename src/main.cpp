#include <iostream>
#include "./image.h"
#include "./vec3.h"
#include "./pinhole_camera.h"
#include "./sphere.h"
#include "./intersect_info.h"
#include "./scene.h"

void assert_eq(bool eq) {
  if(eq) {
    std::cout << "ok" << std::endl;
  } else {
    std::cout << "bad" << std::endl;
  }
}

bool vec3_and_scalar_multi_test() {
  auto result = vec3f(1, 1, 1) * 2;

  return result.eqValue(vec3f(2, 2, 2));
}

bool vec3_hadamard_test() {
  auto result = vec3f(1, 2, 3) * vec3f(1, 2, 3);

  return result.eqValue(vec3f(1, 4, 9));
}

bool vec3_add_test() {
  auto result = vec3f(1, 1, 1) + vec3f(1, 1, 1);

  return result.eqValue(vec3f(2, 2, 2));
}

bool vec3_sub_test() {
  auto result = vec3f(1, 1, 1) - vec3f(1, 1, 1);
  
  return result.eqValue(vec3f(0, 0, 0));
}

void write_image_test() {
  image img(512, 512);
  auto canvas_size = img.get_size();

  auto width = std::get<0>(canvas_size);
  auto height = std::get<1>(canvas_size);

  for(int j = 0; j < height; ++j) {
    for(int i = 0; i < width; ++i) {
      img.set_pixel(i, j, vec3f(static_cast<float>(i) / width, static_cast<float>(j) / height, 1.0f));
    }
  }
  
  img.write_ppm("output.ppm");
}

void pinhole_camera_test() {
  image img(512, 512);
  auto canvas_size = img.get_size();

  auto width = std::get<0>(canvas_size);
  auto height = std::get<1>(canvas_size);
  
  pinhole_camera camera(vec3f(0.0f, 0.0f, 0.0f), vec3f(0.0f, 0.0f, -1.0f));

  for(int j = 0; j < height; j++) {
    for(int i = 0; i < width; i++) {
      const float u = (2.0f * i - width) / height;
      const float v = (2.0f * j - height) / height;
      const auto ray = camera.pinhole_ray(u, v);
      img.set_pixel(i, j, (ray.get_direction() + vec3f(1.0f, 1.0f, 1.0f)) * 0.5f);
    }
  }
  img.write_ppm("output.ppm");
}

void sphere_test() {
  image img(512, 512);
  auto canvas_size = img.get_size();

  auto width = std::get<0>(canvas_size);
  auto height = std::get<1>(canvas_size);

  pinhole_camera camera(vec3f(0, 0, 3), vec3f(0, 0, -1));

  sphere _sphere(vec3f(0.0f, 0.0f, 0.0f), 1.0f, sphere_material::diffuse, vec3f(1.0f, 1.0f, 1.0f));

  for(int j = 0; j < height; j++) {
    for(int i = 0; i < width; i++) {

      auto u = (2.0f * i - width) / height;
      auto v = (2.0f * j - height) / height;

      const auto ray = camera.pinhole_ray(u, v);

      intersect_info info;
      if(_sphere.collision_detect(ray, info)) {
        img.set_pixel(i, j, info.normal * 0.5f + vec3f(0.5f, 0.5f, 0.5f));
      } else {
        img.set_pixel(i, j, vec3f(0, 0, 0));
      }
    }
  }

  img.write_ppm("output.ppm");
}

void spheres_test() {
  image img(512, 512);
  auto canvas_size = img.get_size();

  auto width = std::get<0>(canvas_size);
  auto height = std::get<1>(canvas_size);

  pinhole_camera camera(vec3f(0, 0, 5), vec3f(0, 0, -1));

  scene scene;
  scene.add_sphere(sphere(vec3f(-1, 0, 1), 1.0f, sphere_material::diffuse, vec3f(1, 1, 1)));
  scene.add_sphere(sphere(vec3f(0, 0, 0), 1.0f, sphere_material::diffuse, vec3f(1, 1, 1)));
  scene.add_sphere(sphere(vec3f(1, 0, -1), 1.0f, sphere_material::diffuse, vec3f(1, 1, 1)));

  for(int j = 0; j < height; j++) {
    for(int i = 0; i < width; i++) {
      float u = (2.0f * i - width) / height;
      float v = (2.0f * j - height) / height;

      const auto ray = camera.pinhole_ray(u, v);

      intersect_info info;
      if(scene.collisions_detect(ray, info)) {
        //色を法線ではなく光源を追加する
        //球ではなく三角形の交差判定の実装
        img.set_pixel(i, j, info.normal * 0.5f + vec3f(0.5f, 0.5f, 0.5f));
      } else {
        img.set_pixel(i, j, vec3f(0, 0, 0));
      }
    }
  }

  img.write_ppm("output.ppm");
}

void tests() {
  assert_eq(vec3_and_scalar_multi_test());
  assert_eq(vec3_hadamard_test());
  assert_eq(vec3_add_test());
  assert_eq(vec3_sub_test());

  write_image_test();
}

int main() {
  
  spheres_test();
  return 0;
}