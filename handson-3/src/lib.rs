pub struct Node {
    n_attractions: u32,
    prefix_sum: u32,
}

pub fn knapsack(itineraries: &mut [&mut Vec<Node>], days: usize, n_itiner: usize) -> u32 {
    let mut dp = vec![vec![0; days + 1]; n_itiner + 1];

    // Compute the prefix sums of the attractions for each itinerary
    itineraries.iter_mut().for_each(|it| {
        let mut sum = 0;

        it.iter_mut().for_each(|node| {
            sum += node.n_attractions;
            node.prefix_sum = sum;
        });
    });

    for i in 1..=n_itiner {
        for j in 1..=days {
            let mut max_val = u32::MIN;
            // Find the max by picking k items on the current itinerary
            // and j-k items from the previous itinerary
            for k in 1..=j {
                // i-1 and k-1 because i and k are index of matrix dp that has
                // an extra column and an extra row
                let prefix_sum = itineraries[i - 1][k - 1].prefix_sum;
                max_val = std::cmp::max(max_val, dp[i - 1][j - k] + prefix_sum);
            }
            // Two options:
            // - not include any attractions of the current itinerary
            // - include the attractions from the current itinerary (computed before)
            dp[i][j] = std::cmp::max(max_val, dp[i - 1][j]);
        }
    }
    dp[n_itiner][days]
}

pub fn optimal_selection(topics: &mut [(u32, u32)]) -> u32 {
    // Sort by x
    topics.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let mut lis = Vec::new();

    // Find LIS
    for &pair in topics.iter() {
        // Find the position in LIS where the pair can be inserted
        let pos = match lis.binary_search_by(|&p: &(u32, u32)| {
            if p.0 >= pair.0 || p.1 >= pair.1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        // If the position is equal to the length,
        // the element is the largest element, so it
        // is pushed in the lis
        if pos == lis.len() {
            lis.push(pair);
        } else {
            // Otherwise it replaces the first larger element
            // than it
            lis[pos] = pair;
        }
    }

    // The length of LIS is the answer
    lis.len() as u32
}
