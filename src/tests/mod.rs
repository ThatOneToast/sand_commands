use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::sync::Once;
use tokio::runtime::Runtime;

pub mod validation;

#[cfg(test)]
pub mod ban;
#[cfg(test)]
pub mod damage;
#[cfg(test)]
pub mod defaultgamemode;
#[cfg(test)]
pub mod difficulty;
#[cfg(test)]
pub mod enchant;
#[cfg(test)]
pub mod fill;
#[cfg(test)]
pub mod gamemode;
#[cfg(test)]
pub mod gamerule;
#[cfg(test)]
pub mod give;
#[cfg(test)]
pub mod kill;
#[cfg(test)]
pub mod macros;
#[cfg(test)]
pub mod op;
#[cfg(test)]
pub mod save;
#[cfg(test)]
pub mod scoreboard;
#[cfg(test)]
pub mod setblock;
#[cfg(test)]
pub mod tag;
#[cfg(test)]
pub mod teleport;
#[cfg(test)]
pub mod time;
#[cfg(test)]
pub mod title;
#[cfg(test)]
pub mod trigger;
#[cfg(test)]
pub mod unban;
#[cfg(test)]
pub mod weather;
#[cfg(test)]
pub mod whitelist;
#[cfg(test)]
pub mod worldborder;
#[cfg(test)]
pub mod xp;

const SERVER_BYTES1_21_4: &[u8] = include_bytes!("../../assets/CommandValidator1.21.4.zip");

// Static initializers for singleton pattern
#[cfg(test)]
static INIT: Once = Once::new();
#[cfg(test)]
static CLEANUP: Once = Once::new();

#[allow(dead_code)]
fn load_command_validator() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir().join("command_validator_server");
    fs::create_dir_all(&temp_dir)?;
    let reader = std::io::Cursor::new(SERVER_BYTES1_21_4);
    let mut archive = zip::ZipArchive::new(reader)?;
    archive.extract(&temp_dir)?;

    let script_path = temp_dir.join("1.21.4").join("start.sh");
    fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;

    Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.join("1.21.4"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    Ok(())
}

#[cfg(test)]
fn wait_for_server_shutdown() -> bool {
    // Send the shutdown command
    let _ = std::process::Command::new("curl")
        .args(["-X", "POST", "http://0.0.0.0:3000/shutdown"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    println!("Shutdown command sent, waiting for server to stop (up to 30 seconds)...");

    // Check every second for 30 seconds
    for i in 0..30 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Try to connect to the main endpoint
        let status = std::process::Command::new("curl")
            .arg("http://0.0.0.0:3000")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        // If curl fails to connect, the server is down
        if status.is_err() || !status.unwrap().success() {
            println!("Server stopped after {} seconds", i + 1);
            return true;
        }
    }

    println!("Warning: Server did not shut down within 30 seconds");
    false
}

#[cfg(test)]
async fn init_command_validator() -> Result<(), Box<dyn std::error::Error>> {
    load_command_validator()?;

    // Poll until server is ready
    let client = reqwest::Client::new();
    for i in 0..30 {
        match client.get("http://localhost:3000/").send().await {
            Ok(_) => {
                println!("Command validator server started successfully");
                return Ok(());
            }
            Err(e) => {
                if i == 29 {
                    return Err(format!("Failed to connect to server: {}", e).into());
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
    Err("Server failed to start within 30 seconds".into())
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    INIT.call_once(|| {
        println!("Starting Command Validator... Can take up to 30 Seconds");
        let rt = Runtime::new().expect("Failed to create runtime");
        rt.block_on(async {
            init_command_validator()
                .await
                .expect("Failed to start command validator");
        });
    });
}

#[cfg(test)]
#[ctor::dtor]
fn cleanup() {
    CLEANUP.call_once(|| {
        if INIT.is_completed() {
            // Try to shutdown server and wait for confirmation
            if !wait_for_server_shutdown() {
                println!("Failed to confirm server shutdown");
            }

            // Remove temporary directory
            if let Ok(temp_dir) = std::env::temp_dir()
                .join("command_validator_server")
                .canonicalize()
            {
                let _ = std::fs::remove_dir_all(temp_dir);
            }
        }
    });
}
