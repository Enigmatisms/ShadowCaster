#pragma once
#include "cast_kernel.hpp"

__host__ void deallocatePoints();

/** Allocate on intialization, 
 * global_memory (float* for angles, sizeof(float) * point_num)
 * global_memory (bool* for [mesh being valid], sizeof(bool) * point_num)
 * global_memory (char* for [next id], sizeof(char) * point_num)
 * constant memory for points
*/
__host__ void updatePointInfo(const Vec2* const meshes, const char* const nexts, int point_num, bool initialized);

// 所有点都已经在初始化时移动到常量内存中
__host__ void shadowCasting(const Vec3& pose, Vec2* const host_output, int& point_num);
