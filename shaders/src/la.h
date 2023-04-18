/* Linear algebra types used by cpu_runner.
 * gpu_runner uses Metal's versions instead.
 * So any inefficiency here will only slow down debugging on the CPU.
 * However, that means using the CPU profiler won't accurately reflect the behaviour on the GPU.
 * Tried to use apple's <simd/simd.h> but it has different repr so passing data from rust didn't work.
 */

#ifndef LA_H
#define LA_H

#ifdef NOT_BUILDING_AS_MSL

#ifndef DOING_RUST_BINDGEN
#include <cmath>
#else
// idk why bindgen cant find the std headers but also i dont care enough.
typedef int uint32_t;
float sqrt(float v);
#endif

struct alignas(16) float4 {
    float x, y, z, w;

    float4() : float4(0, 0, 0, 0) {}
    float4(float x, float y, float z, float w) : x{x}, y{y}, z{z}, w{w} {}

    float4 operator+(float4 other) const;
    float4& operator+=(float4 other);
    float4 operator-(float4 other) const;
    float4 operator*(float4 other) const;
    float4 operator*(float other) const;
    float4 operator/(float other) const;
    float4 operator-() const;

    float& get(int i);
};

struct alignas(16) float3 {
    float x, y, z;

    float3() : float3(0, 0, 0) {}
    float3(float x, float y, float z) : x{x}, y{y}, z{z} {}

    float3 operator+(float3 other) const;
    float3 operator-(float3 other) const;
    float3 operator*(float3 other) const;
    float3 operator*(float other) const;
    float3 operator/(float other) const;
    float3 operator-() const;
    float3& operator+=(float3 other);
};

struct alignas(16) float4x4 {
    float data[4][4];

    float4x4();
    // names swapped cause glam is row major order
    float get(int c, int r) const;
    void set(int c, int r, float v);
    float4x4 operator*(float4x4 other) const;
    float4 operator*(float4 other) const;
};

// Not actually the dot product since I only use them as 3d vectors
float dot(float4 a, float4 b);

float length_squared(float4 v);

float length(float4 v);

float4 normalize(float4 v);

float4 reflect(float4 v, float4 n);

float4x4 transpose(float4x4 m);

#endif

float3 black();
float4 zero_vec();
float4 point(float x, float y, float z);

#endif