use container::Container;
use libc::pid_t;
use nix::unistd::Pid;
use std::process::Command;
use types::{Error, Result};

#[derive(Clone, Debug)]
pub struct Lxd {}

impl Container for Lxd {
    fn lookup(&self, container_id: &str) -> Result<Pid> {
        let command = format!("lxc info {}", container_id);
        let output = tryfmt!(
            Command::new("lxc")
                .args(&["lxc", "info", container_id])
                .output(),
            "Running '{}' failed",
            command
        );

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return errfmt!(format!(
                "Failed to list containers. '{}' exited with {}: {}",
                command,
                output.status,
                stderr.trim_right()
            ));
        }

        let pid = String::from_utf8_lossy(&output.stdout);

        Ok(Pid::from_raw(tryfmt!(
            pid.trim_right().parse::<pid_t>(),
            "expected valid process id from {}, got: {}",
            command,
            pid
        )))
    }
    fn check_required_tools(&self) -> Result<()> {
        tryfmt!(
            Command::new("lxc").arg("--version").output(),
            "cannot execute `lxc`"
        );
        Ok(())
    }
}
