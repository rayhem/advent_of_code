/// 6-by-4 letter grids seen in [2016 Day
/// 8](https://adventofcode.com/2016/day/8), [2019 Day
/// 8](https://adventofcode.com/2019/day/8) and
/// [11](https://adventofcode.com/2019/day/11), [2021 Day
/// 13](https://adventofcode.com/2021/day/13), and [2022 Day
/// 10](https://adventofcode.com/2022/day/10).
const RAW_LETTER_FORMS: &str =
    ".##..###...##..####.####..##..#..#.###...##.#..#.#.....##..###..###...###.#..#.#...#.####
#..#.#..#.#..#.#....#....#..#.#..#..#.....#.#.#..#....#..#.#..#.#..#.#....#..#.#...#....#
#..#.###..#....###..###..#....####..#.....#.##...#....#..#.#..#.#..#.#....#..#..#.#....#.
####.#..#.#....#....#....#.##.#..#..#.....#.#.#..#....#..#.###..###...##..#..#...#....#..
#..#.#..#.#..#.#....#....#..#.#..#..#..#..#.#.#..#....#..#.#....#.#.....#.#..#...#...#...
#..#.###...##..####.#.....###.#..#.###..##..#..#.####..##..#....#..#.###...##....#...####";

const RAW_LETTERS: [char; 18] = [
    'A', 'B', 'C', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'O', 'P', 'R', 'S', 'U', 'Y', 'Z',
];

/// Transpose a string on newline `'\n'` characters, padding resulting rows shorter
/// than the longest input column with spaces.
fn transpose_string(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let max_length = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let mut transposed_lines = vec![String::with_capacity(lines.len()); max_length];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            transposed_lines[i].push(c);
        }

        for transposed_line in transposed_lines
            .iter_mut()
            .take(max_length)
            .skip(line.chars().count())
        {
            transposed_line.push(' ');
        }
    }

    transposed_lines.join("\n")
}

// Splits a string by applying `f` to each line. Lines where `f(line)` evaluates
// to `true` become the split points and are excluded from the resulting groups.
fn split_on_line_matching(input: &str, f: fn(&str) -> bool) -> Vec<String> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for line in input.lines() {
        if f(line) {
            if !current_group.is_empty() {
                groups.push(current_group.join("\n"));
                current_group.clear();
            }
        } else {
            current_group.push(line);
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group.join("\n"));
    }

    groups
}

/// Converts characters encoded in 6-by-4 ASCII grids to their resulting `char` values.
///
/// # Examples
///
/// ```
/// let input = ".##..\n#..#.\n#..#.\n####.\n#..#.\n#..#.\n";
/// assert_eq!(advent::utils::ascii::graphical_chars(input), "A");
/// ```
pub fn graphical_chars(s: &str) -> String {
    assert!(s.is_ascii(), "Non-ascii input is not supported");

    const SPACE_CHAR: char = '.';

    fn all_space_chars(s: &str) -> bool {
        s.chars().all(|ch| ch == SPACE_CHAR)
    }

    let forms = split_on_line_matching(&transpose_string(RAW_LETTER_FORMS), all_space_chars)
        .into_iter()
        .zip(RAW_LETTERS)
        .collect::<Vec<_>>();

    split_on_line_matching(&transpose_string(s), all_space_chars)
        .iter()
        .map(
            |letter| match forms.iter().find(|(raw_form, _)| letter == raw_form) {
                Some((_, ch)) => *ch,
                None => '\0',
            },
        )
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_string() {
        const INPUT: &str = "hello\nworld";
        assert_eq!(super::transpose_string(INPUT), "hw\neo\nlr\nll\nod");
    }

    #[test]
    fn graphical_input() {
        const EXAMPLE: &str = "####..##..#.....##..#..#.#....###...##..
#....#..#.#....#..#.#..#.#....#..#.#..#.
###..#..#.#....#....#..#.#....#..#.#....
#....####.#....#.##.#..#.#....###..#.##.
#....#..#.#....#..#.#..#.#....#....#..#.
####.#..#.####..###..##..####.#.....###.";

        assert_eq!(graphical_chars(EXAMPLE), String::from("EALGULPG"));
    }
}
