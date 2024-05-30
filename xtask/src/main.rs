#[cfg(test)]
mod tidy;

use std::time::Instant;

use xshell::{cmd, Shell};

fn main() -> xshell::Result<()> {
    let sh = Shell::new()?;

    cmd!(sh, "rustup toolchain install stable --no-self-update").run()?;
    let _e = sh.push_env("RUSTUP_TOOLCHAIN", "stable");
    cmd!(sh, "rustc --version").run()?;

    Ok(())
}

fn section(name: &'static str) -> impl Drop {
    println!("::group::{name}");
    let start = Instant::now();
    defer(move || {
        let elapsed = start.elapsed();
        eprintln!("{name}: {elapsed:.2?}");
        println!("::endgroup::");
    })
}

//  Executes the closure when the scope in which it was created exits akin to defer in Go or try -finally in other languages
fn defer<F: FnOnce()>(f: F) -> impl Drop {
    struct D<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for D<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f()
            }
        }
    }
    D(Some(f))
}