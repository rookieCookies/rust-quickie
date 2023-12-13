use std::{io::{stdin, Read}, fmt::Write, fs, process::Command, os::unix::process::CommandExt};

fn main() {
    let data = {
        let mut input = Vec::new();
        let result = stdin().read_to_end(&mut input);
        if result.is_err() {
            println!("unable to read stdin");
            return;
        };

        let Ok(result) = String::from_utf8(input)
        else {
            println!("stdin must be valid utf8");
            return;
        };

        result
    };
    

    let dep = { 
        let mut dep = String::new();
        let mut iter = data.lines();
        loop {
            let Some(line) = iter.next()
            else { break };

            let trimmed = line.trim_start();
            if trimmed.starts_with("//+") {
                let _ = writeln!(&mut dep, "{}", trimmed.split_at(3).1);
                continue
            };

            break
        }

        dep
    };

    generate(&dep, &data).unwrap();

    let mut command = Command::new("cargo");
    command.arg("run");
    command.current_dir("./_rust_quickie/");
    let _ = command.spawn().unwrap().wait();

    fs::remove_dir_all("./_rust_quickie").unwrap();
}



fn generate(deps: &str, all: &str) -> std::io::Result<()> {
    fs::create_dir("_rust_quickie")?;
    fs::write("_rust_quickie/Cargo.toml", format!(r#"
    [package]
    name = "rust-quickie"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    {deps}
    "#))?;

    fs::create_dir("_rust_quickie/src")?;
    fs::write("_rust_quickie/src/main.rs", format!("{all}"))?;
    Ok(())
}
