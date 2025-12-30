/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/


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
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 检查起始节点是否有效
        if start >= self.adj.len() {
            return vec![];
        }

        // 1. 初始化访问标记数组：记录节点是否已被访问，避免重复访问（处理有环图）
        let mut visited = vec![false; self.adj.len()];
        // 2. 初始化队列：用于存储待层级遍历的节点
        let mut queue = VecDeque::new();
        // 3. 初始化访问顺序向量
        let mut visit_order = vec![];

        // 4. 起始节点入队，并标记为已访问
        visited[start] = true;
        queue.push_back(start);

        // 5. 循环处理队列，直到队列为空
        while let Some(current_node) = queue.pop_front() {
            // 将当前节点加入访问顺序
            visit_order.push(current_node);

            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[current_node] {
                // 若邻接节点未被访问，标记后入队
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
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
