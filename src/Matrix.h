#ifndef RAYTRACER_MATRIX_H
#define RAYTRACER_MATRIX_H

#include "Tuple.h"

class Matrix {
private:
    Tuple columns[4];
    int size;
public:
    Matrix();
    Matrix(const Matrix& other);
    Matrix(Tuple c1, Tuple c2, Tuple c3, Tuple c4);

    Matrix add(const Matrix& other) const;
    Matrix multiply(const Matrix& other) const;
    Tuple multiply(const Tuple& other) const;
    Matrix inverse() const;
    Matrix transpose() const;
    double determinant() const;
    bool equals(const Matrix& other) const;
    Matrix submatrix(int row, int col) const;
    void print() const;

    int getSize() const {
        return size;
    }

    // Get the transpose of the inverse of this matrix. Just exists as a little helper here so MemoMatrix can intercept it.
    Matrix transpose_of_inverse() const {
        return inverse().transpose();
    }

    Matrix static fromRows(const Tuple& r1, const Tuple& r2, const Tuple& r3, const Tuple& r4);

    double cofactor(int row, int col) const{
        return submatrix(row, col).determinant() * ((double) ((row + col) % 2 == 0 ? 1 : -1));
    }
    bool invertible() const {
        return determinant() != 0;
    }

    const Tuple& getCol(int col) const {
#ifdef DEBUG_CHECKS
        if (col < 0 || col >= size) {
            error() << "One does not simply get columns outside the matrix." << endl;
        }
#endif
        return columns[col];
    }

    double get(int row, int col) const {
#ifdef DEBUG_CHECKS
        if (row < 0 || row >= size) {
            error() << "One does not simply get rows outside the matrix." << endl;
        }
#endif
        return getCol(col).get(row);
    }

    void set(int row, int col, double value){
#ifdef DEBUG_CHECKS
        if (row < 0 || row >= size) {
            error() << "One does not simply set rows outside the matrix." << endl;
        }
#endif
        columns[col].set(row, value);
    }

    void setCol(int col, const Tuple& vector){
#ifdef DEBUG_CHECKS
        if (col < 0 || col >= size) error() << "One does not simply set columns outside the matrix." << endl;
#endif
        columns[col] = vector;  // this does the copy so the version in the array isn't const anymore
    }
};

class Transformation {
public:
    static Matrix identity();
    static Matrix translation(double x, double y, double z);
    static Matrix scaling(double x, double y, double z);
    static Matrix rotation_x(double rad);
    static Matrix rotation_y(double rad);
    static Matrix rotation_z(double rad);
    static Matrix shearing(double xy, double xz, double yx, double yz, double zx, double zy);
    static Matrix view_transform(const Tuple& from, const Tuple& to, const Tuple& up);
};

// TODO: Actually understand virtual functions.
//       I think what i have now works when you have an object of MemoMatrix.
//       But if I had a Matrix* that happened to have come from a MemoMatrix I would be calling the base functions instead of the overriden ones.
//       Doing it this way is cool because you can see the performance change by just switching the type of a field between Matrix and MemoMatrix.

class MemoMatrix: public Matrix {
private:
    Matrix memo_inverse;
    Matrix memo_transpose_of_inverse;
    void memoize(){
        memo_inverse = Matrix::inverse();
        memo_transpose_of_inverse = memo_inverse.transpose();
    }
public:
    MemoMatrix(const Matrix &other) : Matrix(other) {
        memoize();
    }
    MemoMatrix() : Matrix() {
        // Default Matrix is all zeros which has no inverse.
        // memoize();
    }
    MemoMatrix(Tuple c1, Tuple c2, Tuple c3, Tuple c4) : Matrix(c1, c2, c3, c4) {
        memoize();
    }
    const Matrix& transpose_of_inverse() const {
        return memo_transpose_of_inverse;
    }
    const Matrix& inverse() const {
        return memo_inverse;
    }
    void set(int row, int col, double value){
        Matrix::set(row, col, value);
        memoize();
    }
    void setCol(int col, const Tuple& vector){
        Matrix::setCol(col, vector);
        memoize();
    }
};

#endif