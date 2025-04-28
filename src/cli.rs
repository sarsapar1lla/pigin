use clap::{Arg, Command};

pub fn pigin() -> Command {
    Command::new("pigin")
        .author("sarsapar1lla")
        .about("Chess visualisation TUI")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .required(true)
                .num_args(1..)
                .help("File or files to visualise"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_err_if_no_file_provided() {
        let matches = pigin().try_get_matches_from(&["pigin"]);
        assert!(matches.is_err())
    }

    #[test]
    fn parses_file() {
        let matches = pigin().get_matches_from(&["pigin", "--file", "example.pgn"]);
        let files: Vec<_> = matches.get_many::<String>("file").unwrap().collect();
        assert_eq!(files, vec!["example.pgn"])
    }

    #[test]
    fn parses_multiple_files() {
        let matches =
            pigin().get_matches_from(&["pigin", "--file", "example1.pgn", "example2.pgn"]);
        let files: Vec<_> = matches.get_many::<String>("file").unwrap().collect();
        assert_eq!(files, vec!["example1.pgn", "example2.pgn"])
    }
}
