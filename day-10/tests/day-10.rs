#[test]
fn should_calculate_number_of_steps() {
    let input = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    let steps = day_10::calculate_steps(input);

    assert_eq![steps, 8];
}

#[test]
fn should_calculate_number_of_enclosing_points() {
    let input = vec![
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ];

    let enclosing_points = day_10::calculate_number_of_enclosing_points(input);

    assert_eq!(enclosing_points, 4);
}
