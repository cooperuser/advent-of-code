use crate::prelude::*;
use crate::vector3::{Vector3, Vector3f};

use ndarray::prelude::*;
use ndarray_linalg::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    stones: Vec<Stone>,
    area: (Vector3f, Vector3f),
}

#[derive(Debug)]
struct Stone {
    position: Vector3,
    velocity: Vector3,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 2,
            answer_b: 47,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        Self {
            raw: raw.clone(),
            stones: raw.iter().map(|line| line.parse().unwrap()).collect(),
            area: if raw.len() < 10 {
                (Vector3f::new(7.0, 7.0, 0.0), Vector3f::new(27.0, 27.0, 1.0))
            } else {
                (
                    Vector3f::new(200000000000000.0, 200000000000000.0, 0.0),
                    Vector3f::new(400000000000000.0, 400000000000000.0, 0.0),
                )
            },
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut count = 0;
        let stones: Vec<_> = self.stones.iter().map(|s| s.xy()).collect();
        for (i, &a) in stones.iter().enumerate() {
            for &b in stones.iter().skip(i + 1) {
                let Some((scale_a, scale_b)) = Vector3f::closest_points(a, b) else {
                    continue;
                };
                if scale_a < 0.0 || scale_b < 0.0 {
                    continue;
                }
                let intersection = a.0 + a.1 * scale_a;
                if intersection.contained_in(self.area.0, self.area.1) {
                    count += 1;
                }
            }
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        // (p0 x v0) - (p0 x v[i]) - (p[i] x v0) + (p[i] x v[i])
        // (p0 x v0) - (p0 x v1) - (p1 x v0) + (p1 x v1)
        // (p0 x v0) - (p0 x v2) - (p2 x v0) + (p2 x v2)
        // -
        // -(p x v0) - (p0 x v) + (p0 x v0) + (p x v1) + (p1 x v) - (p1 x v1)
        // -(p x v0) - (p0 x v) + (p0 x v0) + (p x v2) + (p2 x v) - (p2 x v2)
        //
        // -(pp.y * v0.z - pp.z * v0.y) - (p0.y * vv.z - p0.z * vv.y) + (p0.y * v0.z - p0.z * v0.y) + (pp.y * v1.z - pp.z * v1.y) + (p1.y * vv.z - p1.z * vv.y) - (p1.y * v1.z - p1.z * v1.y)
        // -(pp.z * v0.x - pp.x * v0.z) - (p0.z * vv.x - p0.x * vv.z) + (p0.z * v0.x - p0.x * v0.z) + (pp.z * v1.x - pp.x * v1.z) + (p1.z * vv.x - p1.x * vv.z) - (p1.z * v1.x - p1.x * v1.z)
        // -(pp.x * v0.y - pp.y * v0.x) - (p0.x * vv.y - p0.y * vv.x) + (p0.x * v0.y - p0.y * v0.x) + (pp.x * v1.y - pp.y * v1.x) + (p1.x * vv.y - p1.y * vv.x) - (p1.x * v1.y - p1.y * v1.x)
        // -(pp.y * v0.z - pp.z * v0.y) - (p0.y * vv.z - p0.z * vv.y) + (p0.y * v0.z - p0.z * v0.y) + (pp.y * v2.z - pp.z * v2.y) + (p2.y * vv.z - p2.z * vv.y) - (p2.y * v2.z - p2.z * v2.y)
        // -(pp.z * v0.x - pp.x * v0.z) - (p0.z * vv.x - p0.x * vv.z) + (p0.z * v0.x - p0.x * v0.z) + (pp.z * v2.x - pp.x * v2.z) + (p2.z * vv.x - p2.x * vv.z) - (p2.z * v2.x - p2.x * v2.z)
        // -(pp.x * v0.y - pp.y * v0.x) - (p0.x * vv.y - p0.y * vv.x) + (p0.x * v0.y - p0.y * v0.x) + (pp.x * v2.y - pp.y * v2.x) + (p2.x * vv.y - p2.y * vv.x) - (p2.x * v2.y - p2.y * v2.x)
        //
        // (p0.y * v0.z - p0.z * v0.y) - (p0.y * v[i].z - p0.z * v[i].y) - (p[i].y * v0.z - p[i].z * v0.y) - (p[i].y * v[i].z - p[i].z * v[i].y)
        // (p0.z * v0.x - p0.x * v0.z) - (p0.z * v[i].x - p0.x * v[i].z) - (p[i].z * v0.x - p[i].x * v0.z) - (p[i].z * v[i].x - p[i].x * v[i].z)
        // (p0.x * v0.y - p0.y * v0.x) - (p0.x * v[i].y - p0.y * v[i].x) - (p[i].x * v0.y - p[i].y * v0.x) - (p[i].x * v[i].y - p[i].y * v[i].x)
        //
        // -(pp_y * v_0_z - pp_z * v_0_y) - (p_0_y * vv_z - p_0_z * vv_y) + (p_0_y * v_0_z - p_0_z * v_0_y) + (pp_y * v_1_z - pp_z * v_1_y) + (p_1_y * vv_z - p_1_z * vv_y) - (p_1_y * v_1_z - p_1_z * v_1_y)
        // -(pp_z * v_0_x - pp_x * v_0_z) - (p_0_z * vv_x - p_0_x * vv_z) + (p_0_z * v_0_x - p_0_x * v_0_z) + (pp_z * v_1_x - pp_x * v_1_z) + (p_1_z * vv_x - p_1_x * vv_z) - (p_1_z * v_1_x - p_1_x * v_1_z)
        // -(pp_x * v_0_y - pp_y * v_0_x) - (p_0_x * vv_y - p_0_y * vv_x) + (p_0_x * v_0_y - p_0_y * v_0_x) + (pp_x * v_1_y - pp_y * v_1_x) + (p_1_x * vv_y - p_1_y * vv_x) - (p_1_x * v_1_y - p_1_y * v_1_x)
        // -(pp_y * v_0_z - pp_z * v_0_y) - (p_0_y * vv_z - p_0_z * vv_y) + (p_0_y * v_0_z - p_0_z * v_0_y) + (pp_y * v_2_z - pp_z * v_2_y) + (p_2_y * vv_z - p_2_z * vv_y) - (p_2_y * v_2_z - p_2_z * v_2_y)
        // -(pp_z * v_0_x - pp_x * v_0_z) - (p_0_z * vv_x - p_0_x * vv_z) + (p_0_z * v_0_x - p_0_x * v_0_z) + (pp_z * v_2_x - pp_x * v_2_z) + (p_2_z * vv_x - p_2_x * vv_z) - (p_2_z * v_2_x - p_2_x * v_2_z)
        // -(pp_x * v_0_y - pp_y * v_0_x) - (p_0_x * vv_y - p_0_y * vv_x) + (p_0_x * v_0_y - p_0_y * v_0_x) + (pp_x * v_2_y - pp_y * v_2_x) + (p_2_x * vv_y - p_2_y * vv_x) - (p_2_x * v_2_y - p_2_y * v_2_x)
        //
        // pp_y * (v_1_z - v_0_z) + pp_z * (v_0_y - v_1_y) + vv_y * (p_0_z - p_1_z) + vv_z * (p_1_y - p_0_y) = -((p_0_y * v_0_z) - (p_0_z * v_0_y) - (p_1_y * v_1_z) + (p_1_z * v_1_y))
        // pp_x * (v_0_z - v_1_z) + pp_z * (v_1_x - v_0_x) + vv_x * (p_1_z - p_0_z) + vv_z * (p_0_x - p_1_x) = -((p_0_z * v_0_x) - (p_0_x * v_0_z) + (p_1_x * v_1_z) - (p_1_z * v_1_x))
        // pp_x * (v_1_y - v_0_y) + pp_y * (v_0_x - v_1_x) + vv_x * (p_0_y - p_1_y) + vv_y * (p_1_x - p_0_x) = -((p_0_x * v_0_y) - (p_0_y * v_0_x) - (p_1_x * v_1_y) + (p_1_y * v_1_x))
        // pp_y * (v_2_z - v_0_z) + pp_z * (v_0_y - v_2_y) + vv_y * (p_0_z - p_2_z) + vv_z * (p_2_y - p_0_y) = -((p_0_y * v_0_z) - (p_0_z * v_0_y) - (p_2_y * v_2_z) + (p_2_z * v_2_y))
        // pp_x * (v_0_z - v_2_z) + pp_z * (v_2_x - v_0_x) + vv_x * (p_2_z - p_0_z) + vv_z * (p_0_x - p_2_x) = -((p_0_z * v_0_x) - (p_0_x * v_0_z) + (p_2_x * v_2_z) - (p_2_z * v_2_x))
        // pp_x * (v_2_y - v_0_y) + pp_y * (v_0_x - v_2_x) + vv_x * (p_0_y - p_2_y) + vv_y * (p_2_x - p_0_x) = -((p_0_x * v_0_y) - (p_0_y * v_0_x) - (p_2_x * v_2_y) + (p_2_y * v_2_x))
        //
        let p_0 = self.stones[0].position;
        let p_0 = Vector3f::new(p_0.x as f64, p_0.y as f64, p_0.z as f64);
        let v_0 = self.stones[0].velocity;
        let v_0 = Vector3f::new(v_0.x as f64, v_0.y as f64, v_0.z as f64);
        let p_1 = self.stones[2].position;
        let p_1 = Vector3f::new(p_1.x as f64, p_1.y as f64, p_1.z as f64);
        let v_1 = self.stones[2].velocity;
        let v_1 = Vector3f::new(v_1.x as f64, v_1.y as f64, v_1.z as f64);
        let p_2 = self.stones[3].position;
        let p_2 = Vector3f::new(p_2.x as f64, p_2.y as f64, p_2.z as f64);
        let v_2 = self.stones[3].velocity;
        let v_2 = Vector3f::new(v_2.x as f64, v_2.y as f64, v_2.z as f64);
        let a: Array2<f64> = array![
            [
                0.0,
                v_1.z - v_0.z,
                v_0.y - v_1.y,
                0.0,
                p_0.z - p_1.z,
                p_1.y - p_0.y,
            ],
            [
                v_0.z - v_1.z,
                0.0,
                v_1.x - v_0.x,
                p_1.z - p_0.z,
                0.0,
                p_0.x - p_1.x,
            ],
            [
                v_1.y - v_0.y,
                v_0.x - v_1.x,
                0.0,
                p_0.y - p_1.y,
                p_1.x - p_0.x,
                0.0,
            ],
            [
                0.0,
                v_2.z - v_0.z,
                v_0.y - v_2.y,
                0.0,
                p_0.z - p_2.z,
                p_2.y - p_0.y,
            ],
            [
                v_0.z - v_2.z,
                0.0,
                v_2.x - v_0.x,
                p_2.z - p_0.z,
                0.0,
                p_0.x - p_2.x,
            ],
            [
                v_2.y - v_0.y,
                v_0.x - v_2.x,
                0.0,
                p_0.y - p_2.y,
                p_2.x - p_0.x,
                0.0,
            ],
        ];
        let b = array![
            -((p_0.y * v_0.z) - (p_0.z * v_0.y) - (p_1.y * v_1.z) + (p_1.z * v_1.y)),
            -((p_0.z * v_0.x) - (p_0.x * v_0.z) + (p_1.x * v_1.z) - (p_1.z * v_1.x)),
            -((p_0.x * v_0.y) - (p_0.y * v_0.x) - (p_1.x * v_1.y) + (p_1.y * v_1.x)),
            -((p_0.y * v_0.z) - (p_0.z * v_0.y) - (p_2.y * v_2.z) + (p_2.z * v_2.y)),
            -((p_0.z * v_0.x) - (p_0.x * v_0.z) + (p_2.x * v_2.z) - (p_2.z * v_2.x)),
            -((p_0.x * v_0.y) - (p_0.y * v_0.x) - (p_2.x * v_2.y) + (p_2.y * v_2.x)),
        ];
        let x = a.solve_into(b).unwrap();
        let position = Vector3::new(
            x[0].round() as i64,
            x[1].round() as i64,
            x[2].round() as i64,
        );
        let _velocity = Vector3::new(
            x[3].round() as i64,
            x[4].round() as i64,
            x[5].round() as i64,
        );
        Some(position.x + position.y + position.z)
    }
}

impl Stone {
    const fn xy(&self) -> (Vector3f, Vector3f) {
        (
            Vector3f::new(self.position.x as f64, self.position.y as f64, 0.0),
            Vector3f::new(self.velocity.x as f64, self.velocity.y as f64, 0.0),
        )
    }
}

impl std::str::FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" @ ").unwrap();
        let left: Vec<_> = left.split(", ").map(|s| s.trim()).collect();
        let right: Vec<_> = right.split(", ").map(|s| s.trim()).collect();
        Ok(Stone {
            position: Vector3::new(
                left[0].parse().unwrap(),
                left[1].parse().unwrap(),
                left[2].parse().unwrap(),
            ),
            velocity: Vector3::new(
                right[0].parse().unwrap(),
                right[1].parse().unwrap(),
                right[2].parse().unwrap(),
            ),
        })
    }
}

crate::solution::test_solution!();
