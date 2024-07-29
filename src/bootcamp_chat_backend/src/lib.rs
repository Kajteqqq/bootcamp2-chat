use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static NOTES: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn getNotes() -> Vec<String> {
    NOTES.with_borrow(|notes| notes.clone())
}

#[ic_cdk::update]
fn addNote(note: String) {
    NOTES.with_borrow_mut(|notes|{
        notes.push(note);
    })
}