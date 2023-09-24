// attribute
// #![] : 전체 다 해당
// #[]  : 바로 아래만 해당
// Hints to the compiler
// annotation
// proc macro
// warn <-> allow

// #[warn(unused_variables)]    // default
// #[warn(dead_code)]           // default

#![allow(unused_variables)]
#![allow(dead_code)]

// #[cfg()]    // config,
// #[cfg(not(target_os = "linux"))]
#[cfg(target_os = "linux")]
fn do_something() {
    println!("I am running in Linux");
}

#[cfg(target_os = "windows")]
fn do_something() {
    println!("I am running in Windows");
}

struct JustAsStruct {}

fn main() {
    let some_char = 'a';
    do_something();
}
