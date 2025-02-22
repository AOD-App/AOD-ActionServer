use rocket::{get, State};
use enigo::{Enigo, Direction::Click, Mouse, Button};
use std::sync::{Arc, Mutex};

#[get("/mouseevent/left")]
pub async fn left_click(enigo: &State<Arc<Mutex<Enigo>>>) {
    enigo.lock().unwrap().button(Button::Left, Click).unwrap();
}

#[get("/mouseevent/right")]
pub async fn right_click(enigo: &State<Arc<Mutex<Enigo>>>) {
    enigo.lock().unwrap().button(Button::Right, Click).unwrap();
}