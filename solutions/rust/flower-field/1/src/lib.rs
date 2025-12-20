pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let rows = garden.len();
    let cols = garden[0].len();
    let mut result = vec![vec![' '; cols]; rows];

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if garden[r].as_bytes()[c] == b'*' {
                result[r][c] = '*';
            } else {
                let mut count = 0;
                for &(dr, dc) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if garden[nr as usize].as_bytes()[nc as usize] == b'*' {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    result[r][c] = (b'0' + count) as char;
                }
            }
        }
    }

    result
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}