/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

impl std::error::Error for NodeNotInGraph {}

pub trait Graph {
    /// 创建一个新的图
    fn new() -> Self;
    
    /// 获取可变的邻接表
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    
    /// 获取不可变的邻接表
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    /// 添加一个节点到图中
    /// 如果节点已存在返回false，否则返回true
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            return false;
        }
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }
    
    /// 添加一条边到图中
    fn add_edge(&mut self, edge: (&str, &str, i32));
    
    /// 检查图中是否包含指定节点
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }
    
    /// 获取图中所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    
    /// 获取图中所有边的列表
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
    
    /// 获取指定节点的邻居
    fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        self.adjacency_table()
            .get(node)
            .ok_or(NodeNotInGraph)
    }
    
    /// 获取图中节点的数量
    fn node_count(&self) -> usize {
        self.adjacency_table().len()
    }
    
    /// 获取图中边的数量
    fn edge_count(&self) -> usize {
        self.edges().len()
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> Self {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        
        // 确保起点和终点都存在于图中
        self.add_node(from);
        self.add_node(to);
        
        // 转换为String类型
        let from_str = from.to_string();
        let to_str = to.to_string();
        
        // 无向图：向起点的邻接表添加终点和权重
        self.adjacency_table_mutable()
            .get_mut(&from_str)
            .unwrap()
            .push((to_str.clone(), weight));
        
        // 无向图：向终点的邻接表添加起点和权重（双向边）
        self.adjacency_table_mutable()
            .get_mut(&to_str)
            .unwrap()
            .push((from_str, weight));
    }
}

pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    fn new() -> Self {
        DirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        
        // 确保起点和终点都存在于图中
        self.add_node(from);
        self.add_node(to);
        
        // 转换为String类型
        let from_str = from.to_string();
        let to_str = to.to_string();
        
        // 有向图：只向起点的邻接表添加终点和权重
        self.adjacency_table_mutable()
            .get_mut(&from_str)
            .unwrap()
            .push((to_str, weight));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undirected_graph_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        
        // 检查节点是否都被添加
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
        
        // 检查边是否都被正确添加（无向图应该有双向边）
        let edges = graph.edges();
        assert!(edges.contains(&(&"a".to_string(), &"b".to_string(), 5)));
        assert!(edges.contains(&(&"b".to_string(), &"a".to_string(), 5)));
        assert!(edges.contains(&(&"b".to_string(), &"c".to_string(), 10)));
        assert!(edges.contains(&(&"c".to_string(), &"b".to_string(), 10)));
        assert!(edges.contains(&(&"c".to_string(), &"a".to_string(), 7)));
        assert!(edges.contains(&(&"a".to_string(), &"c".to_string(), 7)));
    }

    #[test]
    fn test_directed_graph_add_edge() {
        let mut graph = DirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        
        // 检查节点是否都被添加
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
        
        // 检查边是否都被正确添加（有向图只有单向边）
        let edges = graph.edges();
        assert!(edges.contains(&(&"a".to_string(), &"b".to_string(), 5)));
        assert!(edges.contains(&(&"b".to_string(), &"c".to_string(), 10)));
        assert!(!edges.contains(&(&"b".to_string(), &"a".to_string(), 5)));
        assert!(!edges.contains(&(&"c".to_string(), &"b".to_string(), 10)));
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        
        // 添加新节点应该返回true
        assert!(graph.add_node("a"));
        assert!(graph.add_node("b"));
        
        // 添加已存在的节点应该返回false
        assert!(!graph.add_node("a"));
        assert!(!graph.add_node("b"));
        
        // 检查节点数量
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_neighbors() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("a", "c", 7));
        graph.add_edge(("b", "c", 10));
        
        // 获取节点a的邻居
        let neighbors_a = graph.neighbors("a").unwrap();
        assert_eq!(neighbors_a.len(), 2);
        assert!(neighbors_a.contains(&("b".to_string(), 5)));
        assert!(neighbors_a.contains(&("c".to_string(), 7)));
        
        // 获取不存在的节点应该返回错误
        assert!(graph.neighbors("d").is_err());
    }
}