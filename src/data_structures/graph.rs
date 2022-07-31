use std::ops::{Add, Sub};
use uuid::Uuid;

pub enum Error {
    VertexAlreadyPresent,
    VertexNotFound,
    EdgeAlreadyPresent,
    EdgeNotFound,
}

type Result<T> = std::result::Result<T, Error>;

pub struct Graph<T, W: Add + Sub> {
    vertices: Vec<Vertex<T>>,
    edges: Vec<Edge<W>>,
}

impl<T, W: Add + Sub> Graph<T, W> {
    pub fn are_adjacent(&self, n: Vertex<T>, m: Vertex<T>) -> bool {
        self.edges
            .iter()
            .any(|e| e.from == n.id && e.to == m.id || e.to == n.id && e.from == m.id)
    }

    pub fn neighbors(&self, x: Vertex<T>) -> Vec<&Vertex<T>> {
        self.edges
            .iter()
            .filter(|e| e.from == x.id || e.to == x.id)
            .flat_map(|e| {
                self.vertices
                    .iter()
                    .find(|v| v.id == e.from || v.id == e.to)
            })
            .collect()
    }

    pub fn add_vertex(&mut self, x: Vertex<T>) -> Result<()> {
        match self.vertices.iter().find(|v| v.id == x.id) {
            Some(_) => Err(Error::VertexAlreadyPresent),
            None => {
                self.vertices.push(x);
                Ok(())
            }
        }
    }

    pub fn remove_vertex(&mut self, x: Vertex<T>) -> Result<()> {
        match self
            .vertices
            .iter()
            .enumerate()
            .find(|(_, v)| v.id == x.id)
            .map(|(i, _)| i)
        {
            Some(index) => {
                self.vertices.remove(index);
                Ok(())
            }
            None => Err(Error::VertexNotFound),
        }
    }

    pub fn add_edge(&mut self, edge: Edge<W>) -> Result<()> {
        match self
            .edges
            .iter()
            .find(|e| e.from == edge.from && e.to == edge.to)
        {
            Some(_) => Err(Error::EdgeAlreadyPresent),
            None => {
                self.edges.push(edge);
                Ok(())
            }
        }
    }

    pub fn remove_edge(&mut self, edge: Edge<W>) -> Result<()> {
        match self
            .edges
            .iter()
            .enumerate()
            .find(|(_, e)| e.from == edge.from && e.to == edge.to)
            .map(|(i, _)| i)
        {
            Some(index) => {
                self.edges.remove(index);
                Ok(())
            }
            None => Err(Error::EdgeNotFound),
        }
    }

    pub fn get_vertex(&self, id: Uuid) -> Option<&Vertex<T>> {
        self.vertices.iter().find(|v| v.id == id)
    }

    // get_vertex_value(G, x): returns the value associated with the vertex x;
    // set_vertex_value(G, x, v): sets the value associated with the vertex x to v.
}

// Implement Eq / PartialEq by id!
pub struct Vertex<T> {
    id: Uuid,
    data: Option<T>,
}

impl<T> Vertex<T> {
    fn new(data: Option<T>) -> Self {
        Vertex {
            data,
            id: Uuid::new_v4(),
        }
    }
}

// Implement Eq / PartialEq by id!

pub struct Edge<W: Add + Sub> {
    from: Uuid,
    to: Uuid,
    weight: Option<W>,
}

impl<W: Add + Sub> Edge<W> {
    fn new(from: Uuid, to: Uuid, weight: Option<W>) -> Edge<W> {
        Edge { from, to, weight }
    }
}

// Tiefensuche, Breitensuche und Bipartittät testen
// Adjazienzlisten / matrizen

/*
The basic operations provided by a graph data structure G usually include:[1]

Structures that associate values to the edges usually also provide:[1]

    get_edge_value(G, x, y): returns the value associated with the edge (x, y);
    set_edge_value(G, x, y, v): sets the value associated with the edge (x, y) to v.
*/
