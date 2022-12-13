mod geometry;
mod global;

use geometry::geometryMod::*;
use global::globalMod::*;
use raylib::prelude::*;

fn main() {
    //init

    let mut projection_matrix: [[f32; 4]; 4] = set_projection_matrix(90.0, 0.1, 100.0);
    let mut world_to_camera: [[f32; 4]; 4] = [[0.0; 4]; 4];
    world_to_camera[3][1] = -10.0;
    world_to_camera[3][1] = -20.0;

    let mut triangle_vec: Vec<TriangleJ> = Vec::new(); //create_cube(Vec3J::new(0.0, 0.0, 0.0), 150.0);
    let mut object_vec: Vec<Object3d> = Vec::new();

    object_vec.push(Object3d::new(
        triangle_vec.len(),
        triangle_vec.len() + 8,
        Vec3J::new(200.0, 0.0, 0.0),
    ));
    triangle_vec.append(&mut create_cube(Vec3J::new(200.0, 0.0, 0.0), 150.0));

    // object_vec.push(Object3d::new(
    //     triangle_vec.len(),
    //     triangle_vec.len() + 8,
    //     Vec3J::new(0.0, 0.0, 0.0),
    // ));
    // triangle_vec.append(&mut create_cube(Vec3J::new(0.0, 0.0, 0.0), 50.0));
    //
    // object_vec.push(Object3d::new(
    //     triangle_vec.len(),
    //     triangle_vec.len() + 8,
    //     Vec3J::new(-200.0, 0.0, 0.0),
    // ));
    // triangle_vec.append(&mut create_cube(Vec3J::new(-200.0, 0.0, 0.0), 50.0));

    // for object_i in 0..object_vec.len() {
    //     for i in object_vec[object_i].first_ref..object_vec[object_i].last_ref {
    //         triangle_vec[i].rotateZ(3.14159 / 4.0, object_vec[object_i].center);
    //         triangle_vec[i].rotateY(3.14159 / 4.0, object_vec[object_i].center);
    //     }
    // }

    let (mut rl, thread) = raylib::init().size(W, H).title("Rasterization").build();
    rl.set_target_fps(1);

    while !rl.window_should_close() {
        //updating

        // for object_i in 0..object_vec.len() {
        //     for i in object_vec[object_i].first_ref..object_vec[object_i].last_ref {
        //         triangle_vec[i].rotateZ(0.04, object_vec[object_i].center);
        //         triangle_vec[i].rotateY(0.03, object_vec[object_i].center);
        //     }
        // }

        for i in object_vec[0].first_ref..object_vec[0].last_ref {
            triangle_vec[i].rotateZ(0.04, object_vec[0].center);
            triangle_vec[i].rotateX(0.03, object_vec[0].center);
        }

        // for i in object_vec[1].first_ref..object_vec[1].last_ref {
        //     triangle_vec[i].rotateY(-0.04, object_vec[1].center);
        //     // triangle_vec[i].rotateY(-0.03, object_vec[1].center);
        // }
        //
        // for i in object_vec[2].first_ref..object_vec[2].last_ref {
        //     triangle_vec[i].rotateZ(-0.04, object_vec[2].center);
        //     // triangle_vec[i].rotateY(0.03, object_vec[2].center);
        // }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        //drawing
        d.draw_fps(10, 10);

        for i in 0..triangle_vec.len() {
            //orthographic

            // let pa = triangle_vec[i].a.get_projected_position(M_W, M_H);
            // let pb = triangle_vec[i].b.get_projected_position(M_W, M_H);
            // let pc = triangle_vec[i].c.get_projected_position(M_W, M_H);
            //
            // d.draw_line(pa.0, pa.1, pb.0, pb.1, Color::WHITE);
            // d.draw_line(pc.0, pc.1, pb.0, pb.1, Color::WHITE);
            // d.draw_line(pa.0, pa.1, pc.0, pc.1, Color::WHITE);

            //perspective

            let vertex: [Vec3J; 3] = [triangle_vec[i].a, triangle_vec[i].b, triangle_vec[i].c];
            let mut vertex_projected: [Vec3J; 3] = [Vec3J::new(0.0, 0.0, 0.0); 3];

            for vertex_i in 0..3 {
                let vert_camera: Vec3J =
                    Vec3J::mult_point_matrix(&vertex[vertex_i], &world_to_camera);
                // println!("{:?}", vert_camera);
                vertex_projected[vertex_i] =
                    Vec3J::mult_point_matrix(&vert_camera, &projection_matrix);

                d.draw_circle(
                    ((vertex_projected[vertex_i].x + 1.0) * 0.5 * W as f32) as i32,
                    400,
                    10.0,
                    Color::WHITE,
                );
            }
        }
    }
}

pub fn create_cube(center: Vec3J, cube_size: f32) -> Vec<TriangleJ> {
    let mut tab_retour_triangle: Vec<TriangleJ> = Vec::new();
    let offset_x_c = center.x; //(W / 2) as f32;
    let offset_y_c = center.y; //(H / 2) as f32;
    let offset_z_c = center.z; //(H / 2) as f32;

    let c_a: Vec3J = Vec3J::new(
        cube_size + offset_x_c,
        cube_size + offset_y_c,
        cube_size + offset_z_c,
    );
    let c_b: Vec3J = Vec3J::new(
        -cube_size + offset_x_c,
        cube_size + offset_y_c,
        cube_size + offset_z_c,
    );
    let c_c: Vec3J = Vec3J::new(
        -cube_size + offset_x_c,
        cube_size + offset_y_c,
        -cube_size + offset_z_c,
    );
    let c_d: Vec3J = Vec3J::new(
        cube_size + offset_x_c,
        cube_size + offset_y_c,
        -cube_size + offset_z_c,
    );
    let c_e: Vec3J = Vec3J::new(
        cube_size + offset_x_c,
        -cube_size + offset_y_c,
        cube_size + offset_z_c,
    );
    let c_f: Vec3J = Vec3J::new(
        -cube_size + offset_x_c,
        -cube_size + offset_y_c,
        cube_size + offset_z_c,
    );
    let c_g: Vec3J = Vec3J::new(
        -cube_size + offset_x_c,
        -cube_size + offset_y_c,
        -cube_size + offset_z_c,
    );
    let c_h: Vec3J = Vec3J::new(
        cube_size + offset_x_c,
        -cube_size + offset_y_c,
        -cube_size + offset_z_c,
    );

    tab_retour_triangle.push(TriangleJ::new(c_a, c_b, c_d));
    tab_retour_triangle.push(TriangleJ::new(c_d, c_b, c_c));
    tab_retour_triangle.push(TriangleJ::new(c_a, c_e, c_b));
    tab_retour_triangle.push(TriangleJ::new(c_e, c_b, c_f));
    tab_retour_triangle.push(TriangleJ::new(c_d, c_g, c_c));
    tab_retour_triangle.push(TriangleJ::new(c_d, c_h, c_g));
    tab_retour_triangle.push(TriangleJ::new(c_f, c_h, c_g));
    tab_retour_triangle.push(TriangleJ::new(c_e, c_h, c_f));

    return tab_retour_triangle;
}
