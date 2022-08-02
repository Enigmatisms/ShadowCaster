#pragma once
#include "cast_kernel.hpp"

/** 开始时分配内存： , 
 * global_memory (float* for angles, sizeof(float) * point_num)
 * global_memory (bool* for [mesh being valid], sizeof(bool) * point_num)
 * global_memory (char* for [next id], sizeof(char) * point_num)
 * constant memory for points
*/
__host__ void memAllocator(size_t point_num);

// TODO: 将所有点以及点的id转移到常量内存中
__host__ void initializeCopy();

// 所有点都已经在初始化时移动到常量内存中
__host__ void backCullPreprocess(const Vec3& pose);