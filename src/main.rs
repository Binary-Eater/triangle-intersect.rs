use std::io;

#[derive(Clone, Copy)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

impl Vertex {
    fn new(x: f64, y: f64, z: f64) -> Vertex {
        Vertex { x, y, z }
    }

    fn subtract(&self, v: &Vertex) -> Vertex {
        Vertex {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    fn cross_product(&self, v: &Vertex) -> Vertex {
        // Yeah, I am using a Vertex as a 3-by-1 matrix...
        Vertex {
            x: (self.y * v.z) - (self.z * v.y),
            y: -((self.x * v.z) - (self.z * v.x)),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    fn dot_product(&self, v: &Vertex) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }
}

struct Edge {
    vertices: [Vertex; 2],
}

impl Edge {
    fn new(u: Vertex, v: Vertex) -> Edge {
        Edge { vertices: [u, v] }
    }
}

struct Triangle {
    vertices: [Vertex; 3],
    edges: [Edge; 3],
}

/*
 * V = (1/6) * det(a - d, b - d, c - d)
 * V = (1/6) * (([a - d] X [b - d]) o [c - d])
 *
 * Link: https://en.wikipedia.org/wiki/Tetrahedron#Volume
 */
fn tetrahedran_signed_volume(a: &Vertex, b: &Vertex, c: &Vertex, d: &Vertex) -> f64 {
    (a.subtract(d)
        .cross_product(&b.subtract(d))
        .dot_product(&c.subtract(d)))
        / 6.0
}

impl Triangle {
    fn new(u: Vertex, v: Vertex, w: Vertex) -> Triangle {
        Triangle {
            vertices: [u, v, w],
            edges: [Edge::new(u, v), Edge::new(v, w), Edge::new(w, u)],
        }
    }

    /* Link: https://stackoverflow.com/a/42752998 */
    fn edge_intersect(&self, e: &Edge) -> bool {
        // Use each point in the provided edge
        let mut sv_e_arr: [f64; 2] = [0.0; 2];
        for i in 0..2 {
            sv_e_arr[i] = tetrahedran_signed_volume(
                &self.vertices[0],
                &self.vertices[1],
                &self.vertices[2],
                &e.vertices[i],
            );
        }

        // Use each of the triangles edges
        let mut sv_t_arr: [f64; 3] = [0.0; 3];
        for i in 0..3 {
            sv_t_arr[i] = tetrahedran_signed_volume(
                &self.edges[i].vertices[0],
                &self.edges[i].vertices[1],
                &e.vertices[0],
                &e.vertices[1],
            );
        }

        sv_e_arr[0].signum() != sv_e_arr[1].signum()
            && sv_t_arr[0].signum() == sv_t_arr[1].signum()
            && sv_t_arr[1].signum() == sv_t_arr[2].signum()
            && sv_t_arr[2].signum() == sv_t_arr[0].signum()
    }

    fn intersect(&self, t: &Triangle) -> bool {
        // Check if any edge in t intersects self
        for edge in &t.edges {
            if self.edge_intersect(edge) {
                return true;
            }
        }

        // Check if any edge in self intersects t
        for edge in &self.edges {
            if t.edge_intersect(edge) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let mut triangle_idx = 1;
    let mut vertex_idx = 1;

    let mut vertices = [Vertex::new(0.0, 0.0, 0.0); 6];

    while triangle_idx <= 2 {
        println!(
            "Please input floating point values (ex. 0.0 0.0 0.0) for vertex {} of triangle {}.",
            vertex_idx, triangle_idx
        );

        let mut coords = String::new();

        io::stdin()
            .read_line(&mut coords)
            .expect("Failed to read line");

        let mut coord_vec: Vec<f64> = Vec::new();

        for coord in coords.split_ascii_whitespace() {
            let coord: f64 = match coord.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            coord_vec.push(coord);
        }

        if coord_vec.len() != 3 {
            continue;
        }

        vertices[((triangle_idx - 1) * 3) + (vertex_idx - 1)] =
            Vertex::new(coord_vec[0], coord_vec[1], coord_vec[2]);

        vertex_idx += 1;

        if vertex_idx > 3 {
            triangle_idx += 1;
            vertex_idx = 1;
        }
    }

    let t1 = Triangle::new(
        vertices[0],
        vertices[1],
        vertices[2],
    );
    let t2 = Triangle::new(
        vertices[3],
        vertices[4],
        vertices[5],
    );

    println!(
        "Do the two triangles intersect?: {}",
        if t1.intersect(&t2) { "yes" } else { "no" }
    )
}
