use std::ops::Range;

/// Takes a vector of ranges and merged any overlapping ranges.
#[allow(clippy::ptr_arg)]
pub fn merge<T>(ranges: &Vec<Range<T>>) -> Vec<Range<T>>
where
    T: PartialOrd + Ord + Copy,
{
    let mut sorted = ranges.clone();
    let mut merged = Vec::new();
    sorted.sort_by_key(|range| range.start);

    for range in sorted {
        let mut last = merged.last().unwrap_or(&range).clone();

        if range.start < last.end {
            last.end = range.end.max(last.end);
            merged.pop();
        } else {
            last = range;
        }

        merged.push(last);
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlaps() {
        let ranges = vec![0..1, 2..3, 4..5];
        let merged = merge(&ranges);
        assert_eq!(merged, vec![0..1, 2..3, 4..5]);
    }

    #[test]
    fn union() {
        let ranges = vec![0..2, 1..3, 4..5];
        let merged = merge(&ranges);
        assert_eq!(merged, vec![0..3, 4..5]);
    }

    #[test]
    fn subset() {
        let ranges = vec![0..3, 1..2, 4..5];
        let merged = merge(&ranges);
        assert_eq!(merged, vec![0..3, 4..5]);
    }
}
