//! Connects two nodes and returns a PNG image.

extern crate pathfinder;

use pathfinder::map::*;
use pathfinder::node::*;
use pathfinder::node::coordinates::Coordinate;

fn main() {
    let string = String::new();
    let ls = 35; // Letter spacing.
    let hs = ls/2; // Letter spacing half.

    // This is a "small" list of positions required to make a figure!
    let node_pos: Vec<(i16, i16)> = vec!(
        (0,0), (hs, -hs), (0, -ls), (0, ls),
        (ls, -hs), (ls+hs, ls), (hs, 0), (ls*2 +hs, 0),
        (ls*2, -hs), (ls*2, ls), (ls*2 +hs, ls),
        (ls*3, -hs), (ls*3, ls), (ls*3, hs), (ls*3 +hs, hs), (ls*3 +hs, ls),
        (ls*4, ls), (ls*4, -hs), (ls*4 +hs, -hs), (ls*4, -hs), (ls*4, 0), (ls*4 +hs, 0),
        (ls*5 -hs/2, hs/2), (ls*5 -hs/4, hs/2), (ls*5, hs/2), (ls*5, ls), (ls*5 +hs/2, ls),
        (ls*6 -hs, 0), (ls*6, ls), (ls*6 +hs/3, 0),
        (ls*7 -hs, -hs), (ls*7 -hs, ls), (ls*7, ls -hs), (ls*7 -hs, hs/2),
        (ls*8, 0), (ls*8 -hs, -hs), (ls*8 -ls, 0), (ls*8 -hs, ls), (ls*8 +hs, ls),
        (ls*9 -hs, -hs/3), (ls*9 -hs, 0), (ls*9, 0), (ls*9, hs/3)
    );
    let mut node_vec = Vec::new();

    // Add each position as a node.
    for pos in node_pos.iter() {
        let mut node = Node::new(string.clone(), coordinates::Coordinate::new(pos.0,pos.1));
        node.set_radius(5);
        node_vec.push(node);
    }

    // Link them sequentially in order.
    let link_vec = sequentially_link_nodes(&node_vec);

    // Create the image using the resource nodes and links.
    node_and_links(&node_vec, &link_vec);
}