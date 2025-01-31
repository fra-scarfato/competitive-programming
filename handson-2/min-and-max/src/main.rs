use handson2::{parse_input, QueryType, SegmentTree};

fn main() {
    let (nums, queries) = parse_input();
    let mut st = SegmentTree::new(&nums);
    for query in queries {
        //println!("{}", query);
        match query {
            QueryType::Update {
                left_query,
                right_query,
                value,
            } => {
                st.update(left_query - 1, right_query - 1, value);
                // if left_query == 8 && right_query == 36 {
                //     println!("{st}")
                // }
            }
            QueryType::Max {
                left_query,
                right_query,
            } => {
                let max_val = st.max(left_query - 1, right_query - 1);
                // if left_query == 2 && right_query == 9 {
                //     println!("{st}")
                // }
                println!("{}", max_val);
            }
        }
    }
    println!();
}
