use itertools::Itertools;

pub fn parse_input_line(input: &str) -> Vec<i64> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn extrapolate_sequence(sequence: &[i64], reverse: bool) -> i64 {
    let mut differences_in_line = Vec::from(sequence);

    let mut extrapolated = if reverse {
        vec![sequence.first().copied().unwrap()]
    } else {
        vec![sequence.last().copied().unwrap()]
    };

    // build first and last collections
    loop {
        if differences_in_line.iter().all(|&v| v == 0) {
            break;
        }
        let next_differences = differences(&differences_in_line);
        differences_in_line = next_differences;
        if reverse {
            extrapolated.push(differences_in_line.first().copied().unwrap());
        } else {
            extrapolated.push(differences_in_line.last().copied().unwrap());
        }
    }

    if reverse {
        extrapolated
            .into_iter()
            .rev()
            .reduce(|acc, x| x - acc)
            .unwrap()
    } else {
        extrapolated.iter().sum()
    }
}

fn differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differences() {
        assert_eq!(differences(&[3, 6, 9, 12, 15]), &[3, 3, 3, 3]);
    }
}
