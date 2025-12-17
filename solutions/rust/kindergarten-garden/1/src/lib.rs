pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let index = students.iter().position(|&s| s == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|line| line.chars().skip(index).take(2))
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => "",
        })
        .collect()
}