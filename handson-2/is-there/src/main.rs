use handson2_2::{parse_input, SegmentTree};

fn main() {
    let (segments, queries) = parse_input();
    let segment_tree = SegmentTree::new(&segments);

    for query in queries {
        println!("{}", segment_tree.is_there(query));
    }
}
