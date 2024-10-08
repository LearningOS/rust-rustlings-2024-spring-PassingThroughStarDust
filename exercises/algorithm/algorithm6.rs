/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

//
use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        //TODO
        //  递归实现
        visited.insert(v);
        visit_order.push(v);
        for i in 0..self.adj[v].len() {
            if let None = visited.get(&self.adj[v][i]) {
                self.dfs_util(self.adj[v][i], visited, visit_order);
            }
        }

        /*  循环实现 方法1
        //  vector当stack用
        let mut stack = Vec::new();
        visited.insert(v);
        visit_order.push(v);
        stack.push(v);
        while !stack.is_empty(){
            //  这里使用stack.last()或stack.last_mut()获取末尾数值会有违背reference rules的现象
            let i = stack[stack.len() - 1];
            for j in 0..self.adj[i].len() {
                if let None = visited.get(&self.adj[i][j]) {
                    visited.insert(self.adj[i][j]);
                    visit_order.push(self.adj[i][j]);
                    stack.push(self.adj[i][j]);
                    break;
                }
            }
            if i == stack[stack.len() - 1] {
                stack.pop();
            }
        }   */

        /*  循环实现 方法2
        let mut stack = Vec::new();
        visited.insert(v);
        visit_order.push(v);
        stack.push(v);
        while let Some(i0) = stack.last_mut() {
            //  必须在这里解引用，如果沿用i0会导致下方发生reference冲突
            let mut i = *i0;
            for j in 0..self.adj[i].len() {
                if let None = visited.get(&self.adj[i][j]) {
                    visited.insert(self.adj[i][j]);
                    visit_order.push(self.adj[i][j]);
                    stack.push(self.adj[i][j]);
                    break;
                }
            }
            if Some(&mut i) == stack.last_mut() {
                stack.pop();
            }
        }   */
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

