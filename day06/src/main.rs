// puzzle input
const TIMES:     &[u64] = &[53,      71,     78,     80];
const DISTANCES: &[u64] = &[275,   1181,   1215,   1524];

fn part1(ts: &[u64], ds: &[u64]) -> usize {
    std::iter::zip(ts, ds)
        .map(|(&t, &d)| number_of_ways_to_win(t, d))
        .product()
}

fn part2() -> usize {
    let t = 53717880;
    let d = 275118112151524;
    number_of_ways_to_win(t, d)
}

fn main() {
    println!("part1 = {}", part1(TIMES, DISTANCES));
    println!("part2 = {}", part2());
}

fn number_of_ways_to_win(t: u64, d: u64) -> usize {
    (0..t).into_iter()
        .filter(|&hold_time| compute_distance(hold_time, t) > d)
        .count()
}

fn compute_distance(hold_time: u64, race_duration: u64) -> u64 {
    (race_duration - hold_time) * hold_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ways_to_win() {
        assert_eq!(number_of_ways_to_win(7, 9), 4);
        assert_eq!(number_of_ways_to_win(15, 40), 8);
        assert_eq!(number_of_ways_to_win(30, 200), 9);
    }

    #[test]
    fn test_part1() {
        let ts = vec![7, 15, 30];
        let ds = vec![9, 40, 200];
        assert_eq!(part1(&ts, &ds), 288);
    }
}
