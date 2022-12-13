pub mod globalMod {
    pub const W: i32 = 800;
    pub const H: i32 = 800;

    pub const M_W: i32 = 400;
    pub const M_H: i32 = 400;

    //fonctions

    pub fn set_projection_matrix(angle_of_view: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
        let mut matrix_retour: [[f32; 4]; 4] = [[0.0; 4]; 4];

        let scale = 1.0 / f32::tan(angle_of_view * 0.5 * (3.14159256 / 180.0));

        matrix_retour[0][0] = scale;
        matrix_retour[1][1] = scale;
        matrix_retour[2][2] = -far / (far - near);
        matrix_retour[3][2] = -far * near / (far - near);
        matrix_retour[2][3] = -1.0;
        matrix_retour[3][3] = 0.0;

        return matrix_retour;
    }
}
