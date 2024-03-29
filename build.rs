#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./assets/icon.ico"); // Optional: Set the application icon
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
