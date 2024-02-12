use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Vertex<T1, T2> {
    id: u32,
    value: T1,
    edges: HashMap<u32, Rc<RefCell<Edge<T1, T2>>>>,
}

struct Edge<T1, T2> {
    value: T2,
    out_vertex: Rc<RefCell<Vertex<T1, T2>>>,
    in_vertex: Rc<RefCell<Vertex<T1, T2>>>,
}

struct Graph<T1, T2> {
    vertices: HashMap<u32, Rc<RefCell<Vertex<T1, T2>>>>,
    edges: Vec<Rc<RefCell<Edge<T1, T2>>>>,
}


impl<T1, T2> Vertex<T1, T2> {
    fn new(id: u32, value: T1) -> Self {
        Vertex{ id, value, edges: HashMap::new() }
    }

    fn get_value(&self) -> &T1 {
        &self.value
    }

    fn get_id(&self) -> &u32 {
        &self.id
    }
}

impl<T1, T2> Edge<T1, T2> {
    fn new(in_vertex: Rc<RefCell<Vertex<T1, T2>>>, out_vertex: Rc<RefCell<Vertex<T1, T2>>>, value: T2) -> Edge<T1, T2> {
        Edge{ in_vertex, out_vertex, value }
    }

}

impl<T1, T2> Graph<T1, T2> {

    fn new() -> Self {
        Graph { vertices: HashMap::new(), edges: Vec::new() }
    }

    pub fn add_vertex(&mut self, id: u32, value: T1) -> Option<Rc<RefCell<Vertex<T1, T2>>>> {
        self.vertices.insert(id, Rc::new(RefCell::new(Vertex::new(id, value))))
    }

    pub fn get_vertex(&self, id: & u32) -> Option<&Rc<RefCell<Vertex<T1, T2>>>> {
        self.vertices.get(id)
    }

    pub fn add_edge(&mut self, id_in: u32, id_out: u32, value: T2) -> Option<Rc<RefCell<Edge<T1, T2>>>> {
        if id_in == id_out {
            return None
        }

        let v_in = self.get_vertex(&id_in);
        let v_out = self.get_vertex(&id_out);

        if v_in.is_none() || v_out.is_none() {
            return None
        }

        let input_vertex = v_in.cloned().unwrap();
        let output_vertex = v_out.cloned().unwrap();
        let edge =
            Rc::new(
                RefCell::new(
                    Edge::new(input_vertex.clone(), output_vertex.clone(), value)));

        input_vertex.borrow_mut().edges.insert(*output_vertex.borrow().get_id(), edge.clone());
        output_vertex.borrow_mut().edges.insert(*input_vertex.borrow().get_id(), edge.clone());
        self.edges.push(edge.clone());

        Some(edge)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_vertex() {
        let mut graph = Graph::<u32, u32>::new();
        graph.add_vertex(1, 10);
        graph.add_vertex(2, 20);
        graph.add_vertex(3, 30);
        graph.add_vertex(4, 40);
        graph.add_vertex(5, 50);

        assert_eq!(graph.get_vertex(&1).unwrap().borrow().get_value(), &10);
        assert_eq!(graph.get_vertex(&2).unwrap().borrow().get_value(), &20);
        assert_eq!(graph.get_vertex(&3).unwrap().borrow().get_value(), &30);
        assert_eq!(graph.get_vertex(&4).unwrap().borrow().get_value(), &40);
        assert_eq!(graph.get_vertex(&5).unwrap().borrow().get_value(), &50);
        assert!(graph.get_vertex(&6).is_none());
    }

    #[test]
    fn add_edges() {
        let mut graph = Graph::<u32, u32>::new();
        graph.add_vertex(1, 10);
        graph.add_vertex(2, 20);
        graph.add_vertex(3, 30);
        graph.add_vertex(4, 40);

        assert!(graph.add_edge(1, 5, 0).is_none());
        assert!(graph.add_edge(7, 5, 0).is_none());
        assert!(graph.add_edge(5, 1, 0).is_none());
        assert!(graph.add_edge(1, 1, 0).is_none());
        assert!(graph.add_edge(1, 2, 10).is_some());
        assert!(graph.add_edge(2, 3, 20).is_some());

        assert_eq!(graph.edges.len(), 2);
        let vertex1 = graph.get_vertex(&1).unwrap().borrow();
        let edge1 = vertex1.edges.get(&2);
        assert!(edge1.is_some());
        assert_eq!(edge1.unwrap().borrow().value, 10);
        assert_eq!(edge1.unwrap().borrow().in_vertex.borrow().get_id(), &1);
        assert_eq!(edge1.unwrap().borrow().out_vertex.borrow().get_id(), &2);


        let vertex2 = graph.get_vertex(&2).unwrap().borrow();
        let edge2 = vertex2.edges.get(&1);
        assert!(edge2.is_some());
        assert_eq!(edge2.unwrap().borrow().value, 10);
        assert_eq!(edge2.unwrap().borrow().in_vertex.borrow().get_id(), &1);
        assert_eq!(edge2.unwrap().borrow().out_vertex.borrow().get_id(), &2);

        let vertex3 = graph.get_vertex(&2).unwrap().borrow();
        let edge3 = vertex3.edges.get(&3);
        assert!(edge3.is_some());
        assert_eq!(edge3.unwrap().borrow().value, 20);
        assert_eq!(edge3.unwrap().borrow().in_vertex.borrow().get_id(), &2);
        assert_eq!(edge3.unwrap().borrow().out_vertex.borrow().get_id(), &3);

        assert!(graph.get_vertex(&2).unwrap().borrow().edges.get(&1).is_some());
        assert!(graph.get_vertex(&2).unwrap().borrow().edges.get(&3).is_some());
        assert!(graph.get_vertex(&3).unwrap().borrow().edges.get(&2).is_some());
        assert!(graph.get_vertex(&1).unwrap().borrow().edges.get(&3).is_none());
        assert!(graph.get_vertex(&3).unwrap().borrow().edges.get(&1).is_none());
    }
}