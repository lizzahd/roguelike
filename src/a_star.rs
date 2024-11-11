use macroquad::prelude::*;

use crate::primitives::*;
use crate::level::*;

struct Node<'a> {
	pub pos: Vec2,
	pub parent: Option<&'a Self>,
	pub dist_from_start: f32,
	pub dist_from_target: f32,
	pub cost: f32,
}

impl<'a> Node<'a> {
	pub fn new(pos: Vec2, parent: Option<&'a Node>) -> Self {
		Self {
			pos,
			parent,
			dist_from_start: if let Some(p) = parent {
				p.dist_from_start + 1.
			} else {
				0.
			},
			dist_from_target: 0.,
			cost: 0.,
		}
	}
}

impl PartialEq for Node<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.pos == other.pos
	}
}

fn get_path_from_node(node: &Node) -> Vec<Vec2> {
	let mut path = Vec::<Vec2>::new();

	let mut next_node = node;
	loop {
		path.push(next_node.pos);
		if let Some(p) = next_node.parent {
			next_node = p;
		} else {
			break;
		}
	}

	path
}

fn is_invalid_pos(chunk: &Chunk, node: &Node) -> bool {
	if chunk.colliders.iter().position(|w: &Wall| w.rect.point() == node.pos).is_some() {
		return true;
	}

	false
}

fn get_sqr_dist(first: Vec2, second: Vec2) -> f32 {
	let sqr_x_dist = (first.x - second.x).powf(2.);
	let sqr_y_dist = (first.y - second.y).powf(2.);

	sqr_x_dist + sqr_y_dist
}

pub fn astar(chunk: &Chunk, start_pos: Vec2, end_pos: Vec2) -> Vec<Vec2> {
	let start_node = Node::new(start_pos, None);
	let end_node = Node::new(end_pos, None);

	let mut nodes = vec![start_node];
	let mut visited = Vec::<Vec2>::new();

	let dist = start_pos.distance(end_pos);

	while nodes.len() > 0 {
		if nodes.len() as f32 > dist * 10. {
			return Vec::new();
		}

		let mut current_index = 0;
		for i in 0..nodes.len() {
			if nodes[i].cost < nodes[current_index].cost {
				current_index = i;
			}
		}

		visited.push(nodes[current_index].pos);

		if nodes[current_index] == end_node {
			return get_path_from_node(&nodes[current_index])[1..].to_vec();
		}

		'adj_loop: for new_pos in adj_8_t() {
			let node_pos = nodes[current_index].pos + new_pos;
			let dist_from_start = nodes[current_index].dist_from_start + 1.;
			let dist_from_target = get_sqr_dist(node_pos, end_node.pos);
			let parent = &(nodes[current_index]) as *const Node;

			unsafe {
				nodes.push(Node {
					pos: node_pos,
					parent: Some(&*parent as &Node),
					dist_from_start,
					dist_from_target,
					cost: dist_from_start + dist_from_target,
				});
			}
			let new_node = nodes.last().unwrap();

			if is_invalid_pos(chunk, &new_node) {
				continue;
			}

			if visited.contains(&new_node.pos) {
				continue;
			}

			for node in &nodes {
				if *new_node == *node && new_node.dist_from_start > node.dist_from_start {
					continue 'adj_loop;
				}
			}
		}
	}

	Vec::new()
}

pub fn shite_step(chunk: &Chunk, start_pos: Vec2, end_pos: Vec2) -> Option<Vec2> {
	let mut shortest = Option::<Vec2>::None;
	let mut shortest_dist = Option::<f32>::None;

	for adj_pos in adj_8_t() {
		let new_pos = adj_pos + start_pos;
		let new_dist = new_pos.distance(end_pos);
		
		if chunk.colliders.iter().position(|w: &Wall| w.rect.point() == new_pos).is_some() {
			continue;
		}

		if let Some(dist) = shortest_dist {
			if new_dist < dist {
				shortest = Some(new_pos);
				shortest_dist = Some(new_dist);
			}			
		} else {
			shortest = Some(new_pos);
			shortest_dist = Some(new_dist);
		}
	}

	shortest
}