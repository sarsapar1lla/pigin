use clap::{Arg, Command};

pub fn pigin() -> Command {
    Command::new("pgn").arg(
        Arg::new("file")
            .index(1)
            .required(true)
            .help("File or files to visualise"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_err_if_no_file_provided() {
        let matches = pigin().try_get_matches_from(&["pgn"]);
        assert!(matches.is_err())
    }

    #[test]
    fn parses_file() {
        let matches = pigin().get_matches_from(&["pgn", "example.pgn"]);
        assert_eq!(
            matches.get_one::<String>("file"),
            Some(&"example.pgn".to_string())
        )
    }
}
