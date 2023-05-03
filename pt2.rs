use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use assert_approx_eq::assert_approx_eq;

// Part 2: 
// On average how many degrees does one user have (how many connections in a sample of this set) does someone have?

struct Graph {
    adj_list: Vec<Vec<usize>>,
   
}

impl Graph {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n: usize = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        for (_i, line) in buf_reader.enumerate() {
            let line_str = line.expect("Error reading");
            if _i == 0 {
                n = line_str.parse::<usize>().unwrap();
                adj_list = vec![vec![];n];
            }
            else {
                let v: Vec<&str> = line_str.trim().split_whitespace().collect();
                let x = v[0].parse::<usize>().unwrap();
                let y = v[1].parse::<usize>().unwrap();
                adj_list[x].push(y);
            }
        }
        return Graph {
            adj_list,
        };
    }

    // calculate degree of a node or vertex
    fn calculate_degree(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_node: usize = rng.gen_range(0..self.adj_list.len());
    
        let mut count = 0;

        for _ in self.adj_list[random_node as usize].iter() {
            count +=1
    }
        // println!("degree of the node is {}", count);
        return count;
} 
   
}

// vector with 500 nodes and their respective amount of degrees (connections) 
// calculated to have the average degrees of each vertex
// represents average number of connections that each user would have 
pub fn main() {
    let data = Graph::new("facebook_combined.txt");
    let mut vector_of_nodes:Vec<i32> = vec![];

    for _ in 1..= 500 {
        let degree_of_node:i32 = Graph::calculate_degree(&data); 
        // println!("{}", degree_of_node);
        vector_of_nodes.push(degree_of_node);
    }

    let sum: i32 =  vector_of_nodes.iter().sum();
    let average_degree_vertex: f32 = sum as f32/ vector_of_nodes.len() as f32;

    println!("Random 500 nodes and the degree value they each have");
    println!("{:?}", vector_of_nodes);
    println!("Average degree of all the nodes (average amount of connections a user has): {}", average_degree_vertex);

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let graph = Graph::new("test.txt");
        assert_eq!(graph.adj_list.len(), 4);
        assert_eq!(graph.adj_list[0], vec![1]);
        assert_eq!(graph.adj_list[1], vec![2]);
        assert_eq!(graph.adj_list[2], vec![3]);
        assert_eq!(graph.adj_list[3], vec![1]);
    }

    #[test]
    fn test_calculate_degree_of_node() {
        let graph = Graph::new("test.txt");
        let degree = graph.calculate_degree();
        assert!(degree == 1 || degree == 2);
    }

    #[test]
    fn test_main() {
        main();
    }
}