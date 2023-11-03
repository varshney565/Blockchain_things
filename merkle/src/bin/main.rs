use merkle::merkleproof;

fn main() {
    let nodes = merkleproof::merkleproof(90, 12);
    for node in nodes {
        print!("{:?} ",node);
    }
}