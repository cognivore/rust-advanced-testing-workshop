use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn get_cli_path(config_path: &Path) -> PathBuf {
    let config = std::fs::File::open(config_path).expect("Failed to open config file");
    let reader = BufReader::new(config);

    let path = reader
        .lines()
        .next()
        .expect("The config file is empty")
        .expect("First line is not valid UTF-8");
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::eq;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[googletest::test]
    // Tip: you can use `expected` to specify a value that must be **contained** in the panic message!
    #[should_panic(expected = "Failed to open config file")]
    fn panics_if_file_does_not_exist() {
        let config_file = NamedTempFile::new().unwrap();
        let config_path = config_file.path();
        std::fs::remove_file(&config_path).unwrap();
        super::get_cli_path(&config_path);
    }

    #[googletest::test]
    #[should_panic(expected = "The config file is empty")]
    fn panics_if_file_is_empty() {
        let tmp = NamedTempFile::new().unwrap();
        // Write an empty string to the file
        let mut config_file = tmp;
        write!(config_file, "").unwrap();
        super::get_cli_path(config_file.path());
    }

    #[googletest::test]
    #[should_panic(expected = "First line is not valid UTF-8")]
    fn panics_if_file_contains_invalid_utf8() {
        let invalid_utf8 = [0xFF];
        let mut config_file = NamedTempFile::new().unwrap();
        config_file.write_all(&invalid_utf8).unwrap();
        super::get_cli_path(config_file.path());
    }

    #[googletest::test]
    fn happy_path() {
        let cli_path = PathBuf::from("my_cli");

        let mut config_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(config_file, "{}", cli_path.to_str().unwrap()).unwrap();

        let actual = super::get_cli_path(config_file.path());
        assert_that!(&actual, eq(&cli_path));
    }
}
