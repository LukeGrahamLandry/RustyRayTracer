#include "la.h"

#ifdef NOT_BUILDING_AS_MSL

float4 float4::operator+(float4 other) const {
    return {x + other.x, y + other.y, z + other.z, w + other.w};
}
float4& float4::operator+=(float4 other) {
    x += other.x;
    y += other.y;
    z += other.z;
    w += other.w;
    return *this;
}
float4 float4::operator-(float4 other) const {
    return {x - other.x, y - other.y, z - other.z, w - other.w};
}
float4 float4::operator*(float4 other) const {
    return {x * other.x, y * other.y, z * other.z, w * other.w};
}
float4 float4::operator*(float other) const {
    return {x * other, y * other, z * other, w * other};
}
float4 float4::operator/(float other) const {
    return {x / other, y / other, z / other, w / other};
}
float4 float4::operator-() const {
    return *this*-1;
}

float& float4::get(int i) {
    switch (i) {
        case 0: return x;
        case 1: return y;
        case 2: return z;
        case 3: return w;
        default:
            __builtin_unreachable();
    }
}


float3 float3::operator+(float3 other) const {
    return {x + other.x, y + other.y, z + other.z};
}
float3 float3::operator-(float3 other) const {
    return {x - other.x, y - other.y, z - other.z};
}
float3 float3::operator*(float3 other) const {
    return {x * other.x, y * other.y, z * other.z};
}
float3 float3::operator*(float other) const {
    return {x * other, y * other, z * other};
}
float3 float3::operator/(float other) const {
    return {x / other, y / other, z / other};
}
float3 float3::operator-() const {
    return *this*-1;
}
float3& float3::operator+=(float3 other) {
    x += other.x;
    y += other.y;
    z += other.z;
    return *this;
}


float4x4::float4x4() {
    for (int r=0;r<4;r++) {
        for (int c = 0; c < 4; c++) {
            set(r, c, 0);
        }
    }
}
// names swapped cause glam is row major order
float float4x4::get(int c, int r) const {
    return data[r][c];
}
void float4x4::set(int c, int r, float v){
    data[r][c] = v;
}
float4x4 float4x4::operator*(float4x4 other) const {
    float4x4 result;
    for (int r=0;r<4;r++) {
        for (int c = 0; c < 4; c++) {
            float v = get(r, 0) * other.get(0, c) + get(r, 1) * other.get(1, c) + get(r, 2) * other.get(2, c) + get(r, 3) * other.get(3, c);
            result.set(r, c, v);
        }
    }
    return result;
}
float4 float4x4::operator*(float4 other) const {
    float4 result = {0, 0, 0, 0};
    for (int r=0;r<4;r++) {
        float v = 0;
        for (int c = 0; c < 4; c++) {
            v += get(r, c) * other.get(c);
        }
        result.get(r) = v;
    }
    return result;
}

// Not actually the dot product since I only use them as 3d vectors
float dot(float4 a, float4 b) {
    return (float) (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
}

float length_squared(float4 v){
    return dot(v, v);
}

float length(float4 v){
    return sqrt(length_squared(v));
}

float4 normalize(float4 v) {
    return v / length(v);
}

float4 reflect(float4 v, float4 n){
    return v - (n * 2.0 * dot(v, n));
}

float4x4 transpose(float4x4 m) {
    float4x4 result;
    for (int r=0;r<4;r++) {
        for (int c = 0; c < 4; c++) {
            result.set(r, c, m.get(c, r));
        }

    }
    return result;
}

#endif

float3 black(){
    return {0.0, 0.0, 0.0};
}

float4 zero_vec(){
    return {0.0, 0.0, 0.0, 0.0};
}

float4 point(float x, float y, float z){
    return {x, y, z, 1.0};
}


