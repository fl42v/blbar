use std::process::Command;

pub fn execute_command(command: String) {
    tokio::spawn(async move {
        Command::new("bash")
            .arg("-c")
            .arg(&command)
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute command {}", &command));
    });
}

pub fn suspend() {
    tokio::spawn(async move {
        Command::new("bash")
            .arg("-c")
            .arg("systemctl suspend")
            .spawn()
            .expect("Failed to execute command.");
    });
}

pub fn shutdown() {
    tokio::spawn(async move {
        Command::new("bash")
            .arg("-c")
            .arg("shutdown now")
            .spawn()
            .expect("Failed to execute command.");
    });
}

pub fn reboot() {
    tokio::spawn(async move {
        Command::new("bash")
            .arg("-c")
            .arg("systemctl reboot")
            .spawn()
            .expect("Failed to execute command.");
    });
}

pub fn logout() {
    tokio::spawn(async move {
        Command::new("bash")
            .arg("-c")
            .arg("loginctl kill-user $(whoami)")
            .spawn()
            .expect("Failed to execute command.");
    });
}
