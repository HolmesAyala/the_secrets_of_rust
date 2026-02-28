use assert_cmd::Command;
use assert_cmd::cargo_bin;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::predicate;

#[test]
fn given_run_without_args_then_it_should_print_a_usage_example() {
    Command::new(cargo_bin!("count"))
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Usage: count <FILES>",
        ));
}

#[test]
fn given_a_valid_file_path_then_it_should_count_lines() {
    Command::new(cargo_bin!("count"))
        .arg("./tests/data/mock_file.txt")
        .arg("./tests/data/mock_file_b.txt")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("./tests/data/mock_file.txt: 4 lines")
                .and(predicate::str::contains("./tests/data/mock_file_b.txt: 5 lines")),
        );
}

#[test]
fn given_the_w_flag_then_it_should_count_words() {
    Command::new(cargo_bin!("count"))
        .arg("-w")
        .arg("./tests/data/mock_file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("./tests/data/mock_file.txt: 5 words"));
}
