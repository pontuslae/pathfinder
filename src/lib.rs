#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate image;
extern crate rand;
extern crate gif;
extern crate pythagoras;

pub mod node;
pub mod map;
pub mod tools;
pub mod group;
pub mod data;

mod tests;

/// Holds a position used for Nodes and Groups.
#[derive(Debug, Eq, Copy, Clone)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

/// A positioned object that can be drawn on an image::ImageBuffer.
#[derive(Clone, Debug)]
pub struct Node<'a, T: Shape> {
    pub hash: u64,
    pub geo: Coordinate,
    pub color: image::Rgba<u8>,
    pub radius: Option<u32>,
    pub connections: Vec<Link<'a>>,
    shape: T
}

/// Holds a set of nodes and applies properties to all child nodes when drawn.
/// The group itself has no displayed output and is not visible.
#[derive(Clone, Debug)]
pub struct Group<'a, 'b, T: Shape> {
    pub settings: Node<'b, T>,
    pub nodes: Vec<Node<'a, T>>,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub add: (i16, i16),
    pub size: u32,
}

/// Connects two Coordinate points.
#[derive(Clone, Debug)]
pub struct Link<'a> {
    pub to: &'a Coordinate,
    pub color: image::Rgba<u8>,
}

#[derive(Clone, Debug)]
pub struct Network<T: Draw + Hash> {
    pub elements: Vec<T>,
}

// ------------------------------------------------------------------

pub trait Shape {
    fn new() -> Self;
    fn area(&self, size: u32) -> Vec<Coordinate>;
}

pub trait Hash {
    fn get_hash(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct Square {}

#[derive(Debug, Clone)]
pub struct Circle {}

#[derive(Debug, Clone)]
pub struct Triangle {}

// ------------------------------------------------------------------


pub trait Draw {
    fn draw(&self, image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) ->
    image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
    fn get_size(&self) -> u32;
    fn get_coordinate(&self) -> &Coordinate;
}

impl<'a, T: Shape> Draw for Node<'a, T> {
    fn draw(&self, mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) ->
    image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let x = self.geo.x +x_offset as i16;
        let y = self.geo.y +y_offset as i16;
        let size = match self.radius {
            Some(_) => self.radius.unwrap(),
            None => size
        };

        for link in self.connections.iter() {
            image = link.draw(image, x_offset, y_offset, size, self.geo.clone());
        }

        for offset in self.shape.area(size) {
            image.put_pixel((x +offset.x) as u32, (y +offset.y) as u32, self.color);
        }
        image
    }
    fn get_size(&self) -> u32 {
        match self.radius.is_none() {
            true => 4,
            false => self.radius.unwrap(),
        }
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.geo
    }
}

impl<'a, 'b, T: Shape> Draw for Group<'a, 'b, T> {
    /// Draws the Nodes inside that Group. If none the Group is draw as blank.
    fn draw(&self, mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) ->
    image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        for node in self.nodes.iter() {
            image = node.draw(image, x_offset, y_offset, size);
        }
        image
    }

    // Returns the largest node that exists within the group.
    fn get_size(&self) -> u32 {
        let mut max = 0;
        for node in self.nodes.iter() {
            let tmp = node.get_size();
            if tmp > max {
                max = tmp;
            }
        }
        match self.settings.radius {
            Some(e) => max + e,
            None => max,
        }
    }
    fn get_coordinate(&self) -> &Coordinate {
        self.settings.get_coordinate()
    }
}

impl<'a> Link<'a> {
    /// Draws the connection using either a modified version of Bresham's line algorithm or a generic one.
    fn draw(&self,
            mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
            x_offset: i16, y_offset: i16,
            size: u32,
            from: Coordinate
    ) ->
            image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let x_offset = x_offset + (size/2) as i16;
        let y_offset = y_offset + (size/2) as i16;

        let a = Coordinate::new(
            from.x +x_offset,
            from.y +y_offset
        );
        let b = Coordinate::new(
            self.to.x +x_offset,
            self.to.y +y_offset
        );

        tools::plot(&a, &b).iter().map(|c|
            image.put_pixel( c.x  as u32, c.y as u32, self.color)
        ).collect::<Vec<_>>();
        image
    }
    fn get_size(&self) -> u32 {
        1
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.to
    }
}

// ------------------------------------------------------------------

impl Shape for Square {
    fn new() -> Square {
        Square {}
    }

    /// Returns all coordinates that the shape occupies.
    /// Assume that you start at coordinate x: 0, y: 0.
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        for i in 0..size {
            for j in 0..size {
                vec.push(Coordinate::new(i as i16, j as i16));
            }
        }
        vec
    }
}

impl Shape for Circle {
    fn new() -> Circle {
        Circle {}
    }

    /// Returns all coordinates that the shape occupies.
    /// Algorithm is derived from: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();

        let mut x: i16 = (size-1) as i16;
        let mut y: i16 = 0;
        let mut dx: i16 = 1;
        let mut dy: i16 = 1;
        let x0: i16 = 0;
        let y0: i16 = 0;
        let mut err: i16 = dx - (size << 1) as i16;

        while x >= y {
            vec.append(&mut tools::plot(&Coordinate::new(x0 + x, y0 + y), &Coordinate::new(x0 - x, y0 + y)));
            vec.append(&mut tools::plot(&Coordinate::new(x0 + x, y0 - y), &Coordinate::new(x0 - x, y0 - y)));
            vec.append(&mut tools::plot(&Coordinate::new(x0 - y, y0 - x), &Coordinate::new(x0 - y, y0 + x)));
            vec.append(&mut tools::plot(&Coordinate::new(x0 + y, y0 - x), &Coordinate::new(x0 + y, y0 + x)));

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            } else {
                x -= 1;
                dx += 2;
                err += dx - (size << 1) as i16;
            }
        }

        vec
    }
}

impl Shape for Triangle {
    fn new() -> Triangle {
        Triangle {}
    }

    /// Returns all coordinates that the shape occupies.
    /// Assume that you start at coordinate x: 0, y: 0.
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        let size = size as i16;
        let start_x = size/2;

        for i in 0..size {
            vec.append(&mut tools::plot(&Coordinate::new(start_x,0), &Coordinate::new(i, size)));
        }
        vec
    }
}

// ------------------------------------------------------------------

impl<'a, T: Shape> Hash for Node<'a, T> {
    fn get_hash(&self) -> u64 {
        self.hash
    }
}

impl<'a, 'b, T: Shape> Hash for Group<'a, 'b, T> {
    fn get_hash(&self) -> u64 {
        self.settings.get_hash()
    }
}

// ------------------------------------------------------------------


impl Coordinate {
    /// Constructs a Coordinate struct.
    pub fn new(x: i16, y: i16) -> Coordinate {
        Coordinate {
            x,
            y
        }
    }
}

impl<'a, T: Shape> Node<'a, T> {
    /// Constructs a Node struct.
    pub fn new(name: &str, geo: Coordinate) -> Node<'a, T> {
        Node {
            hash: data::calculate_hash(&name),
            geo,
            color: image::Rgba {data: [0,0,0,255]},
            radius: None,
            connections: Vec::new(),
            shape: T::new(),
        }
    }

    /// Links Node self to the provided node's coordinate.
    /// ```
    /// use pathfinder::{Node, Square, Coordinate};
    /// let nodeB: Node<Square> = Node::new("B", Coordinate::new(100,100));
    /// let mut nodeA: Node<Square> = Node::new("A", Coordinate::new(0,0));
    /// nodeA.link(&nodeB);
    /// assert_eq!(
    ///     nodeA.connections.get(0).unwrap().to,
    ///     &nodeB.geo);
    /// ```
    pub fn link<S: Shape>(&mut self, other: &'a Node<S>) {
        self.connections.push(Link::new(other.get_coordinate()));
    }

}

impl<'a, 'b, T: Shape> Group<'a, 'b, T> {
    /// Constructs a new Group
    pub fn new(name: &str, coordinates: Coordinate) -> Group<'a, 'b, T> {
        Group {
            settings: Node::new(name, coordinates),
            nodes: Vec::new(),
        }
    }

    /// Links together two groups.
    /// ```
    /// use pathfinder::{Group, Square, Coordinate};
    /// let groupB: Group<Square> = Group::new("B", Coordinate::new(100,100));
    /// let mut groupA: Group<Square> = Group::new("A", Coordinate::new(0,0));
    /// groupA.link(&groupB);
    /// assert_eq!(
    ///     groupA.settings.connections.get(0).unwrap().to,
    ///     &groupB.settings.geo);
    /// ```
    pub fn link<S: Shape>(&mut self, other: &'b Group<'a, 'b, S>) {
        self.settings.link(&other.settings);
    }
}

impl<'a> Link<'a> {
    /// Creates a new Link and binds two nodes together.
    pub fn new(to: &'a Coordinate) -> Link<'a> {
        Link {
            to,
            color: image::Rgba {data: [0,0,0,255]},
        }
    }
}

impl<T: Draw + Hash> Network<T> {
    pub fn new(elements: Vec<T>) -> Network<T> {
        Network {
            elements,
        }
    }
}

// ------------------------------------------------------------------

impl Coordinate {
    // Calculates the different in x and y of two Coordinates.
    pub fn diff(&self, other: &Coordinate) -> (i16, i16) {
        node::coordinates::diff(&self, other)
    }
}

impl<'a, 'b, T: Shape> Group<'a, 'b, T> {

    /// Returns the nodes that exists inside the Group.
    pub fn get_nodes(&self) -> &Vec<Node<T>> {
        &self.nodes
    }

    /// Adds a Node dynamically to the Group.
    pub fn new_node(&mut self, name: &str) {
        let geo = node::coordinates::gen_radius(&self.settings.geo, 0, self.get_dynamic_radius());
        self.new_node_inner(geo, name);
    }

    /// Adds a Node with a static distance from the center of the Group.
    pub fn new_node_min_auto(&mut self, name: &str, min: u32) -> &Node<T> {
        let geo = node::coordinates::gen_radius(&self.settings.geo, 0, min+5);
        self.new_node_inner(geo, name)
    }

    /// Adds a Node with a specific minimum and maximum distance from the center of the Group.
    pub fn new_node_min_max(&mut self, name: &str, min: u32, max: u32) -> &Node<T> {
        let geo = node::coordinates::gen_radius(&self.settings.geo, min, max);
        self.new_node_inner(geo, name)
    }

    /// Constructs a new node for the Group and mirrors the properties to it.
    pub fn new_node_inner(&mut self, geo: Coordinate, name: &str) -> &Node<T> {
        let mut node = Node::new(name,geo.clone());
        node.color = self.gen_color(geo);
        node.radius = self.settings.radius;
        self.push(node);
        &self.nodes.get(self.nodes.len() -1).unwrap()
    }

    /// Removes all non-essentials from the standard implementation.
    pub fn new_simple(x: i16, y: i16) -> Group<'a, 'b, T> {
        Group::new("", Coordinate::new(x, y))
    }

    /// Pushes a Node to the Group.
    pub fn push(&mut self, node: Node<'a, T>) {
        self.nodes.push(node);
    }

    /// Returns a dynamic radius based on the number of Nodes in the Group.
    pub fn get_dynamic_radius(&self) -> u32 {
        match self.settings.radius {
            Some(x) => x,
            None => 7 + self.nodes.len()as u32 /2,
        }
    }

    // Generates an image::Rgba based on the color of the Group and the distance from center.
    pub fn gen_color(&self, coordinates: Coordinate) -> image::Rgba<u8> {
        let radius = self.get_dynamic_radius() as i16;
        let (x_dif, y_dif) = self.settings.geo.diff(&coordinates);
        let x_scale: f64 = (x_dif as f64/radius as f64) as f64;
        let y_scale: f64 = (y_dif as f64/radius as f64) as f64;
        let c = self.settings.color.data;
        let max_multi: f64 = ((c[0] as i32 + c[1] as i32 + c[2] as i32)/3) as f64;
        let modify = (-max_multi*(x_scale+y_scale)/2.0) as i32;
        image::Rgba {data: [
            tools::border(c[0], modify),
            tools::border(c[1], modify),
            tools::border(c[2], modify),
            tools::border(c[3], 0)
        ]}
    }
}

impl Map {
    pub fn new() -> Map {
        Map {
            image: None,
            add: (0, 0),
            size: 4, // TODO set dynamically.
        }
    }

    /// Maps any struct that has implemented Draw, on to an ImageBuffer.
    /// ```
    /// use pathfinder::*;
    /// let nodes: Vec<Node<Square>> = vec!(
    ///     Node::new("1", Coordinate::new(0,0)),
    ///     Node::new("2", Coordinate::new(100,100))
    /// );
    /// // Add content to vectors.
    /// let mut map = Map::new();
    /// map = map.map(&nodes);
    /// ```
    pub fn map<T: Draw>(mut self, element: &[T]) -> Self {
        if self.image.is_none() {
            let min_max = map::min_max(&element);
            // Stabilizes the picture to have the action in the center of the image.
            // This functionality doesn't work that well for gif encoding.
            self.add = map::gen_stuff(min_max);
            let res = map::gen_map_dimensions(min_max);
            // Generates an image buffer.
            self.image = Some(map::gen_canvas(res.0, res.1));
            println!("{}x{} | {}x{} | {}", res.0, res.1, self.add.0, self.add.1, self.size);
        }

        for e in element {
            self.image = Some(e.draw(
                self.image.unwrap(),
                self.add.0,
                self.add.1,
                self.size,
            ));
        }
        self
    }
}

impl<'a, T: Hash + Draw + Clone> Network<T> {

    /// Calculates the path from node A to node B.
    /// ```
    /// use pathfinder::{Node, Coordinate, Network};
    /// let b = Node<Square>::new("B", Coordinate::new(20,20));
    /// let mut a = Node::<Square>new("A", Coordinate::new(0,0));
    /// a.link(&b);
    /// let network = Network::new(vec!(a, b));
    /// let path = network.path("A", "B", Network::path_shortest_leg);
    /// assert_eq!(path, vec!(a));
    /// ```
    pub fn path<'b>(&self, a: &str, b: &str, algorithm: &Fn(&Network<T>, &str, &str) -> Vec<Node<'b, Square>>) -> Vec<Node<'b, Square>> {
        let mut start: Node<Square> = Node::new(a, Coordinate::new(0,0));
        let goal: Node<Square> = Node::new(b, Coordinate::new(0,0));

        if !self.contains(&start) {
            panic!("starting node '{}' not in the network", a);
        }

        algorithm(&self, a, b)
    }

    /// Returns if the given hash exists in the network.
    pub fn contains<H: Hash>(&self, element: &H) -> bool {
        for elem in self.elements.iter() {
            if elem.get_hash() == element.get_hash() {
                return true;
            }
        }
        false
    }

    /// Returns the index of the element.
    pub fn contains_index<H: Hash>(&self, element: &H) -> Option<usize> {
        for (i, elem) in self.elements.iter().enumerate() {
            if elem.get_hash() == element.get_hash() {
                return Some(i);
            }
        }
        None
    }

    /// Retrieves an element given a &str.
    pub fn get_element(&self, id: &str) -> Option<T> {
        let mut tmp: Node<Square> = Node::new(id, Coordinate::new(0,0));
        let goal_index_opt = self.contains_index(&tmp);
        if goal_index_opt.is_none() {
            return None;
        }
        let goal_index = goal_index_opt.unwrap();
        let goal_opt: &T = self.elem.get(goal_index);
        if goal_opt.is_none() {
            return None;
        }
        goal_opt.unwrap()
    }

    /*
        TODO:
        Use any shape.
        Remove panics.
        Implement leg functionality.
        Efficiently do it.
        Simplify and remove dead code.
    */
    pub fn path_shortest_leg(network: &Network<T>, a: &str, b: &str) -> Vec<Node<'a, Square>> {
        let mut node_path: Vec<Node<Square>> = Vec::new();
        let mut m_elem: Vec<T>= network.elements.clone();

        let current: Node<Square> = network.get_element(a)
            .expect("goal does not exist in network");

        let goal: Node<Square> = network.get_element(b)
            .expect("goal does not exist in network");

        // TODO borrow might complain.
        node_path.push(current);
        // TODO remove current from m_elem.

        while current != goal {

            /* Sort the distance between the current node and all the other nodes it is connected too. */
            m_elem.sort_unstable_by(
                |a, b|
                    node::coordinates::distance(
                        a.get_coordinate(),
                        b.get_coordinate())
                        .cmp(&0) // TODO for djikstras, replace with stacked value.
            );

            if m_elem.is_empty() {
                panic!("Ran out of items before finding end.");
            }

            current = m_elem.remove(0);
            node_path.push(current);

            panic!("TODO");
        }

        node_path
    }

}

// ------------------------------------------------------------------


impl<'a> PartialEq for Link<'a> {
    fn eq(&self, other: &Link) -> bool {
        self.to == other.to
    }
}
