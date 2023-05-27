pub mod logger {
    #[allow(unused)]
    macro_rules! debug {
        ($($t:tt)*) => {
            #[cfg(debug_assertions)]
            //print with color yellow
            println!("\x1b[33m[DEBUG]: {}\x1b[0m", format!($($t)*));
        };
    }
    #[allow(unused)]
    macro_rules! info {
        ($($t:tt)*) => {
            //print with color blue
            println!("\x1b[34m[INFO]: {}\x1b[0m", format!($($t)*));
        };
    }
    macro_rules! error {
        ($($t:tt)*) => {
            //print with color red
            println!("\x1b[31m[ERROR]: {}\x1b[0m", format!($($t)*));
        };
    }
    #[allow(unused)]
    pub(crate) use debug;
    #[allow(unused)]
    pub(crate) use info;
    pub(crate) use error;
}