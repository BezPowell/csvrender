use assert_str::assert_str_trim_all_eq;
use csvrender::{Args, process_csv};
use std::{fs, path::PathBuf, str::FromStr};
use temp_testdir::TempDir;

#[test]
fn convert_wordpress() {
    // Arrange
    let temp = TempDir::default();
    let args = Args {
        output_path: format!("{}/{{post_name}}.md", temp.to_string_lossy()),
        file_template: PathBuf::from_str("tests/templates/wordpress.md").unwrap(),
        csv: PathBuf::from_str("tests/data/wp_pages.csv").unwrap(),
    };

    // Act
    process_csv(args).unwrap();

    // Assert
    assert_str_trim_all_eq!(
        fs::read_to_string(temp.to_path_buf().join("hello-world.md")).unwrap(),
        r#"
        +++
        title = "Hello world!"
        description = "My first post!"
        date = 2025-11-26 10:39:38
        updated = 2025-11-26 10:39:38
        +++
        <p>Welcome to WordPress. This is your first post. Edit or delete it, then start writing!</p>
        "#
    );

    assert_str_trim_all_eq!(
        fs::read_to_string(temp.to_path_buf().join("hello-world-again.md")).unwrap(),
        r#"
        +++
        title = "Hello world again"
        date = 2025-11-26 10:47:00
        updated = 2025-11-26 10:47:00
        +++
        <p>This is another test post, to show off things.</p>
        "#
    );
}
