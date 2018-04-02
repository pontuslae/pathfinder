/*
    Store all nodes within a file.
        -> inside a subdirectory
       (->) If to many nodes exist use a secondary file.


    ( Node generation should be a secondary project, as they are too complex to hand write.)
    A node contains:
        -> name: &str
        -> connecting nodes
            -> distance to them
            -> ways of transportation with times.
       (->) absolute x and y positions.



    It will use multiple algorithms to find the shortest and fastest path using whatever
    transportation the user has requested. (any transportation by default)
-
    Djikstras, A-pointer
        (->) including alternative paths.

    The user will get a summery of the trip,
    The user can request a generated map of traveling between the nodes to the destination.

 */

extern crate pathfinder;
extern crate rand;
extern crate image;
use pathfinder::{map, data, group};
use std::env;
use pathfinder::node::figure;
use pathfinder::node::link::Link;
use pathfinder::node::coordinates::Coordinate;
use pathfinder::group::Group;
use pathfinder::map::*;
use pathfinder::node::*;
use rand::thread_rng;

use image::Rgba;

fn main() {

    //pathfinder::map::network::create_random_network(2, 70);

    let mut nodes: Vec<Node> = Vec::new();
    let mut links = Vec::new();

    for i in 0..33   {
        let dif = 10;
        let start = Coordinate::new(-(dif/2)*i,-(dif/2)*i);
        let mut new_nodes = figure::rectangle(&start, dif*i, dif*i);
        nodes.append(&mut new_nodes)
    }

    let start = Coordinate::new(0,0);
    nodes = figure::rectangle(&start, 50, 50);

    for (i, item) in nodes.iter().enumerate() {
        for (j, item2) in nodes.iter().enumerate() {
            if i == j {continue};

            links.push(
                Link::new(&item.geo, &item2.geo)
            );

        }
    }
}


