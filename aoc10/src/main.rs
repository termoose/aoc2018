#[macro_use] extern crate scan_fmt;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::ops::Add;

#[derive(Debug, Clone)]
struct Coord {
    x: i128,
    y: i128
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Debug)]
struct Point {
    pos: Coord,
    vel: Coord
}

#[derive(Debug, Clone)]
struct BoundingBox {
    tl: Coord,
    br: Coord
}

type Points = Vec<Point>;

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);
    
    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn to_point(line: &str) -> Point {
    let (px, py, vx, vy)
        = scan_fmt!(line,
                    "position=<{d},{d}> velocity=<{d},{d}>",
                    i128, i128, i128, i128);
    
    Point {
        pos: Coord {x: px.unwrap(),
                    y: py.unwrap()},
        vel: Coord {x: vx.unwrap(),
                    y: vy.unwrap()}
    }
}

fn bounding_box(points: &Points) -> BoundingBox {
    if points.is_empty() {
        return BoundingBox{tl: Coord{x:0, y:0},
                           br: Coord{x:0, y:0}};
    }

    let mut max: Coord = points[0].pos.clone();
    let mut min: Coord = points[0].pos.clone();

    for p in points {
        if p.pos.x > max.x {
            max.x = p.pos.x;
        }
        if p.pos.y > max.y {
            max.y = p.pos.y;
        }

        if p.pos.x < min.x {
            min.x = p.pos.x;
        }
        if p.pos.y < min.y {
            min.y = p.pos.y;
        }
    }

    BoundingBox{tl: Coord {x: min.x, y: max.y},
                br: Coord {x: max.x, y: min.y}}
}

fn is_point(point: Coord, points: &Points) -> bool {
    points.iter()
        .any(|p| { point.x == p.pos.x && point.y == p.pos.y })
}

fn print(points: &Points, bb: &BoundingBox) {
    println!("x: {:?}", bb.tl.x..bb.br.x);
    for y in (bb.br.y-3)..(bb.tl.y+3) {
        for x in (bb.tl.x-3)..(bb.br.x+3) {
            if is_point(Coord{x: x, y: y}, points) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn iter(point: &Point) -> Point {
    Point {pos: point.pos.clone() + point.vel.clone(),
           vel: point.vel.clone()}
}

fn iters(points: &Points) -> Points {
    points.iter()
        .map(|p| iter(&p))
        .collect()
}

fn box_size(bb: &BoundingBox) -> i128 {
    //(bb.tl.x - bb.br.x) * (bb.tl.y - bb.br.y)
    bb.br.x.saturating_sub(bb.tl.x)
        .saturating_mul(bb.tl.y.saturating_sub(bb.br.y))
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let list = str_list(&f);
    let mut points = list.iter()
        .map(|l| to_point(&l))
        .collect();

    let bb = bounding_box(&points);

    let mut prev_box = bb.clone();
    let mut min_box = box_size(&bb);
    let mut iter = 0;
    loop {
        iter += 1;
        points = iters(&points);
        let bb = bounding_box(&points);
        let bb_size = box_size(&bb);

        if bb_size == 549 {
            println!("Iter: {}", iter);
            print(&points, &bb);
        }

        if bb_size > min_box {
        //if box_size(&bb) > box_size(&prev_box) {
            println!("Size: {} BB: {:?}", min_box, bb);
            //print(&points, &prev_box);
            break;
        }
        else {
            println!("Size: {} BB: {:?}", bb_size, bb);
            prev_box = bb;
            min_box = bb_size;
        }
    }

    // for point in points {
    //     println!("{:?}", point);
    // }
    //println!("Bounding box: {:?}", bb);
}
