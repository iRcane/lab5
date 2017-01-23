use std::io;
use std::str::FromStr;
use std::fmt;

struct Vector {
	x: f64,
	y: f64,
	x_end: f64,
	y_end: f64
}

struct Point {
	x: f64,
	y: f64
}

struct Ray {
	start: Point,
	intersects: Point 
}

impl Point {
	fn distance (&self, p: &Point) -> f64 {
		((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
	}
}

impl Vector {
	fn get_start(&self) -> Point {
		Point { x: self.x, y: self.y}
    }

	fn get_end(&self) -> Point {
		Point { x: self.x_end, y: self.y_end}
	}

	fn has_point(&self, p: &Point) -> bool {
		p.x >= self.x && p.x <= self.x_end &&
		p.y >= self.y && p.y <= self.y_end
    }

	fn intersects (&self, ray: &Ray) -> bool {
		match self.get_intersection(ray) {

			Some(p) => {
				if self.has_point(&p) && ray.has_point(&p) {
					return true;
				}
				false
            },
			None => false
        }
	}

	fn get_intersection(&self, ray: &Ray) -> Option<Point> {
		let vec_ray = ray.to_vector();
		let zn = det(&self, &vec_ray);
		let eps = 1e-9;

		if zn.abs() < eps {
			return None;
		}
		let x = ((self.x * self.y_end - self.y * self.x_end) * (vec_ray.x - vec_ray.x_end) - (self.x - self.x_end) * (vec_ray.x * vec_ray.y_end - vec_ray.y * vec_ray.x_end)) / zn;
		let y = ((self.x * self.y_end - self.y * self.x_end) * (vec_ray.y - vec_ray.y_end) - (self.y - self.y_end) * (vec_ray.x * vec_ray.y_end - vec_ray.y * vec_ray.x_end)) / zn;
		
		Some(Point {x: x, y: y})
    }
}

impl Ray {
	fn new(start: Point, intersects: Point) -> Ray {
		Ray { start: start, intersects: intersects}
    }

	fn has_point(&self, p: &Point) -> bool {
		(p.x > self.start.x) == (self.intersects.x > self.start.x) &&
		(p.y > self.start.y) == (self.intersects.y > self.start.y)
    }

	fn to_vector(&self) -> Vector {
		Vector { x: self.start.x, y: self.start.y, x_end: self.intersects.x, y_end: self.intersects.y}
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {}, x_end: {}, y_end: {})", self.x, self.y, self.x_end, self.y_end)
    }
}


fn det (v1: &Vector, v2: &Vector) -> f64 {
	(v1.get_start().x - v1.get_end().x) * (v2.get_start().y - v2.get_end().y) - (v1.get_start().y - v1.get_end().y) * (v2.get_start().x - v2.get_end().x)
}

fn comma_split (string: &str) -> (f64, f64) {
	let mut ray_begin_xy = string.split(',');
	let x: f64 = FromStr::from_str(ray_begin_xy.next().unwrap()).unwrap();
	let y: f64 = FromStr::from_str(ray_begin_xy.next().unwrap()).unwrap();
	(x, y)
}

fn main() {
    let mut segments: Vec<Vector> = Vec::new();
	let mut input = String::new();
	let mut ray = String::new();
	
	io::stdin().read_line(&mut ray).unwrap();
	let mut iter = ray.split_whitespace();
	
	let ray_begin_xy = iter.next().unwrap();
	let (ray_x, ray_y) = comma_split(&ray_begin_xy);
	let ray_begin = Point {x: ray_x, y: ray_y };

	let ray_end_xy = iter.next().unwrap();
	let (ray_x, ray_y) = comma_split(&ray_end_xy);
	let ray_end = Point {x: ray_x, y: ray_y };

	let ray = Ray::new(ray_begin, ray_end);

	while let Ok(_) = io::stdin().read_line(&mut input) {
		if input == "\r\n" {
			break;
        }

		{
			let mut iter = input.split_whitespace();

			let mut seg_begin_xy = iter.next().unwrap();
			let (seg_x, seg_y) = comma_split(&seg_begin_xy);

			seg_begin_xy = iter.next().unwrap();
			let (seg_x_end, seg_y_end) = comma_split(&seg_begin_xy);

			let segment = Vector {
				x: seg_x, y: seg_y, 
				x_end: seg_x_end, y_end: seg_y_end
			};

			segments.push(segment);
        }
		input.clear();
    }

	let mut min_distance = std::f64::INFINITY;
	let mut n_seg = segments.first().unwrap();
	for seg in &segments {
		if seg.intersects(&ray) {
			let distance = seg.get_start().distance(&seg.get_intersection(&ray).unwrap());
			if min_distance > distance {
				min_distance = distance;
				n_seg = seg;
            }
        }
    }

    if min_distance == std::f64::INFINITY {
        println!();
    }
    else {
	    println!("Nearest segment: {}, distance: {:.3}", n_seg, min_distance);
    }
}
