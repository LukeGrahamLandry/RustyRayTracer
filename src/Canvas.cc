#include <fstream>
#include "Canvas.h"

Canvas::Canvas(int width, int height) {
    this->width = width;
    this->height = height;

    pixels = new Colour*[width];
    for (int i=0;i<width;i++){
        pixels[i] = new Colour[height];
    }
}

Canvas::~Canvas() {
    for (int i=0;i<height;i++){
        delete [] pixels[i];
    }
    delete [] pixels;
}

void Canvas::write_pixel(int x, int y, const Colour& pixel) {
#ifdef DEBUG_CHECKS
    if (x < 0 || x >= width || y < 0 || y >= height) {
        error() << "One does not simply draw out of bounds" << endl;
    }
#endif

    pixels[x][y] = pixel;
}

const Colour & Canvas::pixel_at(int x, int y) const {
#ifdef DEBUG_CHECKS
    if (x < 0 || x >= width || y < 0 || y >= height) {
        error() << "One does not simply render out of bounds" << endl;
    }
#endif

    return pixels[x][y];
}

inline int clamp_rgb(double x){
    if (x < 0) return 0;
    if (x > 1) return 255;
    return (int) (x * 255);
}

string Canvas::to_ppm() const {
    string s = string("P3\n");
    s.append(to_string(width) + " " + to_string(height) + "\n255\n");
    for (int y=0;y<height;y++){
        for (int x=0;x<width;x++){
            Colour c = pixel_at(x, y);
            s.append(to_string(clamp_rgb(c.red)) + " " + to_string(clamp_rgb(c.green)) + " " + to_string(clamp_rgb(c.blue)) + " ");
            // TODO: limit to 70 characters per line
        }
        s.append("\n");
    }
    return s;
}

void Canvas::write_ppm(const char* path) const {
    string s = to_ppm();

    ofstream myfile;
    myfile.open (path);
    myfile << s;
    myfile.close();
}