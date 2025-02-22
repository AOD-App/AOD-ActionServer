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