use super::util;

#[derive(Debug,Clone)]
enum Move {
    U,
    D,
    L,
    R,
}

type Moves = Vec<Move>;
type MoveSet = Vec<Moves>;

#[derive(Debug,Clone)]
struct Button(usize);

fn moves_from_str(moves: &str) -> Moves {
    moves.chars().map(|c| match c {
        'U' => Move::U,
        'D' => Move::D,
        'L' => Move::L,
        'R' => Move::R,
        _ => unreachable!(),
    }).collect::<Moves>()
}

fn bathroom_code(moveset: &MoveSet) -> Vec<Button> {
    let mut current = Button(5);
    let mut buttons: Vec<Button> = vec![];
    for moves in moveset {
        current = bathroom_code_digit(&current, &moves);
        buttons.push(current.clone());
    }
    buttons
}

fn bathroom_code_digit(start_button: &Button, moves: &[Move]) -> Button {
    let mut current: Button = (*start_button).clone();
    for move_ in moves.iter() {
        current = next_button(&current, &move_);
    }
    current
}

fn next_button(current: &Button, move_: &Move) -> Button {
    use self::Move::*;
    let &Button(i) = current;
    match (i, (*move_).clone()) {
        (1, U) | (1, L) | (2, L) | (4, U) => Button(1),
        (2, U) | (1, R) | (3, L) | (5, U) => Button(2),
        (3, U) | (3, R) | (2, R) | (6, U) => Button(3),
        (4, L) | (1, D) | (5, L) | (7, U) => Button(4),
        (2, D) | (4, R) | (6, L) | (8, U) => Button(5),
        (3, D) | (5, R) | (6, R) | (9, U) => Button(6),
        (4, D) | (7, L) | (7, D) | (8, L) => Button(7),
        (5, D) | (7, R) | (9, L) | (8, D) => Button(8),
        (6, D) | (8, R) | (9, R) | (9, D) => Button(9),
        _ => unreachable!(),
    }
}

#[derive(Debug,Clone)]
struct StupidKeypadButton(char);

fn stupid_keypad_bathroom_code(moveset: &MoveSet) -> Vec<StupidKeypadButton> {
    let mut current = StupidKeypadButton('5');
    let mut buttons: Vec<StupidKeypadButton> = vec![];
    for moves in moveset {
        current = stupid_keypad_bathroom_code_digit(&current, &moves);
        buttons.push(current.clone());
    }
    buttons
}

fn stupid_keypad_bathroom_code_digit(start_button: &StupidKeypadButton, moves: &[Move]) -> StupidKeypadButton {
    let mut current: StupidKeypadButton = (*start_button).clone();
    for move_ in moves.iter() {
        current = stupid_keypad_next_button(&current, &move_);
    }
    current
}

fn stupid_keypad_next_button(current: &StupidKeypadButton, move_: &Move) -> StupidKeypadButton {
    use self::Move::*;
    let &StupidKeypadButton(c) = current;
    match (c, (*move_).clone()) {
        ('1', U) | ('1', L) | ('1', R) | ('3', U) => StupidKeypadButton('1'),
        ('2', U) | ('2', L) | ('3', L) | ('6', U) => StupidKeypadButton('2'),
        ('1', D) | ('2', R) | ('4', L) | ('7', U) => StupidKeypadButton('3'),
        ('4', U) | ('4', R) | ('3', R) | ('8', U) => StupidKeypadButton('4'),
        ('5', U) | ('5', L) | ('5', D) | ('6', L) => StupidKeypadButton('5'),
        ('2', D) | ('5', R) | ('7', L) | ('A', U) => StupidKeypadButton('6'),
        ('3', D) | ('6', R) | ('8', L) | ('B', U) => StupidKeypadButton('7'),
        ('4', D) | ('7', R) | ('9', L) | ('C', U) => StupidKeypadButton('8'),
        ('9', U) | ('8', R) | ('9', R) | ('9', D) => StupidKeypadButton('9'),
        ('6', D) | ('A', L) | ('B', L) | ('A', D) => StupidKeypadButton('A'),
        ('7', D) | ('A', R) | ('C', L) | ('D', U) => StupidKeypadButton('B'),
        ('8', D) | ('B', R) | ('C', R) | ('C', D) => StupidKeypadButton('C'),
        ('B', D) | ('D', L) | ('D', R) | ('D', D) => StupidKeypadButton('D'),
        _ => unreachable!(),
    }
}

pub fn main() {
    println!("DAY 2");
    let instructions_str = util::read_file("day2_input.txt")
        .expect("Could not get file contents");
    let lines = instructions_str.lines();
    let move_set = lines.map(|l| moves_from_str(l)).collect::<MoveSet>();
    println!("Bathroom code: {:?}", bathroom_code(&move_set));
    println!("Bathroom code for stupid keypad: {:?}", stupid_keypad_bathroom_code(&move_set));
}
