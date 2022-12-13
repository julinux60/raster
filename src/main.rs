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

    let c_a: Vec3J = Vec3J::new(cube_size + mid_w, cube_size + mid_h, cube_size);
    let c_b: Vec3J = Vec3J::new(-cube_size + mid_w, cube_size + mid_h, cube_size);
    let c_c: Vec3J = Vec3J::new(-cube_size + mid_w, cube_size + mid_h, -cube_size);
    let c_d: Vec3J = Vec3J::new(cube_size + mid_w, cube_size + mid_h, -cube_size);
    let c_e: Vec3J = Vec3J::new(cube_size + mid_w, -cube_size + mid_h, cube_size);
    let c_f: Vec3J = Vec3J::new(-cube_size + mid_w, -cube_size + mid_h, cube_size);
    let c_g: Vec3J = Vec3J::new(-cube_size + mid_w, -cube_size + mid_h, -cube_size);
    let c_h: Vec3J = Vec3J::new(cube_size + mid_w, -cube_size + mid_h, -cube_size);

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
