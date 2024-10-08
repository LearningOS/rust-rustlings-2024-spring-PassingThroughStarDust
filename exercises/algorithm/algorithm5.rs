/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        /*  从图的角度是在两个节点间构造边（联系），只是在计算机中通过数组存储，数组的结构与实际图的结构会有差异。
            主数组的每个索引值表示值与之对应的节点，该索引下的子数组表示与该节点相关连的节点。
            例如：
            let mut graph = Graph::new(5);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(3, 4);
            形成的数组结构：
            1 - 0 - 0 - 4 - 3
            |
            2
            实际的图结构：
            2 - 0 - 1   3 - 4
            在计算机中遍历图元素应当结合数组结构与图结构。 */
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO
        let mut visit_order = vec![];
        //  借助队列辅助实现BFS
        //  由于需要不断增减数组索引，故不能用for循环；也不能使用递归，因为递归会优先进行深度搜索
        let mut vec_que = VecDeque::new();
        visit_order.push(start);
        vec_que.push_back(start);
        while let Some(i) = vec_que.pop_front() {
            //  必须使用这种方式，利用for循环遍历"&mut vec_que"与“vec_que.iter()”都会产生multiple mutable reference问题 
            for j in 0..self.adj[i].len() {
                match visit_order.iter().position(|&r| r == self.adj[i][j]) {
                    Some(_) => {}
                    None => {
                        visit_order.push(self.adj[i][j]);
                        vec_que.push_back(self.adj[i][j]);
                    }
                }
            }
        }
        visit_order

        /*  刚开始理解有误，把数组结构当成图结构做了，下面的代码在处理的图只有一个时可以正常使用，但当处理的图有多个时结果会错误。
        //  start表示的是开始find的索引。该参数会在一开始就被压入vector，后面有与该参数相同的数字都将被忽略。
        //  假定当输入索引超出adj的范围时，返回空串
        let mut visit_order = vec![];
        if self.adj.len() != 0 && start <= self.adj.len() {
            visit_order.push(start);
            if !self.adj[0].is_empty() {
                visit_order.push(self.adj[start][0]);
                let mut n = 1;
                let mut continue_count = 1;
                while continue_count != 0 {
                    continue_count = 0;
                    //x与y表示以参考点为起点向x,y方向发生的位移 x + y = n。x的上下限可以确定，减少无效判断次数。!!!!!
                    for x in 0..(n + 1) {
                        let y = n - x;
                        //start - x会报错attempt to subtract overflow
                        if start >= x && y < self.adj[start - x].len() {
                            match visit_order.iter().position(|&r| r == self.adj[start - x][y]) {
                                Some(_) => {}
                                None => {
                                    visit_order.push(self.adj[start - x][y]);
                                }
                            }
                            continue_count += 1;
                        }
                        if start + x < self.adj.len() && y < self.adj[start + x].len() {
                            match visit_order.iter().position(|&r| r == self.adj[start + x][y]) {
                                Some(_) => {}
                                None => {
                                    visit_order.push(self.adj[start + x][y]);
                                }
                            }
                            continue_count += 1;
                        }
                    }
                    n += 1;
                }
            }
        }
        visit_order */
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

