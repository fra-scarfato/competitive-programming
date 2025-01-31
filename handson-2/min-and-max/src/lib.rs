use std::{
    cmp::max,
    fmt,
    io::{self, BufRead},
};

pub enum QueryType {
    Update {
        left_query: usize,
        right_query: usize,
        value: i32,
    },
    Max {
        left_query: usize,
        right_query: usize,
    },
}

impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryType::Update {
                left_query: left_bound,
                right_query: right_bound,
                value,
            } => {
                write!(
                    f,
                    "Update query: left_bound = {}, right_bound = {}, value = {}",
                    left_bound, right_bound, value
                )
            }
            QueryType::Max {
                left_query: left_bound,
                right_query: right_bound,
            } => {
                write!(
                    f,
                    "Max query: left_bound = {}, right_bound = {}",
                    left_bound, right_bound
                )
            }
        }
    }
}

pub struct SegmentTree {
    tree: Vec<i32>,
    upper_bound: usize,
    lazy: Vec<Option<i32>>,
}

impl SegmentTree {
    /// Creates a new Segment Tree from a vector of integers
    ///
    /// # Arguments
    /// * `nums` - Vector of integers to build the tree from
    ///
    /// # Returns
    /// A new SegmentTree instance initialized with the given numbers
    pub fn new(nums: &Vec<i32>) -> Self {
        let upper_bound = nums.len() - 1;
        // 4*n is the size of the perfect tree with every level filled
        // If the vector has 2 * n size and n is not a power of 2,
        // accessing the children of some node with 2 * n + 1 and 2 * n + 2
        // could thrown an out of bound error because it tries to access
        // to a leaf that doesn't exists
        //
        // Example:
        //        1
        //      /   \
        //    2     3
        //  /  \   /  \
        // 4   5  =   =
        //        ^
        //        |
        //
        // left(3) = index out of bound
        let n = 4 * nums.len();
        let tree = vec![-1; n];
        let lazy = vec![None; n];

        let mut segment_tree = SegmentTree {
            tree,
            upper_bound,
            lazy,
        };
        segment_tree.build(nums, 0, upper_bound, 0);
        segment_tree
    }

    /// Recursively builds the segment tree structure
    ///
    /// # Arguments
    /// * `nums` - Source array to build from
    /// * `left_bound` - Left boundary of current segment
    /// * `right_bound` - Right boundary of current segment
    /// * `curr_index` - Current node index in the tree
    fn build(&mut self, nums: &Vec<i32>, left_bound: usize, right_bound: usize, curr_index: usize) {
        // Populate the leaves with the array's elements
        if left_bound == right_bound {
            self.tree[curr_index] = nums[left_bound];
            return;
        }
        let mid = Self::mid(left_bound, right_bound);
        let left_child = Self::left(curr_index);
        let right_child = Self::right(curr_index);

        self.build(nums, left_bound, mid, left_child);
        self.build(nums, mid + 1, right_bound, right_child);

        // Populate node with the max of the children
        self.tree[curr_index] = max(self.tree[left_child], self.tree[right_child]);
    }

    pub fn update(&mut self, left_query: usize, right_query: usize, new_val: i32) {
        let upper_bound = self.upper_bound;
        self.update_query(0, left_query, right_query, 0, upper_bound, new_val);
    }

    /// Updates a range in the segment tree with lazy propagation
    ///
    /// # Arguments
    /// * `curr_index` - Current node index in the segment tree
    /// * `left_query` - Left boundary of the query range
    /// * `right_query` - Right boundary of the query range
    /// * `left_bound` - Left boundary of current node's range
    /// * `right_bound` - Right boundary of current node's range
    /// * `new_val` - New value to set in the range
    fn update_query(
        &mut self,
        curr_index: usize,
        left_query: usize,
        right_query: usize,
        left_bound: usize,
        right_bound: usize,
        new_val: i32,
    ) {
        assert!(left_query <= right_query, "Invalid query range");

        // Update the current node and propagate the updates if needed
        self.propagate(curr_index);

        // No overlap
        if left_query > right_bound || right_query < left_bound {
            return;
        }

        // Total overlap
        if left_query <= left_bound && right_query >= right_bound {
            // If the new value is less than the current node, then force the update and the propagation
            if new_val < self.tree[curr_index] {
                self.lazy[curr_index] = Some(new_val);
                self.propagate(curr_index);
            }
            return;
        }

        // Partial overlap
        let mid_index = Self::mid(left_bound, right_bound);
        let left_child = Self::left(curr_index);
        let right_child = Self::right(curr_index);

        self.update_query(
            left_child,
            left_query,
            right_query,
            left_bound,
            mid_index,
            new_val,
        );
        self.update_query(
            right_child,
            left_query,
            right_query,
            mid_index + 1,
            right_bound,
            new_val,
        );

        // Update the node if the children are changed
        self.tree[curr_index] = max(self.tree[left_child], self.tree[right_child]);
    }

    /// Propagates lazy updates from a node to its children
    ///
    /// # Arguments
    /// * `curr_index` - Index of the current node to propagate updates from
    fn propagate(&mut self, curr_index: usize) {
        // If there is a pending update on this node
        if let Some(lazy_val) = self.lazy[curr_index] {
            let left_child = Self::left(curr_index);
            let right_child = Self::right(curr_index);

            self.tree[curr_index] = lazy_val;

            // Propagate the pending update on the children if it is an internal node
            if curr_index < self.tree.len() / 2 - 1 {
                if self.tree[left_child] > lazy_val {
                    self.lazy[left_child] = Some(lazy_val);
                }
                if self.tree[right_child] > lazy_val {
                    self.lazy[right_child] = Some(lazy_val);
                }
            }
            // The node is updated and the pending update is set to None
            self.lazy[curr_index] = None;
        }
    }

    pub fn max(&mut self, left_query: usize, right_query: usize) -> i32 {
        self.max_query(0, left_query, right_query, 0, self.upper_bound)
    }

    fn max_query(
        &mut self,
        curr_index: usize,
        left_query: usize,
        right_query: usize,
        left_bound: usize,
        right_bound: usize,
    ) -> i32 {
        assert!(left_query <= right_query, "Invalid query range");

        // Propagate the updates if needed
        self.propagate(curr_index);

        // No overlap
        if left_query > right_bound || right_query < left_bound {
            return -1;
        }

        // Total overlap
        if left_query <= left_bound && right_query >= right_bound {
            return self.tree[curr_index];
        }

        // Partial overlap
        let mid_index = Self::mid(left_bound, right_bound);
        let left_child = Self::left(curr_index);
        let right_child = Self::right(curr_index);

        let max_left = self.max_query(left_child, left_query, right_query, left_bound, mid_index);
        let max_right = self.max_query(
            right_child,
            left_query,
            right_query,
            mid_index + 1,
            right_bound,
        );
        //println!("L:{left_bound} R:{right_bound} LM:{max_left} RM:{max_right}");

        max(max_left, max_right)
    }

    fn mid(left: usize, right: usize) -> usize {
        // Prevent overflow
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
        writeln!(f, "\nLazy Tree:")?;
        print_tree(&self.lazy, 0, 0, f, "L")?;

        Ok(())
    }
}

// PARSING
pub fn parse_input() -> (Vec<i32>, Vec<QueryType>) {
    let mut input_file = io::stdin().lock().lines();

    // Parse n and m
    let (n, m) = parse_numbers(
        &input_file
            .next()
            .expect("Missing first line")
            .expect("Failed to read query line"),
    );

    // Parse the array
    let num_array = parse_array(
        &input_file
            .next()
            .expect("Second line is missing")
            .expect("Failed to read second line"),
    );

    // Validate array length
    assert!(
        n == num_array.len(),
        "The declared number 'n' does not match the array size"
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

    (num_array, queries)
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

fn parse_array(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|num| num.parse().expect("Parsing error"))
        .collect()
}

fn parse_query(line: &str) -> QueryType {
    let parts: Vec<i32> = line
        .split_whitespace()
        .map(|num| num.parse().expect("Parsing error"))
        .collect();

    match parts[0] {
        0 if parts.len() == 4 => QueryType::Update {
            left_query: parts[1] as usize,
            right_query: parts[2] as usize,
            value: parts[3],
        },
        1 if parts.len() == 3 => QueryType::Max {
            left_query: parts[1] as usize,
            right_query: parts[2] as usize,
        },
        _ => panic!("Invalid query format"),
    }
}
