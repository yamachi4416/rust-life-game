mod test_app;
use test_app::Command;

#[test]
fn test_startup_screen() {
    let app = Command::start();
    insta::assert_snapshot!(app.frame());
    assert!(app.quit());
}

#[test]
fn test_key_quit() {
    let app = Command::start();
    assert!(app.quit());
}

#[test]
fn test_key_next_pattern() {
    let mut app = Command::start();

    insta::assert_snapshot!(app.press('n'));
    insta::assert_snapshot!(app.press('n'));

    assert!(app.quit());
}

#[test]
fn test_key_color_change() {
    let mut app = Command::start();

    insta::assert_snapshot!(app.press('c'));
    insta::assert_snapshot!(app.press('c'));

    assert!(app.quit());
}

#[test]
fn test_key_size_zoom() {
    let mut app = Command::start();

    insta::assert_snapshot!(app.press('+'));
    insta::assert_snapshot!(app.press('-'));

    assert!(app.quit());
}

#[test]
fn test_key_move_navigation() {
    let mut app = Command::start();

    insta::assert_snapshot!(app.press('l'));
    insta::assert_snapshot!(app.press('j'));
    insta::assert_snapshot!(app.press('h'));
    insta::assert_snapshot!(app.press('k'));

    assert!(app.quit());
}

#[test]
fn test_key_space_step() {
    let mut app = Command::start();

    insta::assert_snapshot!(app.press(' '));

    assert!(app.quit());
}
