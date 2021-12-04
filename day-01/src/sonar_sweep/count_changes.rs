use crate::sonar_sweep::DepthChange;

pub fn count_changes(changes: Vec<DepthChange>, depth_change: DepthChange) -> u16 {
    let mut occurences = 0;
    for change in changes {
        if change == depth_change {
            occurences += 1;
        }
    }
    return occurences;
}