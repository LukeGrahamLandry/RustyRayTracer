#ifndef RAYTRACER_MATRIX_H
#define RAYTRACER_MATRIX_H

#include "Tuple.h"

class Matrix {
private:
    int size;
    Tuple columns[4];
public:
    Matrix();
    Matrix(const Matrix& other);
    Matrix(Tuple c1, Tuple c2, Tuple c3, Tuple c4);

    Matrix add(const Matrix& other) const;
    Matrix multiply(const Matrix& other) const;
    Tuple multiply(const Tuple& other) const;
    Matrix inverse() const;
    Matrix transpose() const;
    float determinant() const;
    bool equals(const Matrix& other) const;
    Matrix submatrix(int row, int col) const;
    void print() const;

    inline float cofactor(int row, int col) const{
        return submatrix(row, col).determinant() * ((float) ((row + col) % 2 == 0 ? 1 : -1));
    }
    inline bool invertible() const {
        return determinant() != 0;
    }

    inline const Tuple getCol(int col) const {
#ifdef DEBUG_CHECKS
        if (col < 0 || col >= size) error() << "One does not simply get columns outside the matrix." << endl;
#endif
        return columns[col];
    }

    float inline get(int row, int col) const {
#ifdef DEBUG_CHECKS
        if (row < 0 || row >= size) error() << "One does not simply get rows outside the matrix." << endl;
#endif
        return getCol(col).get(row);
    }

    void inline set(int row, int col, float value){
#ifdef DEBUG_CHECKS
        if (row < 0 || row >= size) error() << "One does not simply set rows outside the matrix." << endl;
#endif
        columns[col].set(row, value);
    }

    void inline setCol(int col, Tuple vector){
#ifdef DEBUG_CHECKS
        if (col < 0 || col >= size) error() << "One does not simply set columns outside the matrix." << endl;
#endif
        columns[col] = vector;
    }
};

class Transformation {
public:
    static Matrix identity();
    static Matrix translation(float x, float y, float z);
    static Matrix scaling(float x, float y, float z);
    static Matrix rotation_x(float rad);
    static Matrix rotation_y(float rad);
    static Matrix rotation_z(float rad);
    static Matrix shearing(float xy, float xz, float yx, float yz, float zx, float zy);
};

#endif //RAYTRACER_MATRIX_H
