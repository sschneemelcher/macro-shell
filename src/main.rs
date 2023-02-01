use std::process::Command;

macro_rules! shell {
    (printf $e:tt$(;)*) => {{
        print!("{}", $e);
    }};
    ($cmd:ident $($arg:tt) *) => {
        let mut cmd = Command::new(stringify! {$cmd});
        // cmd.args(vec! {$($arg) *});
        match cmd.spawn() {
            Ok(mut child) => match child.stdout.take() {
                None => {}
                Some(output) => println! {"{:#?}", output},
            },
            Err(_) => println! {"{}: command not found", stringify! {$cmd}},
        }
    };
}

fn main() {
    shell! {
        printf "Hello World!\n";
    }

    shell! {
        ls
    }

    shell! {
        foo
    }
}
