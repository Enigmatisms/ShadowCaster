#pragma once
#include <cmath>
#include <cuda_runtime.h>
#include <device_functions.h>
#include <device_launch_parameters.h>
#include "cuda_err_check.hpp"

struct Vec2 {
    float x;
    float y;
};

struct Vec3 {
    float x;
    float y;
    float z;
};

struct Point {
    float x = 0.f;
    float y = 0.f;
    __device__ Point() noexcept : x(0), y(0) {}
    __device__ Point(float x, float y) noexcept : x(x), y(y) {}
    __device__ Point(const Vec2& vec) noexcept : x(vec.x), y(vec.y) {}
    __device__ Point(const Vec3& vec) noexcept : x(vec.x), y(vec.y) {}

    __forceinline__ __device__ Point operator-(const Point& p) const noexcept {
        return Point(x - p.x, y - p.y);         // Return value optimized?
    }

    __forceinline__ __device__ Point operator+(const Point& p) const noexcept {
        return Point(x + p.x, y + p.y);         // Return value optimized?
    }

    __forceinline__ __device__ Point operator*(float val) const noexcept { 
        return Point(x * val, y * val);         // Return value optimized?
    }

    __forceinline__ __device__  float get_angle() const {
        return atan2f(y, x);
    }

    __forceinline__ __device__ float dot(const Point& p) const noexcept {
        return x * p.x + y * p.y;
    }

    __forceinline__ __device__ float norm() const noexcept {
        return sqrtf(x * x + y * y);
    }
};

__global__ void backCullPreprocessKernel(
    const Vec2* const points, const char* const next_ids, const Vec3 pose, 
    int all_point_num, float* angles, bool* mesh_valid
);              /// logically correct [checked]

__global__ void pointIntersectKernel(
    const Vec2* const points, const char* const next_ids, const float* const rays, const float* const angles,
    const bool* const mesh_valid, float* output, const Vec3 pose, int all_seg_num, int all_ray_num, int ray_boffset, int seg_boffset
);

__global__ void depth2PointKernel(const float* const all_outputs, const float* const ray_angle, const Vec3 pose, Vec2* const out_pts);

__global__ void simpleDuplicateKernel(const float* const inputs, float* const outputs);
