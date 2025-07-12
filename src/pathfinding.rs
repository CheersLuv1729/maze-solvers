use std::{collections::{BTreeMap, BTreeSet}, fmt::Error, ops::Add};

fn get_path_from_connections<V>(end: V, connections: &BTreeMap<V, V>) -> Result<Vec<V>, Error>
where V: Clone + Ord
{
    let mut ret = Vec::new();
    let mut c = end.clone();
    while let Some(n) = connections.get(&c) {
        ret.push(n.clone());
        c = n.clone();
    }
    ret.reverse();
    return Ok(ret);
}

pub fn depth_first_search<V, I, F>(start: V, end: V, get_edges: F) -> Result<Vec<V>, Error>
where
    V: Clone + Ord,
    I: Iterator<Item = V>,
    F: Fn(&V) -> I,
{
    let mut stack = Vec::new();
    let mut visited = BTreeSet::new();
    let mut connections: BTreeMap<V, V> = BTreeMap::new();
    stack.push(start);
    while let Some(current_node) = stack.pop()
    {
        if current_node == end {
            return get_path_from_connections(end, &connections);
        }
        if visited.insert(current_node.clone())
        {
            get_edges(&current_node).filter(|v|!visited.contains(v)).for_each(|v| {
                stack.push(v.clone());
                connections.insert(v.clone(), current_node.clone());
            });
        }

    }
    return Err(Error);
}

pub fn dijkstra<V, W, I, F>(start: V, end: V, get_edges: F) -> Result<Vec<V>, Error>
where
    V: Clone + Ord,
    W: Add<Output = W> + Ord + Copy + Default,
    I: Iterator<Item = (V, W)>,
    F: Fn(&V) -> I,
{
    let mut distances = BTreeMap::new();
    let mut unvisited = Vec::new();
    let mut shortest_connections: BTreeMap<V, V> = BTreeMap::new();

    distances.insert(start.clone(), W::default());
    unvisited.push(start.clone());

    while let Some(current_node) = unvisited.pop() {
        if current_node == end {
            return get_path_from_connections(end, &shortest_connections);
        }
        let node_distance = distances[&current_node];

        get_edges(&current_node).for_each(|(vertex, weight)| {
            if let Some(dist) = distances.get(&vertex) {
                if *dist > node_distance + weight {
                    distances.insert(vertex.clone(), node_distance + weight);
                    shortest_connections.insert(vertex.clone(), current_node.clone());
                }
            } else {
                shortest_connections.insert(vertex.clone(), current_node.clone());
                distances.insert(vertex.clone(), node_distance + weight);
                unvisited.push(vertex.clone());
            }
        });
        // Sorting every iteration shouldn't be too inefficient because the vec is always mostly sorted
        unvisited.sort_by(|a, b| distances[a].cmp(&distances[b]));
    }
    return Err(Error);
}