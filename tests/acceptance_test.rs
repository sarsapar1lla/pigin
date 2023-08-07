use fenrs::execute_moves;
use fenrs::parse;
use std::fs;

#[test]
fn parses_candidates_games() {
    let test_files = fs::read_dir("./resources/candidates_test").unwrap();
    for file in test_files {
        let file_name = file.as_ref().unwrap().file_name();
        println!("Parsing {:?}", &file_name);
        let path = file.as_ref().unwrap().path();
        let content = fs::read_to_string(path).unwrap();
        let pgn = parse(&content).unwrap();

        let boards = execute_moves(pgn.fen().starting_board(), pgn.ply());
        println!("{boards:?}");
        assert!(boards.is_ok())
    }
}

#[allow(dead_code)]
fn generate_candidates_test() {
    let event_tag = r#"[Event "FIDE Candidates 2022"]"#;
    let file = fs::read_to_string("./samples/Candidates2022.pgn").unwrap();
    let games = file.split_terminator(&event_tag);

    fs::create_dir_all("./resources/candidates_test").unwrap();

    for (idx, game) in games.enumerate() {
        if idx == 0 {
            continue;
        }
        let contents = format!("{event_tag}{game}");
        fs::write(
            format!("./resources/candidates_test/game_{idx}.pgn"),
            contents,
        )
        .unwrap();
    }
}
