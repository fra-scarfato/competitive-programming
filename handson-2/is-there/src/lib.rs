use std::{
    collections::HashSet,
    fmt,
    io::{self, BufRead},
};

pub struct Query {
    left_query: usize,
    right_query: usize,
    overlapped_seg: i32,
}

pub struct SegmentTree {
    tree: Vec<HashSet<i32>>,
    upper_bound: usize,
}

impl SegmentTree {
    pub fn new(segments: &Vec<(usize, usize)>) -> Self {
        let upper_bound = segments.len() - 1;
        let n = 4 * segments.len();
        let tree = vec![HashSet::new(); n];

        let occurrences = SegmentTree::compute_occurrences(segments, upper_bound + 1);
        let mut segment_tree = SegmentTree { tree, upper_bound };
        segment_tree.build(&occurrences, 0, upper_bound, 0);

        segment_tree
    }

    fn compute_occurrences(segments: &Vec<(usize, usize)>, size: usize) -> Vec<i32> {
        let mut count = vec![0; size];
        // Sweep line initialization
        for &(l, r) in segments {
            count[l] += 1;
            // The segment terminates after his right bound
            // Otherwise the segments with equal right and left bound are not counted (+1-1 = 0)
            if r + 1 < size {
                count[r + 1] -= 1;
            }
        }

        // Cumulative sum to have the number of overlapping segments
        for i in 1..size {
            count[i] += count[i - 1];
        }
        count
    }

    fn build(
        &mut self,
        occurrences: &Vec<i32>,
        left_bound: usize,
        right_bound: usize,
        curr_index: usize,
    ) {
        if left_bound == right_bound {
            self.tree[curr_index].insert(occurrences[left_bound]);
            return;
        }

        let mid = SegmentTree::mid(left_bound, right_bound);
        let left_child = SegmentTree::left(curr_index);
        let right_child = SegmentTree::right(curr_index);

        self.build(occurrences, left_bound, mid, left_child);
        self.build(occurrences, mid + 1, right_bound, right_child);

        // Merge two hash-sets.
        // union(): it generates an iterator over references to elements
        // cloned(): it generates an iterator over cloned elements
        // collect(): it generates the collection of elements
        let merged_set: HashSet<i32> = self.tree[left_child]
            .union(&self.tree[right_child])
            .cloned()
            .collect();

        self.tree[curr_index] = merged_set;
    }

    pub fn is_there(&self, query: Query) -> u8 {
        if self.is_there_query(
            0,
            query.left_query,
            query.right_query,
            query.overlapped_seg,
            0,
            self.upper_bound,
        ) {
            1
        } else {
            0
        }
    }

    fn is_there_query(
        &self,
        curr_index: usize,
        left_query: usize,
        right_query: usize,
        overlapped_seg: i32,
        left_bound: usize,
        right_bound: usize,
    ) -> bool {
        assert!(left_query <= right_query, "Invalid query range");

        // No overlap
        if left_query > right_bound || right_query < left_bound {
            return false;
        }

        // Total overlap
        if left_query <= left_bound && right_query >= right_bound {
            return self.tree[curr_index].contains(&overlapped_seg);
        }

        // Partial overlap
        let mid_index = Self::mid(left_bound, right_bound);
        let (left_child, right_child) = (Self::left(curr_index), Self::right(curr_index));

        // Return true if it finds the number of overlapped segments in one of his children or in both
        self.is_there_query(
            left_child,
            left_query,
            right_query,
            overlapped_seg,
            left_bound,
            mid_index,
        ) || self.is_there_query(
            right_child,
            left_query,
            right_query,
            overlapped_seg,
            mid_index + 1,
            right_bound,
        )
    }

    fn mid(left: usize, right: usize) -> usize {
        left + (right - left) / 2
    }

    fn left(curr_index: usize) -> usize {
        (2 * curr_index) + 1
    }

    fn right(curr_index: usize) -> usize {
        (2 * curr_index) + 2
    }
}

impl std::fmt::Display for SegmentTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn print_tree<T: std::fmt::Debug>(
            tree: &[T],
            index: usize,
            level: usize,
            f: &mut std::fmt::Formatter<'_>,
            label: &str,
        ) -> std::fmt::Result {
            if index >= tree.len() {
                return Ok(());
            }

            // Print right subtree
            print_tree(tree, 2 * index + 2, level + 1, f, label)?;

            // Print current node
            writeln!(
                f,
                "{}{}{}: {:?}",
                "    ".repeat(level),
                label,
                index,
                tree[index]
            )?;

            // Print left subtree
            print_tree(tree, 2 * index + 1, level + 1, f, label)?;

            Ok(())
        }

        writeln!(f, "Segment Tree:")?;
        print_tree(&self.tree, 0, 0, f, "T")?;

        Ok(())
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Update query: left bound = {}, right bound = {}, position = {}",
            self.left_query, self.right_query, self.overlapped_seg
        )
    }
}

// PARSING
pub fn parse_input() -> (Vec<(usize, usize)>, Vec<Query>) {
    let mut input_file = io::stdin().lock().lines();

    // Parse n and m
    let (n, m) = parse_numbers(
        &input_file
            .next()
            .expect("Missing first line")
            .expect("Failed to read query line"),
    );

    let mut segments: Vec<(usize, usize)> = Vec::with_capacity(n);
    for _ in 0..n {
        let (l, r) = parse_numbers(
            &input_file
                .next()
                .expect("Missing first line")
                .expect("Failed to read query line"),
        );

        segments.push((l, r));
    }

    assert!(
        n == segments.len(),
        "The declared number 'n' does not match the segments number"
    );

    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let query = parse_query(
            &input_file
                .next()
                .expect("Missing query line")
                .expect("Failed to read query line"),
        );

        queries.push(query);
    }

    (segments, queries)
}

fn parse_numbers(line: &str) -> (usize, usize) {
    let mut parts = line.split_whitespace();

    let n = parts
        .next()
        .expect("Missing the size of the array")
        .parse()
        .expect("Parsing error");

    let m = parts
        .next()
        .expect("Missing the size of the array")
        .parse()
        .expect("Parsing error");

    assert!(parts.next().is_none(), "Too much numbers on the array");

    (n, m)
}

fn parse_query(line: &str) -> Query {
    let parts: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse().expect("Parsing error"))
        .collect();

    // Extract a slice containing the entire vector
    // The slice is an array of reference to the value of the previous vector
    match parts.as_slice() {
        [i, j, k] => Query {
            left_query: *i,
            right_query: *j,
            overlapped_seg: *k as i32,
        },
        _ => panic!("Invalid query format"),
    }
}
