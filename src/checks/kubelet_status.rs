use std::process::Command;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct KubeletStatus {
    pub status: String
}

pub fn check() -> Result<KubeletStatus, anyhow::Error> {
    let msg: &'static str = "Failed to run kubelet";
    let output = match Command::new("/usr/bin/systemctl")
        .arg("status")
        .arg("kubelet.service")
        .output()
    {
        Ok(c) => c,
        Err(_) => return Err(anyhow!(msg)),
    };
   
    if !output.status.success() {
        return Err(anyhow!("kubelet failed with an error " ));
    }

    let stdout = String::from_utf8(output.stdout).expect(msg);

    for line in stdout.lines() {
        if line.contains("Active: ") {
            return Ok(KubeletStatus {
                status: line.to_string(),
            });
        }
    }
    Err(anyhow!("Failed to read systemd status"))
}

