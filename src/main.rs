macro_rules! shell {
    (printf $e:tt$(;)*) => {{
        print!("{}", $e);
    }};
}

fn main() {
    shell! {
        printf "Hello World!\n";
    }
}
