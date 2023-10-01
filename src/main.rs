use std::env;
use std::process::Command;
fn main() {
    let mut file = env::current_exe().unwrap();
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    file.pop();
    file.push("Deno Run");
    let args: Vec<String> = env::args().collect();
    let deno_script_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    let mut cmd = Command::new("cmd");
    cmd.arg("/c").arg("start").arg("cmd").arg("/c").arg("deno").arg("run").arg("--allow-all").arg("--unstable").arg("main.ts").arg(current_dir).args(deno_script_args).current_dir(file);
    let _ = cmd.spawn();
}
