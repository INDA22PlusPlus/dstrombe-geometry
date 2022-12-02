use std::io;
use std::collections::binary_heap;
use core::cmp::min;
use core::cmp::max;
use std::rc::Rc;
#[derive(Clone, Copy, Debug)]

struct Pos {
    x : i64,
    y : i64
}
#[derive(Clone, Debug)]
struct Edge {
    from : Pos,
    to : Pos
}
#[derive(Clone)]
struct MetaData {
    edges : Vec<Edge>,
    queries : Vec<Pos>,
}

fn read_line() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();
    buffer.clone()
}

fn line_as_int_vec(line : String) -> Vec<i64> {
    let mut to_ret = Vec::new();
    let split : Vec<String>  = line.trim().split(" ").map(|x| x.to_string()).collect();
    for s_item in split.iter() {
        to_ret.push(s_item.parse::<i64>().unwrap() as i64);
    }
    to_ret.clone()
}

fn std_in_to_ivec() -> Vec<i64> {
    line_as_int_vec(read_line())
}

fn parse_metadata() -> Option<MetaData> {
    let header = std_in_to_ivec();
    let vert_cnt =  header[0];
    if vert_cnt == 0 {
        return None;
    }

    let mut edges : Vec<Edge> = vec![Edge {from : Pos {x : 0, y : 0}, to : Pos {x : 0, y : 0}}; vert_cnt as usize];
    let mut set_prev = false;
    for i in 0..vert_cnt {
        set_prev = false;
        let line = std_in_to_ivec();
        let vert : Pos = Pos { x : line[0], y : line[1]};
        edges[i as usize].from = vert;
        if i == 0 {

        }
        else if i == vert_cnt - 1 {
            set_prev = true;
            edges[i as usize].to = edges[0].from.clone();
        }
        else {
            set_prev = true;
        }
        if set_prev {
            edges[i as usize - 1].to = vert;
        }
    }
    let query_cnt = std_in_to_ivec()[0] as usize;
    let mut queries : Vec<Pos> = Vec::new();
    for i in 0..query_cnt {
        let line = std_in_to_ivec();
        queries.push(Pos {x : line[0], y : line[1]});
    }
   
    Some(MetaData {
        edges,
        queries
    })
} 

// check if the triangle bounded by verts a, b, c is oriented counter clockwise
fn tri_is_counterclockwise (a : Pos, b : Pos, c : Pos) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}
 
// if the triangle bounded by a.from, and b and the one bounded by a.to, b
// are oriented in opposite orientations, it follows that a and b intersect 
//  *   .
//   .   *
fn edges_intersect (a : Edge, b : Edge) -> bool {
    tri_is_counterclockwise(a.from.clone(), b.from.clone(), b.to.clone()) ^ tri_is_counterclockwise(a.to.clone(), b.from.clone(), b.to.clone())
}

// check if point is in rect bounded by the verts of a line
fn inside_bounding_edge(a : &Pos, b : &Edge) -> bool {
    (a.x <= b.to.x && a.x >= b.from.x && a.y <= b.to.y && a.y >= b.from.y)
    ||
    (a.x <= b.from.x && a.x >= b.to.x && a.y <= b.from.y && a.y >= b.to.y)

}


// determinant
fn det(a : &Pos, b : &Pos) -> i64 {
    a.x * b.y - a.y*b.x
}
fn point_is_on_edge(a : &Pos, e : &Edge) -> bool {
    det(a, &Pos { x : e.to.x - e.from.x, y : e.to.y - e.from.y}) == 0 && inside_bounding_edge(a, e)
}
fn explore_all(state : &MetaData) {

    for v in &state.queries {
        let mut is_in_polygon : bool = false;

        for e in &state.edges {
            println!("Edge: {:?}", e);
            if point_is_on_edge(v, e) {
                println!("on");
                break;
            }
            is_in_polygon = if edges_intersect(Edge {from : v.clone(), to : Pos {x : 11000, y : 0} }, e.clone()) {
                !is_in_polygon
            }
            else {
                is_in_polygon
            };
        }
        if is_in_polygon {
            
            println!("in");
        }
        else {
            println!("out");
        }
        
    }
}



fn main() {
    loop {
        let case = parse_metadata();
        if case.is_none() {
            break;
        }
        explore_all(&case.unwrap());
    }
}

