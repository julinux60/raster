mod geometry;
mod global;

use geometry::geometryMod::*;
use global::globalMod::*;
use raylib::prelude::*;

fn main() {
    //init
    let center_of_cube = Vec3J::new(0.0, 0.0, 0.0);

    let mut triangle_vec: Vec<TriangleJ> = create_cube(Vec3J::new(0.0, 0.0, 0.0), 150.0);

    let (mut rl, thread) = raylib::init().size(W, H).title("Rasterization").build();
    rl.set_target_fps(75);

    while !rl.window_should_close() {
        //updating

        for i in 0..triangle_vec.len() {
            triangle_vec[i].rotateZ(0.04, center_of_cube);
            triangle_vec[i].rotateY(0.03, center_of_cube);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        //drawing
        d.draw_fps(10, 10);

        for i in 0..triangle_vec.len() {
            let pa = triangle_vec[i].a.get_projected_position(M_W, M_H);
            let pb = triangle_vec[i].b.get_projected_position(M_W, M_H);
            let pc = triangle_vec[i].c.get_projected_position(M_W, M_H);

            // d.draw_circle(pa.0, pa.1, 5.0, Color::WHITE);
            // d.draw_circle(pb.0, pb.1, 5.0, Color::WHITE);
            // d.draw_circle(pc.0, pc.1, 5.0, Color::WHITE);

            d.draw_line(pa.0, pa.1, pb.0, pb.1, Color::WHITE);
            d.draw_line(pc.0, pc.1, pb.0, pb.1, Color::WHITE);
            d.draw_line(pa.0, pa.1, pc.0, pc.1, Color::WHITE);
        }
    }
}

pub fn create_cube(center: Vec3J, cube_size: f32) -> Vec<TriangleJ> {
    let mut tab_retour_triangle: Vec<TriangleJ> = Vec::new();
    let mid_w = center.x; //(W / 2) as f32;
    let mid_h = center.y; //(H / 2) as f32;

    let A: Vec3J = Vec3J::new(cube_size + mid_w, cube_size + mid_h, cube_size);
    let B: Vec3J = Vec3J::new(-cube_size + mid_w, cube_size + mid_h, cube_size);
    let C: Vec3J = Vec3J::new(-cube_size + mid_w, cube_size + mid_h, -cube_size);
    let D: Vec3J = Vec3J::new(cube_size + mid_w, cube_size + mid_h, -cube_size);
    let E: Vec3J = Vec3J::new(cube_size + mid_w, -cube_size + mid_h, cube_size);
    let F: Vec3J = Vec3J::new(-cube_size + mid_w, -cube_size + mid_h, cube_size);
    let G: Vec3J = Vec3J::new(-cube_size + mid_w, -cube_size + mid_h, -cube_size);
    let H2: Vec3J = Vec3J::new(cube_size + mid_w, -cube_size + mid_h, -cube_size);

    tab_retour_triangle.push(TriangleJ::new(A, B, D));
    tab_retour_triangle.push(TriangleJ::new(D, B, C));
    tab_retour_triangle.push(TriangleJ::new(A, E, B));
    tab_retour_triangle.push(TriangleJ::new(E, B, F));
    tab_retour_triangle.push(TriangleJ::new(D, G, C));
    tab_retour_triangle.push(TriangleJ::new(D, H2, G));
    tab_retour_triangle.push(TriangleJ::new(F, H2, G));
    tab_retour_triangle.push(TriangleJ::new(E, H2, F));

    return tab_retour_triangle;
}
