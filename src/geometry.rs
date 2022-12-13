pub mod geometryMod {
    #[derive(Debug, Copy, Clone)]
    pub struct Vec3J {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Vec3J {
        pub fn new(x: f32, y: f32, z: f32) -> Vec3J {
            return Vec3J { x: x, y: y, z: z };
        }

        pub fn add(&self, otherVec3: Vec3J) -> Vec3J {
            return Vec3J::new(
                self.x + otherVec3.x,
                self.y + otherVec3.y,
                self.z + otherVec3.z,
            );
        }

        pub fn sub(&self, otherVec3: Vec3J) -> Vec3J {
            return Vec3J::new(
                self.x - otherVec3.x,
                self.y - otherVec3.y,
                self.z - otherVec3.z,
            );
        }

        pub fn mult(&self, otherVec3: Vec3J) -> Vec3J {
            return Vec3J::new(
                self.x * otherVec3.x,
                self.y * otherVec3.y,
                self.z * otherVec3.z,
            );
        }

        pub fn multByValue(&mut self, v: f32) {
            self.x *= v;
            self.y *= v;
            self.z *= v;
        }

        pub fn dot(&self, otherVec3: Vec3J) -> f32 {
            return self.x * otherVec3.x + self.y * otherVec3.y + self.z * otherVec3.z;
        }

        pub fn cross(&self, otherVec3: Vec3J) -> Vec3J {
            return Vec3J::new(
                self.y * otherVec3.z - self.z * otherVec3.y,
                self.z * otherVec3.x - self.x * otherVec3.z,
                self.x * otherVec3.y - self.y * otherVec3.x,
            );
        }

        pub fn get_projected_position(&self, m_w: i32, m_h: i32) -> (i32, i32) {
            return (self.x as i32 + m_w, self.y as i32 + m_h);
        }

        pub fn mult_point_matrix(v_in: &Vec3J, m: &[[f32; 4]; 4]) -> Vec3J {
            let mut retour_vecj = Vec3J::new(0.0, 0.0, 0.0);

            retour_vecj.x =
                v_in.x * m[0][0] + v_in.y * m[1][0] + v_in.z * m[2][0] + /* v_in.z = 1 */ m[3][0];
            retour_vecj.y =
                v_in.x * m[0][1] + v_in.y * m[1][1] + v_in.z * m[2][1] + /* v_in.z = 1 */ m[3][1];
            retour_vecj.z =
                v_in.x * m[0][2] + v_in.y * m[1][2] + v_in.z * m[2][2] + /* v_in.z = 1 */ m[3][2];
            let w: f32 =
                v_in.x * m[0][3] + v_in.y * m[1][3] + v_in.z * m[2][3] + /* v_in.z = 1 */ m[3][3];

            println!("{:?}", w);

            if (w != 1.0) {
                retour_vecj.x /= w;
                retour_vecj.y /= w;
                retour_vecj.z /= w;
            }

            return retour_vecj;
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct TriangleJ {
        pub a: Vec3J,
        pub b: Vec3J,
        pub c: Vec3J,
    }

    impl TriangleJ {
        pub fn new(nA: Vec3J, nB: Vec3J, nC: Vec3J) -> TriangleJ {
            return TriangleJ {
                a: nA,
                b: nB,
                c: nC,
            };
        }
        pub fn display(&self) {
            println!("Affichage Triangle");
            println!("A : {:?}", self.a);
            println!("B : {:?}", self.b);
            println!("C : {:?}\n", self.c);
        }

        pub fn rotateX(&mut self, x: f32, rotationPoint: Vec3J) {
            let mut nA = self.a.sub(rotationPoint);
            let mut nB = self.b.sub(rotationPoint);
            let mut nC = self.c.sub(rotationPoint);

            if x != 0.0 {
                let da = nA.y.hypot(nA.z);
                let thetaA = nA.y.atan2(nA.z) + x;
                nA.z = rotationPoint.z + da * thetaA.cos();
                nA.y = rotationPoint.y + da * thetaA.sin();

                let db = nB.y.hypot(nB.z);
                let thetaB = nB.y.atan2(nB.z) + x;
                nB.z = rotationPoint.z + db * thetaB.cos();
                nB.y = rotationPoint.y + db * thetaB.sin();

                let dc = nC.y.hypot(nC.z);
                let thetaC = nC.y.atan2(nC.z) + x;
                nC.z = rotationPoint.z + dc * thetaC.cos();
                nC.y = rotationPoint.y + dc * thetaC.sin();

                nA = nA.add(rotationPoint);
                nB = nB.add(rotationPoint);
                nC = nC.add(rotationPoint);
                self.a = nA;
                self.b = nB;
                self.c = nC;
            }
        }
        pub fn rotateZ(&mut self, z: f32, rotationPoint: Vec3J) {
            let mut nA = self.a.sub(rotationPoint);
            let mut nB = self.b.sub(rotationPoint);
            let mut nC = self.c.sub(rotationPoint);

            if z != 0.0 {
                let da = nA.y.hypot(nA.x);
                let thetaA = nA.y.atan2(nA.x) + z;
                nA.x = rotationPoint.x + da * thetaA.cos();
                nA.y = rotationPoint.y + da * thetaA.sin();

                let db = nB.y.hypot(nB.x);
                let thetaB = nB.y.atan2(nB.x) + z;
                nB.x = rotationPoint.x + db * thetaB.cos();
                nB.y = rotationPoint.y + db * thetaB.sin();

                let dc = nC.y.hypot(nC.x);
                let thetaC = nC.y.atan2(nC.x) + z;
                nC.x = rotationPoint.x + dc * thetaC.cos();
                nC.y = rotationPoint.y + dc * thetaC.sin();

                self.a = nA;
                self.b = nB;
                self.c = nC;
            }
        }

        pub fn rotateY(&mut self, y: f32, rotationPoint: Vec3J) {
            let mut nA = self.a.sub(rotationPoint);
            let mut nB = self.b.sub(rotationPoint);
            let mut nC = self.c.sub(rotationPoint);

            if y != 0.0 {
                let da = nA.x.hypot(nA.z);
                let thetaA = nA.x.atan2(nA.z) + y;
                nA.z = rotationPoint.z + da * thetaA.cos();
                nA.x = rotationPoint.x + da * thetaA.sin();

                let db = nB.x.hypot(nB.z);
                let thetaB = nB.x.atan2(nB.z) + y;
                nB.z = rotationPoint.z + db * thetaB.cos();
                nB.x = rotationPoint.x + db * thetaB.sin();

                let dc = nC.x.hypot(nC.z);
                let thetaC = nC.x.atan2(nC.z) + y;
                nC.z = rotationPoint.z + dc * thetaC.cos();
                nC.x = rotationPoint.x + dc * thetaC.sin();

                self.a = nA;
                self.b = nB;
                self.c = nC;
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Object3d {
        pub first_ref: usize,
        pub last_ref: usize,
        pub center: Vec3J,
    }

    impl Object3d {
        pub fn new(f_ref: usize, l_ref: usize, center: Vec3J) -> Object3d {
            return Object3d {
                first_ref: f_ref,
                last_ref: l_ref,
                center: center,
            };
        }
    }
}
