use crate::sonar_sweep::DepthChange;
use crate::sonar_sweep::Depth;

pub fn calculate_changes(depths: Vec<Depth>, window: usize) -> Vec<DepthChange> {
    let mut changes: Vec<DepthChange> = Vec::new();
    if depths.len() < window+1 {
        return changes;
    }
    for index in window..depths.len() {
        let mut sum_a = 0;
        let mut sum_b = 0;

        for index_sum in 1..window+1 {
            sum_a += depths[index-index_sum].value;
            sum_b += depths[index-index_sum+1].value;
        }

        if sum_a < sum_b {
            changes.push(DepthChange::Increase)
        } else if sum_a > sum_b {
            changes.push(DepthChange::Decrease)
        } else {
            changes.push(DepthChange::NotChanged)
        }
    }
    return changes;
}