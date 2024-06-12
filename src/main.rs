use std::{env, io, process};

mod util;

#[cfg(test)]
mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: depsdot .  or depsdot /home/foobar/rust_project");
        process::exit(1);
    }

    let dir = &args[1];
    let cargos = util::tomls(dir)?;

    let tomls = util::parse_tomls(cargos);

    util::grouped_dependencies(tomls);

    Ok(())
}
