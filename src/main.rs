use std::{io::{stdin, Read}, fmt::Write, fs, process::Command, os::unix::process::CommandExt, env};

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

    let path = format!("{}/rust_quickie", env::var("HOME").unwrap());
    generate(&path, &dep, &data).unwrap();

    let mut command = Command::new("cargo");
    command.arg("run");
    command.current_dir(&path);
    let _ = command.spawn().unwrap().wait();

    fs::remove_dir_all(&path).unwrap();
}



fn generate(path: &str, deps: &str, all: &str) -> std::io::Result<()> {
    fs::create_dir(&path).unwrap();
    fs::write(format!("{path}/Cargo.toml"), format!(r#"
    [package]
    name = "rust-quickie"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    {deps}
    "#))?;

    fs::create_dir(format!("{path}/src"))?;
    fs::write(format!("{path}/src/main.rs"), format!("{all}"))?;
    Ok(())
}
