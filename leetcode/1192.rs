// There are n servers numbered from 0 to n-1 connected by undirected server-to-server
// connections forming a network where connections[i] = [a, b] represents a connection
// between servers a and b. Any server can reach any other server directly or
// indirectly through the network.
//
// A critical connection is a connection that, if removed, will make some server unable
// to reach some other server.
//
// Return all critical connections in the network in any order.
//
// Example:
// Input: n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
// Output: [[1,3]]
// Explanation: [[3,1]] is also accepted.
//
// Constraints:
// 1 <= n <= 10^5
// n-1 <= connections.length <= 10^5
// connections[i][0] != connections[i][1]
// There are no repeated connections.

use std::cmp::min;

// Finds all bridges in an undirected graph.
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n as usize];
        for edge in connections {
            let (i, j) = (edge[0], edge[1]);
            adj[i as usize].push(j as usize);
            adj[j as usize].push(i as usize);
        }
        let mut pre: Vec<i32> = vec![-1; n as usize];
        let mut low: Vec<i32> = vec![-1; n as usize];
        let mut bridges: Vec<Vec<i32>> = Vec::new();
        Self::dfs(/*count=*/0, 0, 0, &adj, &mut pre, &mut low, &mut bridges);
        bridges
    }
    
    fn dfs(mut count: i32, u: usize, v: usize, adj: &Vec<Vec<usize>>,
           pre: &mut Vec<i32>, low: &mut Vec<i32>, bridges: &mut Vec<Vec<i32>>) {
        count += 1;
        pre[v] = count;
        low[v] = count;
        for &w in adj[v].iter() {
            if pre[w] == -1 {
                Self::dfs(count, v, w, adj, pre, low, bridges);
                if pre[w] == low[w] {
                    bridges.push(vec![v as i32, w as i32]);
                }
            }
            if w != u {
                low[v] = min(low[v], low[w]);
            }
        }
    }
}
