fn find_node_to_include(i : u32,s : u32,e : u32) -> Option<u32> {
    //edge case
    if i == e && (i - s + 1) %2 == 1 {
        return None
    }
    if (i - s + 1)%2 == 0 {
        //left
        return Some(i-1)
    }else {
        //right
        return Some(i+1)
    }
}

fn next_level_index(i : u32,s : u32,e : u32) -> u32 {
    e + (i - s)/2 + 1
}

pub fn merkleproof(number_of_transactions : u32,mut node_to_search : u32) -> Vec<u32> {
    let mut nodes = Vec::new();
    let mut st = 1;
    let mut en = number_of_transactions;
    // [e+1,e+(e-s+1)/2+(e-s+1)%2]
    while st < en {
        //find the node_to_search in the range [s,e]
        let node_include = find_node_to_include(node_to_search, st, en);
        if let Some(n) = node_include {
            nodes.push(n);
        }
        node_to_search = next_level_index(node_to_search,st,en);
        //find the next level
        let ns = en+1;
        let ne = en+(en-st+1)/2+(en-st+1)%2;
        st = ns;
        en = ne;
    }
    nodes
}