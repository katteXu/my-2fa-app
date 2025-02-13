use tauri_plugin_shell::ShellExt;

pub fn run(app: &tauri::AppHandle) {
    println!("ntpclient启动");
    match app.shell().sidecar("ntpclient") {
        Ok(sidecar_command) => {
            println!("ntpclient启动success");
            let (mut _rx, mut _child) = sidecar_command.spawn().expect("Failed to spawn sidecar");
        }
        Err(e) => {
            println!("ntpclient报错：Error: {}", e);
        }
    }
}
