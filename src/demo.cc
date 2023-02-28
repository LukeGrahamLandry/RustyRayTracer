#include "demo.h"

void chapter4(){
    double degToRad = M_PI / 180;
    double radius = 10;
    double size = 30;
    int count = 12;
    Canvas screen = Canvas((int) size, (int) size);
    Colour white = Colour(1, 1, 1);

    // How much more to rotate for each subsequent point.
    Matrix base = Transformation::rotation_z(degToRad * (double) (360.0 / count));

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
    double wall_distance = 200;
    Canvas screen = Canvas(size, size);
    Colour black = Colour(0, 0, 0);
    Colour red = Colour(1, 0, 0);

    Matrix move_to_center = Transformation::translation((double) (size / 2.0), (double) (size / 2.0), 0);
    Tuple ray_start = move_to_center.multiply(Point(0, 0, 0));

    Sphere sphere;
    Matrix move_away = Transformation::translation(0, 0, 20);
    double scale = 2;
    Matrix bigger = Transformation::scaling(scale, scale, scale);
    Matrix useless_rotation = Transformation::rotation_z(0);
    sphere.set_transform(Transformation::identity().multiply(move_away).multiply(move_to_center).multiply(bigger).multiply(useless_rotation).multiply(Transformation::scaling(1, 0.5, 1)));

    for (int x = 0;x<size;x++){
        for (int y = 0;y<size;y++){
            Tuple ray_end = Point((double) x, (double) y, wall_distance);
            Tuple ray_direction = ray_end.subtract(ray_start);
            Ray ray = Ray(ray_start, ray_direction);

            bool hit = sphere.intersect(ray).hasHit();
            screen.write_pixel(x, y, hit ? red : black);
        }
    }

    screen.write_ppm("chapter5.ppm");
}

void chapter6() {
    long start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();

    int resolution_scale = 1;
    int size = 100 * resolution_scale;
    double wall_distance = (double) (200.0 * resolution_scale);
    Canvas screen = Canvas(size, size);
    Colour black = Colour(0, 0, 0);

    Matrix move_to_center = Transformation::translation((double) (size / 2.0), (double) (size / 2.0), 0);
    Tuple ray_start = move_to_center.multiply(Point(0, 0, 0));

    Sphere sphere;
    sphere.material.color = Colour(0.2, 1, 1);
    PointLight light = PointLight(Point(-10, 10, -10), Colour(1, 1, 1));

    Matrix move_away = Transformation::translation(0, 0, 20);
    double scale = 2;
    Matrix bigger = Transformation::scaling(scale, scale, scale);
    Matrix useless_rotation = Transformation::rotation_z(0);
    sphere.set_transform(Transformation::identity().multiply(move_away).multiply(move_to_center).multiply(bigger).multiply(useless_rotation).multiply(Transformation::scaling(scale, scale, scale)));

    for (int x = 0;x<size;x++){
        for (int y = 0;y<size;y++){
            Tuple ray_end = Point((double) x, (double) y, wall_distance);
            Tuple ray_direction = ray_end.subtract(ray_start).normalize();
            Ray ray = Ray(ray_start, ray_direction);

            Intersections hit = sphere.intersect(ray);

            if (hit.hasHit()){
                Tuple point_on_sphere = ray.position(hit.hit().t);
                Tuple normal = sphere.normal_at(point_on_sphere);
                Colour colour = sphere.material.lighting(light, point_on_sphere, ray_direction.negate(), normal);

                screen.write_pixel(x, y, colour);
            } else {
                screen.write_pixel(x, y, black);
            }
        }

        cout << x << "/" << size << endl;
    }

    long end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
    cout << "Rendered " << (size * size) << " pixels in " << (end_time - start_time) << " ms." << endl;
    screen.write_ppm("chapter6.ppm");
}



void chapter7() {
    long start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();

    Sphere floor;
    floor.set_transform(Transformation::scaling(10, 0.01, 10));
    floor.material.color = Colour(1, 0.9, 0.9);
    floor.material.specular = 0;

    Sphere left_wall;
    left_wall.set_transform(Transformation::translation(0, 0, 5)
                    .multiply(Transformation::rotation_y(-M_PI/4))
                    .multiply(Transformation::rotation_x(M_PI/2))
                    .multiply(Transformation::scaling(10, 0.01, 10))
                    );
    left_wall.material = floor.material;

    Sphere right_wall;
    right_wall.set_transform(Transformation::translation(0, 0, 5)
                                    .multiply(Transformation::rotation_y(M_PI/4))
                                    .multiply(Transformation::rotation_x(M_PI/2))
                                    .multiply(Transformation::scaling(10, 0.01, 10))
    );
    right_wall.material = floor.material;

    Sphere middle;
    middle.set_transform(Transformation::translation(-0.5, 1, 0.5));
    middle.material.color = Colour(0.1, 1, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    Sphere right;
    right.set_transform(Transformation::translation(1.5, 0.5, -0.5).multiply(Transformation::scaling(0.5, 0.5, 0.5)));
    right.material.color = Colour(0.5, 1, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    Sphere left;
    left.set_transform(Transformation::translation(-1.5, 0.33, -0.75).multiply(Transformation::scaling(0.33, 0.33, 0.33)));
    left.material.color = Colour(1, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;


    Tuple light_pos = Point(-10, 10, -10);
    light_pos = Transformation::rotation_x(0).multiply(light_pos);

    World world;
    world.addLight(PointLight(light_pos, Colour(1, 1, 1)));

    world.addShape(floor);
    world.addShape(right_wall);
    world.addShape(left_wall);
    world.addShape(middle);
    world.addShape(right);
    world.addShape(left);

    int resolution_factor = 2;
    Camera camera = Camera(100 * resolution_factor, 50 * resolution_factor, M_PI/3);
    Matrix perspective = Transformation::view_transform(Point(0, 1.5, -5), Point(0, 1, 0), Vector(0, 1, 0));
    perspective = perspective.multiply(Transformation::translation(0, 0, 1));
    camera.set_transform(perspective);

    Canvas screen = camera.render(world);

    long end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
    cout << "Rendered " << (camera.hsize * camera.vsize) << " pixels in " << (end_time - start_time) << " ms." << endl;
    screen.write_ppm("chapter7.ppm");
}


void test() {
    Sphere middle;
    middle.set_transform(Transformation::translation(-0.5, 1, 0.5));
    middle.material.color = Colour(0.1, 1, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    Sphere right;
    right.set_transform(Transformation::translation(1.5, 0.5, -0.5).multiply(Transformation::scaling(0.5, 0.5, 0.5)));
    right.material.color = Colour(0.5, 1, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    Sphere left;
    left.set_transform(Transformation::translation(-1.5, 0.33, -0.75).multiply(Transformation::scaling(0.33, 0.33, 0.33)));
    left.material.color = Colour(1, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;


    Tuple light_pos = Point(-10, 10, -10);
    light_pos = Transformation::rotation_x(0).multiply(light_pos);

    World world;
    world.addLight(PointLight(light_pos, Colour(1, 1, 1)));

    world.addShape(middle);
    world.addShape(right);
    world.addShape(left);

    int resolution_factor = 2;
    Camera camera = Camera(100 * resolution_factor, 50 * resolution_factor, M_PI/3);
    Matrix perspective = Transformation::view_transform(Point(0, 1.5, -5), Point(0, 1, 0), Vector(0, 1, 0));

    for (int i=0;i<16;i++){
        long start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();

        perspective = perspective.multiply(Transformation::rotation_x(1 * M_PI / 16));
        camera.set_transform(perspective);

        Canvas screen = camera.render(world);

        long end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
        cout << "Rendered " << (camera.hsize * camera.vsize) << " pixels in " << (end_time - start_time) << " ms." << endl;
        screen.write_ppm((to_string(i) + "-test.ppm").c_str());
    }
}

void lights() {
    Sphere middle;
    middle.set_transform(Transformation::translation(-0.5, 1, 0.5));
    middle.material.color = Colour(1, 1, 1);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    Tuple light_pos = Point(-10, 10, -10);
    light_pos = Transformation::rotation_x(0).multiply(light_pos);

    World world;
    world.addLight(PointLight(light_pos, Colour(1, 0, 0)));

    world.addLight(PointLight(Transformation::translation(20, 0, 0).multiply(light_pos), Colour(0, 0, 1)));

    world.addShape(middle);

    int resolution_factor = 2;
    Camera camera = Camera(100 * resolution_factor, 50 * resolution_factor, M_PI/3);
    Matrix perspective = Transformation::view_transform(Point(0, 1.5, -5), Point(0, 1, 0), Vector(0, 1, 0));
    camera.set_transform(perspective);

    Canvas screen = camera.render(world);
    screen.write_ppm("lights.ppm");
}
