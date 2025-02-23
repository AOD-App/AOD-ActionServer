use rocket::{get, State};
use enigo::{Enigo, Direction::Click, Key, Keyboard};
use std::sync::{Arc, Mutex};

#[get("/keyevent/left")]
pub async fn left(enigo: &State<Arc<Mutex<Enigo>>>) {
    enigo.lock().unwrap().key(Key::LeftArrow, Click).unwrap();
}

#[get("/keyevent/right")]
pub async fn right(enigo: &State<Arc<Mutex<Enigo>>>) {
    enigo.lock().unwrap().key(Key::RightArrow, Click).unwrap();
}

// #[get("/keyevent/volumeup")]
// pub async fn volume_up(enigo: &State<Arc<Mutex<Enigo>>>) {
//     enigo.lock().unwrap().key(Key::VolumeUp, Click).unwrap();
// }

// #[get("/keyevent/volumedown")]
// pub async fn volume_down(enigo: &State<Arc<Mutex<Enigo>>>) {
//     enigo.lock().unwrap().key(Key::VolumeDown, Click).unwrap();
// }