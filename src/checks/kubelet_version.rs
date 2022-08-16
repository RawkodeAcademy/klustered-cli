use anyhow::anyhow;
use serde_derive::{Deserialize, Serialize};
use std::process::Command;
use which::which;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version {
    #[serde(rename = "kubeletVersion")]
    pub kubelet_version: KubeletVersion,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KubeletVersion {
    pub major: String,
    pub minor: String,
}

pub fn check() -> anyhow::Result<Version> {
    match which("kubelet") {
        Err(_) => return Err(anyhow!("Couldn't find kubelet")),
        Ok(kubectl) => {
            let mut output = match Command::new(kubelet)
                .arg("version")
                .output()
            {
                Ok(c) => c,
                Err(_) => return Err(anyhow!("Failed to execute kubelet")),
            };

            let stdout = String::from_utf8(output.stdout).expect("msg");

            let version: Version = match serde_json::from_str(stdout.as_str()) {
                Err(e) => return Err(anyhow!("kubelet failed to get version: {}", e)),
                Ok(v) => v,
            };

            return Ok(version);
        }
    }
}