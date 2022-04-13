use std::env;
use std::process;
// use std::error::Error;
//use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
//use std::collections::{HashMap};
use std::collections::BTreeMap;
//use rand::Rng;
//use std::rc::Rc;
//use std::cell::RefCell;


#[derive(Debug, Clone)]
struct Vertex {
	vertex_id: u32,
	incoming: BTreeMap<u32,u32>,
	incoming_cnt: usize,
	outgoing: BTreeMap<u32,u32>,
	outgoing_cnt: usize,
}

impl Vertex {

	pub fn new(id : &u32) -> Vertex {
		let incoming = BTreeMap::<u32,u32>::new();
		let outgoing = BTreeMap::<u32,u32>::new();
		Vertex {vertex_id: id.clone(), 
				incoming: incoming, 
				outgoing: outgoing,
				incoming_cnt : 0,
				outgoing_cnt : 0,
				}
	}
	
	pub fn add_outgoing(&mut self, vertex_id: u32) {
		let counter = self.outgoing.entry(vertex_id).or_insert(0);
		*counter += 1;
		self.outgoing_cnt += 1;
	}

	pub fn del_outgoing (&mut self, vertex_id: u32) ->  Result <(), String> {

		match self.outgoing.get_mut(&vertex_id) {
			None | Some(0)  => Err("Invalid Vertex".to_string()),
			Some(1)        =>  	{ 	
									self.outgoing.remove(&vertex_id); 
									self.outgoing_cnt -= 1;
									Ok(())
								}, 
			Some(x)        => 	{	*x -=1;  
								 	self.outgoing_cnt -= 1;
								 	Ok(())
								},
		}
	}

	pub fn add_incoming(&mut self, vertex_id: u32) {
		let counter = self.incoming.entry(vertex_id).or_insert(0);
		*counter += 1;
		self.incoming_cnt += 1;
	}

	pub fn del_incoming (&mut self, vertex_id: u32) -> Result<(),String> {
	
		match self.incoming.get_mut(&vertex_id) {
			None | Some(0)  => Err("Invalid Vertex".to_string()),
			Some(1)        =>	{ 
									self.incoming.remove(&vertex_id); 
									self.incoming_cnt -= 1;
									Ok(())
								}, 
			Some(x)        => 	{
									*x -=1;
									self.incoming_cnt -= 1;
									Ok(())
								},
		}

	}
}


#[derive(Debug,Clone)]
struct Graph {
	vertex_map:  BTreeMap::<u32, Vertex>,
}


impl Graph {
	pub fn new() -> Graph {
		let v_map = BTreeMap::<u32, Vertex>::new();
		Graph {
				vertex_map: v_map,
		}
	}


	pub fn get_outgoing(&self, vertex: u32) -> Vec<u32>{
		let v = self.vertex_map.get(&vertex).unwrap();
		v.outgoing.keys().cloned().collect()
		
	}

	pub fn get_incoming(&self,vertex: u32) -> Vec<u32> {
		let v = self.vertex_map.get(&vertex).unwrap();
		v.incoming.keys().cloned().collect()
		
	}

	
	pub fn delete_edge(&mut self,v1 : u32, v2 : u32) -> Result<(),String>  {
	
		self.vertex_map.get_mut(&v1).unwrap().del_outgoing(v2)?	;
		self.vertex_map.get_mut(&v2).unwrap().del_incoming(v1)?;
		Ok(())

	}


	pub fn get_vertexes(&self) -> Vec<u32> {
		self.vertex_map.keys().cloned().collect()
			
	}

	pub fn print_vertexes(&self) {
		for (key, value) in &self.vertex_map {
			let out_list : String = value.outgoing.iter().map(|(x, y)| format!("{} {}",x,y)).collect();
			println!("Vertex {} ({}) :  {}",key,value.vertex_id,out_list);
		}
					
	}

	pub fn create_vertex(&mut self,id: &u32) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
			let v = Vertex::new(&id);
			self.vertex_map.insert(id.clone(),v.clone());
			Some(self.vertex_map.len())  
		}
		
	}


	pub fn add_edge(&mut self, v1: u32, v2: u32) -> Option<usize> {

		//create the vertexes, if the don't exist
		self.create_vertex(&v1);
		self.create_vertex(&v2);

		let v_map = &mut self.vertex_map;
		// add the edge to the first vertex's adjanceny list
		let vert = v_map.get_mut(&v1).unwrap(); 
		vert.add_outgoing(v2);
		let new_cnt = vert.outgoing_cnt.clone();

		// add the edge to the second vertex adjacentcy list
		let vert2 = v_map.get_mut(&v2).unwrap(); 
		vert2.add_incoming(v1);

		Some(new_cnt)

	}
}



fn main() {


    let args: Vec<String> = env::args().collect();

	println!("Args {:?} {}",args,args.len());

	if args.len() < 2 { eprintln!("Usage: {} filename <count>", args[0]); process::exit(1); }
	let mut attempts : u32 = 1;
	if args.len() > 2 {
		attempts = args[2].parse().unwrap();
	}
	println!("Attempting mincut {} times",attempts);


  // Create a path to the desired file
    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

	let mut g = Graph::new();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
		let mut tokens = line_data.split_whitespace();
		let vertex = tokens.next().unwrap().parse::<u32>().unwrap();
		let adjacent : Vec<u32> = tokens.map(|x| x.to_string().parse::<u32>().unwrap()).collect();

		g.create_vertex(&vertex);
		for other_v in &adjacent {
			let _num_edges = g.add_edge(vertex,*other_v);
		}
    }
	println!("Read {} lines",_count);

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

	fn setup_basic1() -> Graph {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.add_edge(2,4),Some(2));
		assert_eq!(g.add_edge(3,4),Some(1));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_outgoing(1),&[2]);
		assert_eq!(g.get_incoming(2),&[1]);
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_incoming(2),&[1]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.add_edge(1,2),Some(3));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[3]);
		
	}


}
