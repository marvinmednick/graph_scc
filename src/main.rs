use std::env;
use std::process;
use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap};
use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug,Clone)]
struct Edge {
	edge_id: String,
	vertex1: u32,
	vertex2: u32,	
	count: u32,
	edge_list_indexes: Vec<usize>,
}


impl Edge {

	pub fn new(id : &String, v1: u32, v2: u32) -> Edge {
		Edge {
			edge_id: id.clone(),
			vertex1: v1,
			vertex2: v2,
			count: 1,
			edge_list_indexes: Vec::<usize>::new(),
		}
	}

	pub fn incr_cnt(&mut self) {
		self.count += 1;
	}

	pub fn count(&self) -> u32 {
		self.count
	}
}

#[derive(Debug, Clone)]
struct Vertex {
	vertex_id: u32,
	adjacent: Vec<u32>
}

impl Vertex {

	pub fn new(id : &u32) -> Vertex {
		let adjacent = Vec::<u32>::new();
		Vertex {vertex_id: id.clone(), adjacent: adjacent}
	}
	
	pub fn add_adjacent(&mut self, vertex_id: u32) {
		self.adjacent.push(vertex_id);
	}

}



#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum GraphType {
	Undirected,
	Directed

}

#[derive(Debug,Clone)]
struct Graph {
	graph_type: GraphType,
	pub vertex_list: Vec<u32>,
	pub vlist:  Vec::<Rc::<RefCell<Vertex>>>,
	pub edge_list:Vec<String>,
	vertex_map:  HashMap::<u32, Vertex>,
	edge_map:   HashMap::<String, Edge>,
	vmap:   HashMap::<u32,Rc<RefCell<Vertex>>>,
}


impl Graph {
	pub fn new(gtype: GraphType) -> Graph {
		let v_list = Vec::<u32>::new();
		let v_rclist = Vec::<Rc::<RefCell<Vertex>>>::new();
		let e_list = Vec::<String>::new();
		let v_map = HashMap::<u32, Vertex>::new();
		let v_rcmap = HashMap::<u32,Rc<RefCell<Vertex>>>::new();
		let e_map = HashMap::<String, Edge>::new();
		Graph {
				graph_type:  gtype,
				vertex_list : v_list,
				vlist :   v_rclist,
				edge_list:  e_list,
				vertex_map: v_map,
				vmap: v_rcmap,
				edge_map:  e_map,
		}
	}


	// --------------------------------------
	// mincut

	pub fn mincut(&mut self) -> usize {

		let mut rng = rand::thread_rng();

		while self.vertex_list.len() > 2 {
			let num_edges = self.edge_list.len();
			let selected_edge_idx = rng.gen_range(0..num_edges);
			let selected_edge = self.edge_list[selected_edge_idx].clone();
	//		println!("collapsing {} {:?}",selected_edge_idx,selected_edge);
			self.collapse_edge(selected_edge);
//			self.print_vertexes();
//			self.print_edges();
		}
		self.edge_list.len()
	}

	pub fn get_adjacent(&self, vertex: u32) -> &[u32]{
		let v = self.vertex_map.get(&vertex).unwrap();
		&v.adjacent[..]
		
	}

	pub fn get_edge_count(&self, v1: u32, v2: u32) -> u32 {
        let edge_name = self.edgename(v1,v2);
        if let Some(edge) = self.edge_map.get(&edge_name) {
			edge.count
		}
		else {
			0
		}
	}

	pub fn collapse_edge(&mut self, edge_name: String) {


			if let Some(edge) = self.edge_map.get(&edge_name) {
				// get vertexes from Edge
				let v1_id = self.vertex_map.get(&edge.vertex1).unwrap().vertex_id.clone();
				let v2 = self.vertex_map.get(&edge.vertex2).unwrap().clone();
				let v2_id = self.vertex_map.get(&edge.vertex2).unwrap().vertex_id.clone();
				//println!("collapse edge {} between {} and {}",edge_name,v1_id,v2_id);
				// v1 is kept
				// v2 is mergeed in

		
				//println!("v2 {:?}",v2);

				for node in v2.adjacent.iter() {
					let adj_id = node.clone();
					let _old_adj_edge_name = &self.edgename(v2_id,adj_id);
					let _new_adj_edge_name = &self.edgename(v1_id,adj_id);
				//	println!("processing adj {} old name {} new name {}",node,_old_adj_edge_name, _new_adj_edge_name);

					
					let result =  self.delete_edge_instance(v2_id,adj_id);
					if result.is_err() {
						panic!("error removing edge {} {} {}", v2_id,adj_id,result.unwrap());
					}
					if v1_id != adj_id {
						self.add_edge(v1_id,adj_id);

					}
				}

				// 	delete v2 from the vertex List map
				// and the vertex list
				self.vertex_map.remove(&v2_id);
				if let Some(idx) = self.vertex_list.iter().position(|value| *value == v2_id) {
					self.vertex_list.swap_remove(idx);
				}

			}

	}


	// Deletes a single instance of an edge
	// which removes the edge if instance count is 0
	pub fn delete_edge_instance(&mut self, v1 : u32, v2: u32) -> Result<bool,&'static str> {
        let edge_name = self.edgename(v1,v2);
        if let Some(edge) = self.edge_map.get(&edge_name) {
			let last_entry = edge.edge_list_indexes.len()-1;
			let list_index = edge.edge_list_indexes[last_entry];
			self.delete_edge_by_index(list_index)
        }
		else {
			println!("No such edge {}",edge_name);
			Err("No such edge")
		}

	}

	// fully remove all instances of the edge
	pub fn remove_edge(&mut self, v1 : u32, v2: u32) -> Result<bool, &'static str> {
        let edge_name = self.edgename(v1,v2);
		if !self.edge_map.contains_key(&edge_name) {
			println!("No such edge {}",edge_name);
			return Err("No such edge");
		}
//		println!("Edge list {:?}",self.edge_list);
		while let Some(edge) = self.edge_map.get(&edge_name) {
//			println!("Removed Edge {:?}",edge);
			let last_entry = edge.edge_list_indexes.len()-1;
			let list_index = edge.edge_list_indexes[last_entry];
		//	println!("last Entry {} list Index {}",last_entry, list_index);
			let result = self.delete_edge_by_index(list_index);
		//  let edge2 = self.edge_map.get(&edge_name);
		//	println!("Edge2 {:?}",edge2);
		//	println!("Edge list {:?}",self.edge_list);
			if result.is_err() {
				println!("error during remove edge {}",result.unwrap());
				return result;
			}
        }
		Ok(true)

	}

	pub fn delete_edge_by_index(&mut self,index: usize)  -> Result<bool,&'static str> {
		if index >= self.edge_list.len() {
			return Err("Index out of range");
		}

		// get the edge name from Vector by the index
		let edge_name = self.edge_list[index].clone();

		// get referece to the edge stucture itself bu lookup in the map
		let edge = self.edge_map.get(&edge_name).unwrap();
		let v1 = edge.vertex1.clone();
		let v2 = edge.vertex2.clone();

		// remove the edge from the edge list.


		// since this will use swap_remove to remove the item
		// the item to be deleted will be moved to the end of the
		// list and the last entry will be moved to its place,
		// which means the edge_list_indexes for that entry need to be updated
		let last_entry = self.edge_list.len() -1;
		if index != last_entry  {
			// get the current last entry
			let swap_entry = self.edge_list[last_entry].clone();
			let _old_entry = self.edge_list.swap_remove(index);

			let swap_edge = self.edge_map.get_mut(&swap_entry).unwrap();

			// search for the matching entry in the edge so that it an 
			// be updated with its new index
			for idx in swap_edge.edge_list_indexes.iter_mut().rev() {
				if *idx == last_entry {
					*idx = index;
					break;
				}
			}
		} 
		else {
			// the edge to delete from the lsit if the last entry, so it can just be removed
			let _old_entry = self.edge_list.swap_remove(index);

		}

		// remove the entry from the edges list of indexes in the edge_list
		let edge = self.edge_map.get_mut(&edge_name).unwrap();
		if let Some(idx) = edge.edge_list_indexes.iter().position(|value| *value == index) {
			edge.edge_list_indexes.swap_remove(idx);
		}
		

		// remove the references from the adjacney lists
		// first v1's list
		
		let mut index = 0;
		let  vertex1 = self.vertex_map.get_mut(&v1).unwrap();
//		println!("Updating v1 ({}) adj - removing v2 {} {:?}",v1,v2,vertex1.adjacent);
		for v in vertex1.adjacent.iter() {
			if *v == v2 {
				break;
			} 
			else {
				index +=1
			}
		}
//		println!("removing at index {}",index);
		// remove this item from the list
		vertex1.adjacent.swap_remove(index);


		// then search through v2's list to 
		// remove the reference to v1
		
		let mut index = 0;
		let 	vertex2 = self.vertex_map.get_mut(&v2).unwrap();
//		println!("Updating v2 ({}) adj - removing v1 {} {:?}",v2,v1,vertex2.adjacent);
		for v in vertex2.adjacent.iter() {
			if *v == v1 {
				break;
			} 
			else {
				index +=1
			}
		}
//		println!("removing at index {}",index);
		// remove this item from the list
		// remove this item from the list
		vertex2.adjacent.swap_remove(index);

		// get a new referece to the original edge to remove it
		let edge = self.edge_map.get_mut(&edge_name).unwrap();
		// reduce the edge count
		edge.count -= 1;
		if edge.count <= 0 {
			// if the count is 0, thee remove the edge from the map
			self.edge_map.remove(&edge_name);
		}
		Ok(true)

	}


	pub fn print_vertexes(&self) {
		for (key, value) in &self.vertex_map {
			let adj_list : String = value.adjacent.iter().map(|x| format!("{} ",x)).collect();
			println!("Vertex {} ({}) :  {}",key,value.vertex_id,adj_list);
		}
					
	}

	pub fn print_edges(&self) {
		for (key, value) in &self.edge_map {
			println!("Edge {} : id {}  v1 {} v2 {} cnt {} {:?}",key, value.edge_id, value.vertex1,value.vertex2,value.count,value.edge_list_indexes);
		}
		let mut count = 0;
		for edge in &self.edge_list {
			println!("index {} - {}",count,edge);
			count += 1;
		}
					
	}

//	let mut pt_refcell = RefCell::new(point1);
//	let rc_refcell = Rc::new(pt_refcell);
//	let mut pt_vec = Vec::<Rc::<RefCell<Point>>>::new();

	pub fn create_vertex(&mut self,id: &u32) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
			let v = Vertex::new(&id);
			self.vertex_map.insert(id.clone(),v.clone());
			self.vertex_list.push(id.clone());
			let vtx = Rc::new(RefCell::new(v));
			self.vlist.push(vtx.clone());
			self.vmap.insert(id.clone(),vtx.clone());
			Some(self.vertex_list.len())
		}
		
	}

	pub fn edgename(&self, v1: u32, v2: u32) -> String {
		if self.graph_type == GraphType::Directed {
			format!("{}_{}",v1,v2).to_string()
		}
		else {
			let start = cmp::min(v1,v2);
			let end = cmp::max(v1,v2);
			format!("{}_{}",start,end).to_string()
		}
	}

	pub fn get_edge(&self, v1 : u32, v2: u32) -> Option<&Edge> {
		let edge_name = self.edgename(v1,v2);
		self.edge_map.get(&edge_name)
	}

	pub fn edge_exists(&mut self, edge_name : &String) -> bool {
		self.edge_map.contains_key(edge_name)
	}

	pub fn create_edge(&mut self, v1: u32, v2: u32) -> Option<usize> {
		let edge_name = self.edgename(v1,v2);
		if self.edge_exists(&edge_name) {
			None
		}
		else {
			self.add_edge(v1,v2)
		}

	}

	pub fn add_edge(&mut self, v1: u32, v2: u32) -> Option<usize> {

		//get the edgename
		let edge_name = self.edgename(v1,v2);

		//create the vertexes, if the don't exist
		self.create_vertex(&v1);
		self.create_vertex(&v2);

		
		if self.edge_exists(&edge_name) {
			
			// know what edge exists, since we just checked
			let edge = self.edge_map.get_mut(&edge_name).unwrap();
			
			edge.incr_cnt();
		}
		else {
			let new_edge = Edge::new(&edge_name,v1,v2);

			// insert the edge into the map by name
			self.edge_map.insert(edge_name.clone(),new_edge);		

		}

		let v_map = &mut self.vertex_map;
		// add the edge to the first vertex's adjanceny list
		let mut vert = v_map.get_mut(&v1); 
		vert.unwrap().add_adjacent(v2);

		// add the edge to the second vertex adjacentcy list
		vert = v_map.get_mut(&v2); 
		vert.unwrap().add_adjacent(v1);

		// add the edge to edge list
		self.edge_list.push(edge_name.clone());
		let edge = self.edge_map.get_mut(&edge_name).unwrap();
		edge.edge_list_indexes.push(self.edge_list.len()-1);
			
		Some(self.edge_list.len())

	}
}



fn main() {


    let args: Vec<String> = env::args().collect();

	println!("Args {:?} {}",args,args.len());

	if args.len() < 2 {
        eprintln!("Usage: {} filename <count>", args[0]);
        process::exit(1);
	}
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

	let mut g = Graph::new(GraphType::Undirected);

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
		let mut tokens = line_data.split_whitespace();
		let vertex = tokens.next().unwrap().parse::<u32>().unwrap();
		let adjacent : Vec<u32> = tokens.map(|x| x.to_string().parse::<u32>().unwrap()).collect();

		g.create_vertex(&vertex);
		for other_v in &adjacent {
			let _num_edges = g.create_edge(vertex,*other_v);
		}
    }
	println!("Read {} lines",_count);
	// start the 'min' with the total number of edges... 
	let mut min_min_cuts = g.edge_list.len();
	let mut count = 0;
	while count < attempts {
		count += 1;
		let mut working_g = g.clone();
		let result = working_g.mincut();
		if result < min_min_cuts {
			min_min_cuts = result;
		}
		println!("Attempt {} Mincut is {} (min mincut is {})",count,result,min_min_cuts);
	}
	//g.print_edges();

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
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.create_edge(1,2),Some(1));
		assert_eq!(g.create_edge(1,3),Some(2));
		assert_eq!(g.create_edge(2,3),Some(3));
		assert_eq!(g.create_edge(2,4),Some(4));
		assert_eq!(g.create_edge(3,4),Some(5));
		assert_eq!(g.get_adjacent(1),&[2,3]);
		assert_eq!(g.get_adjacent(2),&[1,3,4]);
		assert_eq!(g.get_adjacent(3),&[1,2,4]);
		assert_eq!(g.get_adjacent(4),&[2,3]);
		assert_eq!(g.edge_list,vec!("1_2".to_string(),
									"1_3".to_string(),
									"2_3".to_string(),
									"2_4".to_string(),
									"3_4".to_string(),
					));
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.create_edge(1,2),Some(1));
		assert_eq!(g.vertex_list,vec!(1,2));
		assert_eq!(g.edge_list,vec!("1_2".to_string()));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.create_edge(1,3),Some(2));
		assert_eq!(g.create_edge(2,3),Some(3));
		assert_eq!(g.vertex_list,vec!(1,2,3));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),"1_3".to_string(),"2_3".to_string()));
		assert_eq!(g.create_edge(1,4),Some(4));
		assert_eq!(g.vertex_list,vec!(1,2,3,4));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),"1_3".to_string(),"2_3".to_string(),"1_4".to_string()));
		println!("{:?}",g);

    }

	#[test]
	fn name() {
		let g = Graph::new(GraphType::Undirected);
		assert_eq!(g.edgename(1,2),"1_2".to_string()); 
		assert_eq!(g.edgename(3,2),"2_3".to_string()); 
		assert_eq!(g.edgename(10,10),"10_10".to_string()); 
	}

	#[test]
	fn test_add() {
		let mut g = Graph::new(GraphType::Undirected);
		assert_eq!(g.create_edge(1,2),Some(1));
		assert_eq!(g.get_adjacent(1),&[2]);
		assert_eq!(g.get_adjacent(2),&[1]);
		assert!(g.get_edge(2,3).is_none());
		assert_eq!(g.add_edge(1,2),Some(2));
		assert_eq!(g.get_adjacent(1),&[2,2]);
		assert_eq!(g.get_adjacent(2),&[1,1]);
		assert_eq!(g.get_edge(1,2).unwrap().count(),2);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.add_edge(1,2),Some(6));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),
									"1_3".to_string(),
									"2_3".to_string(),
									"2_4".to_string(),
									"3_4".to_string(),
									"1_2".to_string(),
				));
		assert_eq!(g.get_adjacent(1),&[2,3,2]);
		assert_eq!(g.get_adjacent(2),&[1,3,4,1]);
		assert_eq!(g.get_adjacent(3),&[1,2,4]);
		assert_eq!(g.delete_edge_by_index(1),Ok(true));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),
									"1_2".to_string(),
									"2_3".to_string(),
									"2_4".to_string(),
									"3_4".to_string(),
				));
		assert_eq!(g.get_edge_count(1,2),2);
		assert_eq!(g.get_adjacent(1),&[2,2]);
		assert_eq!(g.get_adjacent(3),&[4,2]);
		assert_eq!(g.delete_edge_instance(1,2),Ok(true));
		assert_eq!(g.get_edge_count(1,2),1);
		assert_eq!(g.get_adjacent(1),&[2]);
		assert_eq!(g.edge_list,vec!("1_2".to_string(),
									"3_4".to_string(),
									"2_3".to_string(),
									"2_4".to_string(),
				));
		assert_eq!(g.delete_edge_instance(1,2),Ok(true));
		assert_eq!(g.get_edge_count(1,2),0);
		
	}

	#[test]
	fn test_remove () {
		let mut g = setup_basic1();
		assert_eq!(g.add_edge(1,2),Some(6));
		assert_eq!(g.edge_list,vec!("1_2".to_string(),
									"1_3".to_string(),
									"2_3".to_string(),
									"2_4".to_string(),
									"3_4".to_string(),
									"1_2".to_string(),
				));
		assert_eq!(g.get_edge_count(1,2),2);
		assert_eq!(g.remove_edge(1,2),Ok(true));
		assert_eq!(g.get_edge_count(1,2),0);
		assert_eq!(g.edge_list,vec!("3_4".to_string(),
									"1_3".to_string(),
									"2_3".to_string(),
									"2_4".to_string(),
				));
				
	}

	#[test]
	fn test_collapse() {
		let mut g = setup_basic1();
		assert_eq!(g.vertex_list.len(),4);
		println!("Before");
		g.print_edges();
		g.collapse_edge("1_2".to_string());
		assert_eq!(g.vertex_list.len(),3);
		println!("After");
		g.print_edges();
		assert_eq!(g.edge_list,vec!("3_4".to_string(),"1_3".to_string(),"1_3".to_string(),"1_4".to_string()));
		g.collapse_edge("1_3".to_string());
		assert_eq!(g.vertex_list.len(),2);
		g.print_edges();
		assert_eq!(g.edge_list,vec!("1_4".to_string(),"1_4".to_string()));
		g.print_vertexes();
	
	}

	#[test]
	fn test_mincut() {
		let mut g = setup_basic1();
		g.mincut();
	}


}
