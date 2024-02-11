#![windows_subsystem = "windows"]

use env::var;
use dotenv::dotenv;
use hotkey;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use enigo;
use enigo::{Enigo, Key, KeyboardControllable};

fn set_clipboard(item: &str){

    // add new item to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    let the_string = item;
    ctx.set_contents(the_string.to_owned()).unwrap();
}

fn paste(){

    // set variables from .env file
    let key = "EMAIL";
    let val = var(key).unwrap();

    // set clipboard value to email variable
    set_clipboard(&val);

    // invoke paste function
    let mut enigo = Enigo::new();
    //paste
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
}

fn main() {

    // configure dotenv file
    dotenv().ok();

    // set email variable to clipboard

    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'A' as u32,
        move || paste(),
    )
        .unwrap();

    hk.listen();
}
