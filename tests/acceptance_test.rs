use fenrs::execute_moves;
use fenrs::parse;
use std::fs;

#[test]
fn parses_games() {
    let test_files = fs::read_dir("./resources/test/acceptance").unwrap();
    for file in test_files {
        let file_name = file.as_ref().unwrap().file_name();
        println!("Parsing {:?}", &file_name);
        let path = file.as_ref().unwrap().path();
        let content = fs::read_to_string(path).unwrap();
        let pgns = parse(&content).unwrap();

        println!("Games parsed: {}", pgns.len());

        for pgn in pgns.iter() {
            let boards = execute_moves(pgn.fen().starting_board(), pgn.ply());
            assert!(boards.is_ok())
        }
    }
}
