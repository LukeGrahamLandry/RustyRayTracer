#include "demo.h"

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

void chapter5() {
    int size = 100;
    float wall_distance = 200;
    Canvas screen = Canvas(size, size);
    Colour black = Colour(0, 0, 0);
    Colour red = Colour(1, 0, 0);

    Matrix move_to_center = Transformation::translation((float) (size / 2.0), (float) (size / 2.0), 0);
    Tuple ray_start = move_to_center.multiply(Point(0, 0, 0));

    Sphere sphere;
    Matrix move_away = Transformation::translation(0, 0, 20);
    float scale = 2;
    Matrix bigger = Transformation::scaling(scale, scale, scale);
    Matrix useless_rotation = Transformation::rotation_z(0);
    sphere.set_transform(Transformation::identity().multiply(move_away).multiply(move_to_center).multiply(bigger).multiply(useless_rotation).multiply(Transformation::scaling(1, 0.5, 1)));

    for (int x = 0;x<size;x++){
        for (int y = 0;y<size;y++){
            Tuple ray_end = Point((float) x, (float) y, wall_distance);
            Tuple ray_direction = ray_end.subtract(ray_start);
            Ray ray = Ray(ray_start, ray_direction);

            bool hit = sphere.intersect(ray).hasHit();
            screen.write_pixel(x, y, hit ? red : black);
        }
    }

    screen.write_ppm("chapter5.ppm");
}
