use std::ops::{Add, Sub};
use uuid::Uuid;

type 

#[derive(Clone)]
pub struct Graph<W: Add> {
	vertices: Vec<Vertex>,
	edges: Vec<Edge<W>>
}

impl<W: Add> Graph<W> {
	// tests whether there is an edge from the vertex x to the vertex y;
	fn are_adjacent(&self, n: Vertex, m: Vertex) -> bool {
		false
	}
	
    // neighbors(G, x): lists all vertices y such that there is an edge from the vertex x to the vertex y;
    // add_vertex(G, x): adds the vertex x, if it is not there;
    // remove_vertex(G, x): removes the vertex x, if it is there;
    // add_edge(G, x, y): adds the edge from the vertex x to the vertex y, if it is not there;
    // remove_edge(G, x, y): removes the edge from the vertex x to the vertex y, if it is there;
    // get_vertex_value(G, x): returns the value associated with the vertex x;
    // set_vertex_value(G, x, v): sets the value associated with the vertex x to v.
}

#[derive(Clone, PartialEq, Eq)]
pub struct Vertex {
	name: String,
	uuid: Uuid
}

impl Vertex {
	fn new(name: String) -> Vertex {
		Vertex {
			name,
			uuid: Uuid::new_v4()
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
struct Edge<W: Add> {
	uuid: Uuid,
	from: Uuid,
	to: Uuid,
	name: Option<String>,
	weight: Option<W>,
}

impl<W: Add> Edge<W> {
	fn new(from: Uuid, to: Uuid, name: Option<String>, weight: Option<W>) -> Edge<W> {
		Edge {
			uuid: Uuid::new_v4(),
			from,
			to,
			name,
			weight
		}
	}
}


/*
The basic operations provided by a graph data structure G usually include:[1]

Structures that associate values to the edges usually also provide:[1]

    get_edge_value(G, x, y): returns the value associated with the edge (x, y);
    set_edge_value(G, x, y, v): sets the value associated with the edge (x, y) to v.
*/