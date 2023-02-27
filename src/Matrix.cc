#include "Matrix.h"

Matrix::Matrix(): Matrix(Tuple(), Tuple(), Tuple(), Tuple()) {

}

Matrix::Matrix(Tuple c1, Tuple c2, Tuple c3, Tuple c4) {
    size = 4;
    setCol(0, c1);
    setCol(1, c2);
    setCol(2, c3);
    setCol(3, c4);
}

bool Matrix::equals(const Matrix &other) const {
    for (int r=0;r<4;r++){
        for (int c=0;c<4;c++){
            if (!almostEqual(get(r, c), other.get(r, c))) return false;
        }
    }
    return true;
}

Matrix Matrix::multiply(const Matrix &other) const {
    Matrix result;
    for (int r=0;r<4;r++) {
        for (int c = 0; c < 4; c++) {  // Cringe that i'm redoing dot product but its also cringe to remake tuple objects every time
            float v = get(r, 0) * other.get(0, c) + get(r, 1) * other.get(1, c) + get(r, 2) * other.get(2, c) + get(r, 3) * other.get(3, c);
            result.set(r, c, v);
        }
    }
    return result;
}

Tuple Matrix::multiply(const Tuple &other) const {
    Tuple result;
    for (int r=0;r<4;r++) {
        float v = 0;
        for (int c = 0; c < 4; c++) {
            v += get(r, c) * other.get(c);
        }
        result.set(r, v);

    }
    return result;
}

Matrix Matrix::transpose() const {
    Matrix result;
    for (int r=0;r<4;r++) {
        for (int c = 0; c < 4; c++) {
            result.set(r, c, get(c, r));
        }

    }
    return result;
}

Matrix Matrix::submatrix(int row, int col) const {
    Matrix result;
    int rr = 0;
    for (int r=0;r<size;r++){
        if (r == row) continue;
        int cc = 0;
        for (int c = 0; c < size; c++) {
            if (c == col) continue;
            result.set(rr, cc, get(r, c));

            cc++;
        }
        rr++;
    }
    result.size = size - 1;
    return result;
}

float Matrix::determinant() const {
    if (size == 2){
        return (get(0, 0) * get(1, 1)) - (get(1, 0) * get(0, 1));
    }

    float det = 0;
    for (int r=0;r<size;r++){
        det += get(r, 0) * cofactor(r, 0);
    }
    return det;
}

void Matrix::print() const {
    cout << "----" << endl;
    for (int r=0;r<size;r++){
        for (int c=0;c<size;c++){
            cout << get(r, c) << ", ";
        }
        cout << endl;
    }
    cout << "----" << endl;
}

Matrix Matrix::inverse() const {
    float det = determinant();

#ifdef DEBUG_CHECKS
    if (det == 0) error() << "One does not simply invert the un-invertible" << endl;
#endif

    Matrix result;
    result.size = size;
    for (int r=0;r<size;r++){
        for (int c=0;c<size;c++) {
            result.set(c, r, cofactor(r, c) / det);
        }
    }
    return result;
}

Matrix::Matrix(const Matrix &other) {
    for (int r=0;r<other.size;r++){
        for (int c=0;c<other.size;c++) {
            set(r, c, other.get(r, c));
        }
    }
    size = other.size;
}

Matrix Transformation::translation(float x, float y, float z) {
    Matrix result = identity();
    result.set(0, 3, x);
    result.set(1, 3, y);
    result.set(2, 3, z);
    return result;
}

Matrix Transformation::identity() {
    return Matrix(Tuple(1, 0, 0, 0), Tuple(0, 1, 0, 0), Tuple(0, 0, 1, 0), Tuple(0, 0, 0, 1));
}

// TODO: make the constructor take rows so I can write these out so they actually look like the matrix

Matrix Transformation::scaling(float x, float y, float z) {
    Matrix result = identity();
    result.set(0, 0, x);
    result.set(1, 1, y);
    result.set(2, 2, z);
    return result;
}

Matrix Transformation::rotation_x(float rad) {
    Matrix result = identity();
    result.set(1, 1, cos(rad));
    result.set(1, 2, -sin(rad));
    result.set(2, 1, sin(rad));
    result.set(2, 2, cos(rad));
    return result;
}

Matrix Transformation::rotation_y(float rad) {
    Matrix result = identity();
    result.set(0, 0, cos(rad));
    result.set(0, 2, sin(rad));
    result.set(2, 0, -sin(rad));
    result.set(2, 2, cos(rad));
    return result;
}
Matrix Transformation::rotation_z(float rad) {
    Matrix result = identity();
    result.set(0, 0, cos(rad));
    result.set(0, 1, -sin(rad));
    result.set(1, 0, sin(rad));
    result.set(1, 1, cos(rad));
    return result;
}

Matrix Transformation::shearing(float xy, float xz, float yx, float yz, float zx, float zy) {
    Matrix result = identity();
    result.set(0, 1, xy);
    result.set(0, 2, xz);
    result.set(1, 0, yx);
    result.set(1, 2, yz);
    result.set(2, 0, zx);
    result.set(2, 1, zy);
    return result;
}
