// panic

use std::panic::set_hook;

fn main() {
    let important_code = 400;

    set_hook(Box::new(|panic_info| {
        println!("Didn't get a 200 code yet");
        println!("Panic info: {:?}", panic_info.payload().downcast_ref::<&str>());
    }));

    panic!("Oh the humanity!");
}
