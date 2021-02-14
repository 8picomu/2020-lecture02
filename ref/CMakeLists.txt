add_executable(test-image test-image.cpp)
target_compile_features(test-image PRIVATE cxx_std_17)
target_include_directories(test-image PRIVATE src)

add_executable(test-camera test-camera.cpp)
target_compile_features(test-camera PRIVATE cxx_std_17)
target_include_directories(test-camera PRIVATE src)

add_executable(test-sphere test-sphere.cpp)
target_compile_features(test-sphere PRIVATE cxx_std_17)
target_include_directories(test-sphere PRIVATE src)

add_executable(test-intersect test-intersect.cpp)
target_compile_features(test-intersect PRIVATE cxx_std_17)
target_include_directories(test-intersect PRIVATE src)

add_executable(raytracer raytracer.cpp)
target_compile_features(raytracer PRIVATE cxx_std_17)
target_include_directories(raytracer PRIVATE src)

add_executable(test-rng test-rng.cpp)
target_compile_features(test-rng PRIVATE cxx_std_17)
target_include_directories(test-rng PRIVATE src)

add_executable(raytracer-ssaa raytracer-ssaa.cpp)
target_compile_features(raytracer-ssaa PRIVATE cxx_std_17)
target_include_directories(raytracer-ssaa PRIVATE src)

add_executable(openmp-test openmp-test.cpp)
target_compile_features(openmp-test PRIVATE cxx_std_17)
target_include_directories(openmp-test PRIVATE src)
target_link_libraries(openmp-test PRIVATE OpenMP::OpenMP_CXX)

add_executable(raytracer-openmp raytracer-openmp.cpp)
target_compile_features(raytracer-openmp PRIVATE cxx_std_17)
target_include_directories(raytracer-openmp PRIVATE src)
target_link_libraries(raytracer-openmp PRIVATE OpenMP::OpenMP_CXX)

add_executable(test-ao test-ao.cpp)
target_compile_features(test-ao PRIVATE cxx_std_17)
target_include_directories(test-ao PRIVATE src)
target_link_libraries(test-ao PRIVATE OpenMP::OpenMP_CXX)

add_executable(raytracer-ao raytracer-ao.cpp)
target_compile_features(raytracer-ao PRIVATE cxx_std_17)
target_include_directories(raytracer-ao PRIVATE src)
target_link_libraries(raytracer-ao PRIVATE OpenMP::OpenMP_CXX)
