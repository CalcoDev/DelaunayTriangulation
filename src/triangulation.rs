pub trait TriangulationPoint {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
struct PointImpl {
    x: f32,
    y: f32,
}

impl TriangulationPoint for PointImpl {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge(pub usize, pub usize);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub usize, pub usize, pub usize);
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1 && self.2 == other.2)
            || (self.0 == other.0 && self.1 == other.2 && self.2 == other.1)
            || (self.0 == other.1 && self.1 == other.0 && self.2 == other.2)
            || (self.0 == other.1 && self.1 == other.2 && self.2 == other.0)
            || (self.0 == other.2 && self.1 == other.0 && self.2 == other.1)
            || (self.0 == other.2 && self.1 == other.1 && self.2 == other.0)
    }
}

struct DoublePointSlice<'a, T1>
where
    T1: TriangulationPoint,
{
    p1: &'a [T1],
    p2: &'a [PointImpl],
}

impl<'a, T1: TriangulationPoint> DoublePointSlice<'a, T1> {
    fn new(p1: &'a [T1], p2: &'a [PointImpl]) -> Self {
        Self { p1, p2 }
    }

    #[inline(always)]
    fn get(&self, index: usize) -> PointImpl {
        if index < self.p1.len() {
            PointImpl {
                x: self.p1[index].x(),
                y: self.p1[index].y(),
            }
        } else {
            self.p2[index - self.p1.len()]
        }
    }
}

pub fn triangulate<T: TriangulationPoint>(points: &[T]) -> Vec<Triangle> {
    let mut super_triangle = [PointImpl { x: 0f32, y: 0f32 }; 3];
    super_triangle[0].x = 2000f32;
    super_triangle[0].y = 0f32;

    super_triangle[1].x = 0f32;
    super_triangle[1].y = 0f32;

    super_triangle[2].x = 0f32;
    super_triangle[2].y = 2000f32;

    let points_count = points.len();
    let all_points = DoublePointSlice::new(points, &super_triangle);

    let mut triangles = vec![Triangle(points_count, points_count + 1, points_count + 2)];
    let mut bad_triangles = Vec::<Triangle>::new();
    let mut polygon = Vec::<Edge>::new();

    for p_idx in 0..points_count + 3 {
        let p_idx = points_count + 3 - p_idx - 1;
        let p = all_points.get(p_idx);

        bad_triangles.clear();
        for t_idx in 0..triangles.len() {
            let t_idx = triangles.len() - t_idx - 1;

            let t = triangles[t_idx];
            let p0 = all_points.get(t.0);
            let p1 = all_points.get(t.1);
            let p2 = all_points.get(t.2);

            if check_in_circumcircle(&p, &p0, &p1, &p2) {
                bad_triangles.push(t);
            }
        }

        polygon.clear();
        for i in 0..bad_triangles.len() {
            let t = bad_triangles[i];
            let edges = [Edge(t.0, t.1), Edge(t.1, t.2), Edge(t.2, t.0)];

            for j in 0..edges.len() {
                let mut reject_edge = false;

                for k in 0..bad_triangles.len() {
                    let t2 = bad_triangles[k];
                    if t == t2 {
                        continue;
                    }

                    let edges2 = [Edge(t2.0, t2.1), Edge(t2.1, t2.2), Edge(t2.2, t2.0)];
                    if edges[j] == edges2[0] || edges[j] == edges2[1] || edges[j] == edges2[2] {
                        reject_edge = true;
                    }
                }

                if !reject_edge {
                    polygon.push(edges[j]);
                }
            }
        }

        for t in bad_triangles.iter() {
            triangles.retain(|x| x != t);
        }

        for e in polygon.iter() {
            triangles.push(Triangle(e.0, e.1, p_idx));
        }
    }

    triangles.retain(|t| t.0 < points_count && t.1 < points_count && t.2 < points_count);

    triangles
}

fn check_in_circumcircle<T: TriangulationPoint>(point: &T, p1: &T, p2: &T, p3: &T) -> bool {
    let (xc, yc, r) = compute_circumcircle(p1, p2, p3);

    let dx = xc - point.x();
    let dy = yc - point.y();
    let d = dx * dx + dy * dy;

    d <= r
}

fn compute_circumcircle<T: TriangulationPoint>(p1: &T, p2: &T, p3: &T) -> (f32, f32, f32) {
    let xa = p1.x();
    let ya = p1.y();

    let xb = p2.x();
    let yb = p2.y();

    let xc = p3.x();
    let yc = p3.y();

    let x2a = xa * xa;
    let y2a = ya * ya;

    let x2b = xb * xb;
    let y2b = yb * yb;

    let x2c = xc * xc;
    let y2c = yc * yc;

    let d = 2.0 * (xa * (yb - yc) + xb * (yc - ya) + xc * (ya - yb));
    let x = ((x2a + y2a) * (yb - yc) + (x2b + y2b) * (yc - ya) + (x2c + y2c) * (ya - yb)) / d;
    let y = ((x2a + y2a) * (xc - xb) + (x2b + y2b) * (xa - xc) + (x2c + y2c) * (xb - xa)) / d;
    let r = (x - xa) * (x - xa) + (y - ya) * (y - ya);

    (x, y, r)
}
