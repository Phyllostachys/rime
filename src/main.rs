fn main() {
    let mut arg_list = std::env::args();
    let my_name = arg_list.next().unwrap_or("time".to_string());
    let program = arg_list.next().unwrap_or("".to_string());

    let mut additional_args = Vec::new();
    for arg in arg_list {
        additional_args.push(arg);
    }

    let mut cmd = std::process::Command::new(&program);
    cmd.args(additional_args);
    let start = std::time::Instant::now();
    let output = cmd.output();
    let end = std::time::Instant::now();
    let timediff = end - start;

    match output {
        Ok(o) => {
            //println!("{}", my_name);
            //println!("Program exit status: {}", o.status);
            println!(
                "Real: {}.{:03} s",
                timediff.as_secs(),
                timediff.subsec_millis()
            );
        }
        Err(e) => println!("Error running command: {}", e),
    }
}
