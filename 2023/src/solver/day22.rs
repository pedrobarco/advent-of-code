use std::collections::HashSet;

use super::Solver;

pub struct Day22 {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from(input: &str) -> Self {
        let coords: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();
        return Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    points: (Point, Point),
    label: usize,
    dependencies: Vec<usize>,
}

impl Brick {
    fn from(input: &str) -> Self {
        let points: Vec<Point> = input.split("~").map(|x| Point::from(x)).collect();
        let bounding_box = (
            Point {
                x: points[0].x.min(points[1].x),
                y: points[0].y.min(points[1].y),
                z: points[0].z.min(points[1].z),
            },
            Point {
                x: points[0].x.max(points[1].x),
                y: points[0].y.max(points[1].y),
                z: points[0].z.max(points[1].z),
            },
        );
        return Self {
            points: bounding_box,
            dependencies: Vec::new(),
            label: 0,
        };
    }

    fn collides_in_xy(&self, other: &Self) -> bool {
        return self.points.0.x <= other.points.1.x
            && self.points.1.x >= other.points.0.x
            && self.points.0.y <= other.points.1.y
            && self.points.1.y >= other.points.0.y;
    }

    fn fall_to(&mut self, z: usize) {
        let dz = self.points.1.z - self.points.0.z;
        self.points.0.z = z;
        self.points.1.z = z + dz;
    }
}

fn fall(bricks: &mut Vec<Brick>) {
    bricks.sort_by_key(|b| b.points.0.z);

    for i in 0..bricks.len() {
        let collisions = (0..i).filter(|&j| bricks[i].collides_in_xy(&bricks[j]));

        let z = collisions
            .clone()
            .map(|j| bricks[j].points.1.z)
            .max()
            .unwrap_or(0);

        let dependencies = collisions
            .filter(|&i| bricks[i].points.1.z == z)
            .map(|i| bricks[i].label)
            .collect();

        let b = &mut bricks[i];
        b.fall_to(z + 1);
        b.dependencies = dependencies;
    }
}

impl Solver for Day22 {
    fn p1(&self, input: &str) -> String {
        let mut bricks: Vec<Brick> = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let mut b = Brick::from(l);
                b.label = i;
                return b;
            })
            .collect();

        fall(&mut bricks);

        let res: usize = bricks
            .iter()
            .filter(|b| {
                !bricks
                    .iter()
                    .any(|other| other.dependencies.len() == 1 && other.dependencies[0] == b.label)
            })
            .count();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut bricks: Vec<Brick> = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let mut b = Brick::from(l);
                b.label = i;
                return b;
            })
            .collect();

        fall(&mut bricks);

        fn dependencies(labels: HashSet<usize>, bricks: &Vec<Brick>) -> usize {
            let mut next: HashSet<_> = bricks
                .iter()
                .filter(|b| {
                    b.dependencies.len() > 0 && b.dependencies.iter().all(|d| labels.contains(d))
                })
                .map(|b| b.label)
                .collect();

            next.extend(labels.clone());

            if labels.len() == next.len() {
                return labels.len() - 1;
            }

            return dependencies(next, bricks);
        }

        let res: usize = bricks
            .iter()
            .map(|b| dependencies(HashSet::from([b.label]), &bricks))
            .sum();

        return res.to_string();
    }
}
