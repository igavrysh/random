#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum KeyPress {
    Down,
    Up,
}

#[derive(Debug)]
struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

#[allow(dead_code)]
#[derive(Debug)]
enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

impl From<&KeyEvent> for InputEvent {
    fn from(ev: &KeyEvent) -> Self {
        InputEvent::Key(ev.keycode, ev.state.clone())
    }
}

fn main() {
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };

    let input_ev1 = InputEvent::from(&key_ev);
    let input_ev2: InputEvent = (&key_ev).into();

    println!("{input_ev1:?}, {input_ev2:?}")
}