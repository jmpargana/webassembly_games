use crate::*;


#[test]
fn constructors() {
    Board::new(10, 20);
    Board::new(20, 5);
}

#[test]
#[should_panic]
fn small_board() {
    Board::new(3, 3);
}


#[test]
fn return_none_in_all_expected_places() {
    let game = Board::new(10, 10);

    assert!(game.get(9, 0) == None);
    assert!(game.get(-1, -1) == None);
    assert!(game.get(20, 20) == None);
}

#[test]
fn save_chips() {
    let mut game = Board::new(10, 10);

    game.add(0, Chip::Blue);
    assert!(game.get(9, 0) == Some(Chip::Blue));

    game.add(0, Chip::Red);
    assert!(game.get(8, 0) == Some(Chip::Red));
    assert!(game.get(9, 0) == Some(Chip::Blue));
    assert!(game.get(7, 0) == None);

    game.add(0, Chip::Blue);
    assert!(game.get(7, 0) == Some(Chip::Blue));
}

#[test]
fn check_vertical_winner() {
    let mut game = Board::new(10, 10);

    for _ in 0..4 {
        game.add(0, Chip::Blue);
    }

    assert!(game.check() == Some(Chip::Blue)); 
}

#[test]
fn check_horizontal_winner() {
    let mut game = Board::new(10, 10);

    for c in 0..4 {
        println!("{}", c);
        game.add(c, Chip::Red);
    }

    assert!(game.check() == Some(Chip::Red));
}

#[test]
fn check_no_winner() {
    let mut game = Board::new(10, 10);

    for c in 3..6 {
        game.add(c, Chip::Blue);
    }
    game.add(6, Chip::Red);

    assert!(game.check() == None);


}
