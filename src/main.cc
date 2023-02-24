#include "Canvas.h"
#include "Colour.h"
#include "Tuple.h"
#include "Matrix.h"
#include "common.h"


void chapter4(){
    float degToRad = M_PI / 180;
    float radius = 10;
    float size = 30;
    int count = 12;
    Canvas screen = Canvas((int) size, (int) size);
    Colour white = Colour(1, 1, 1);

    // How much more to rotate for each subsequent point.
    Matrix base = Transformation::rotation_z(degToRad * (float) (360.0 / count));

    // Moves from world coordinates to canvas coordinates.
    // Scale up the point, which just moves away from the origin.
    // Then translate down and right to move the origin from the top left corner to the center of the canvas.
    Matrix shift = Transformation::translation(size / 2, size / 2, 0).multiply(Transformation::scaling(radius, radius, 0));

    // Track how much to rotate the current point.
    Matrix r = Transformation::identity();

    // The first point to draw.
    Tuple p = Point(0, 1, 0);

    for (int i=0;i<count;i++){
        Tuple renderPos = shift.multiply(r.multiply(p));
        screen.write_pixel((int) renderPos.x(), (int) renderPos.y(), white);

        // Tick the rotation over by 30 degrees.
        r = r.multiply(base);
    }

    screen.write_ppm("chapter4.ppm");
}

int main(){
    chapter4();
}
