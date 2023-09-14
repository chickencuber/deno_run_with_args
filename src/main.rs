use std::env;
use std::process::Command;
fn main() {
    let mut file = env::current_exe().unwrap();
    file.pop();
    file.push("Deno Run");
    let args: Vec<String> = env::args().collect();
    let deno_script_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    let mut cmd = Command::new("cd");
    cmd.arg(file).arg("&").arg("cmd").arg("/c").arg("start").arg("cmd").arg("/k").arg("deno").arg("run").arg("--allow-all").arg("main.ts").args(deno_script_args);
    let _ = cmd.spawn();

}
