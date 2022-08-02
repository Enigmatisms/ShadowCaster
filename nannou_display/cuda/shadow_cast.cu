#include <thrust/sort.h>
#include <thrust/binary_search.h>
#include <thrust/functional.h>
#include <thrust/execution_policy.h>
#include "shadow_cast.hpp"

#define PREPROCESS_BLOCK 4

float* point_angles = nullptr, *sorted_angles = nullptr;
bool* next_valid = nullptr;
Vec2* all_points = nullptr;
char* next_ids = nullptr;
int all_point_num = 0;              // set in memAllocator

__host__ void memAllocator(size_t point_num) {

}

__host__ void backCullPreprocess(const Vec3& pose, float* host_output) {
    const int thread_per_block = static_cast<int>(std::ceil(static_cast<float>(all_point_num) / PREPROCESS_BLOCK));
    backCullPreprocessKernel<<< PREPROCESS_BLOCK, thread_per_block >>> (all_points, next_ids, pose, all_point_num, point_angles, next_valid);
    // sorting rays
    CUDA_CHECK_RETURN(cudaMemcpy(sorted_angles, point_angles, all_point_num * sizeof(float), cudaMemcpyDeviceToDevice));
    thrust::sort(thrust::device, sorted_angles, sorted_angles + all_point_num, thrust::less<float>());
    const int invalid_bound = thrust::lower_bound(thrust::device, sorted_angles, sorted_angles + all_point_num, 1e2, thrust::less<float>()) - sorted_angles;
    /// duplicate valid rays
    float* actual_rays = nullptr;
    Vec2* out_pts = nullptr;
    const int actual_ray_num = invalid_bound << 1;
    CUDA_CHECK_RETURN(cudaMalloc((void **) &actual_rays, sizeof(float) * actual_ray_num));
    CUDA_CHECK_RETURN(cudaMalloc((void **) &out_pts, sizeof(Vec2) * actual_ray_num));
    simpleDuplicateKernel<<< 1, invalid_bound >>> (sorted_angles, actual_rays);
    /// get ray - mesh segment intersections. Notice that point_num (all_point_num) equals number of segment
    size_t ray_each_block = static_cast<size_t>(std::ceil(static_cast<float>(actual_ray_num) / 8));
    size_t seg_each_block = static_cast<size_t>(std::ceil(static_cast<float>(all_point_num) / 4));
    const size_t shared_memory_size = sizeof(float) * ray_each_block;
    cudaStream_t streams[4];
    for (short i = 0; i < 4; i++)
        cudaStreamCreateWithFlags(&streams[i],cudaStreamNonBlocking);
    for (int i = 0; i < 2; i++) {				// 面片
        for (int j = 0; j < 4; j++) {			// 光线
            // pose 在本处是const Vec3&, 在进入kernel时会发生复制，可以吗？
            pointIntersectKernel<<<dim3(2, 2), dim3(ray_each_block, seg_each_block), shared_memory_size, streams[j]>>>(
                all_points, next_ids, actual_rays, point_angles, next_valid, out_pts, pose, all_point_num, actual_ray_num, j, i
            );
        }
    }
    /// output, cleaning up
    CUDA_CHECK_RETURN(cudaDeviceSynchronize());
    CUDA_CHECK_RETURN(cudaMemcpy(host_output, out_pts, sizeof(Vec2) * actual_ray_num, cudaMemcpyDeviceToHost));
    for (int i = 0; i < 4; i++)
        cudaStreamDestroy(streams[i]);
    CUDA_CHECK_RETURN(cudaFree(out_pts));
    CUDA_CHECK_RETURN(cudaFree(actual_rays));
}
