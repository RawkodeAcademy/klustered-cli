use anyhow::anyhow;
use execute::Execute;
use serde_derive::{Deserialize, Serialize};
use std::process::Command;
use which::which;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version {
    #[serde(rename = "clientVersion")]
    pub client_version: ClientVersion,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientVersion {
    pub major: String,
    pub minor: String,
}

pub fn check() -> anyhow::Result<Version> {
    match which("kubectl") {
        Err(_) => return Err(anyhow!("Couldn't find kubectl")),
        Ok(kubectl) => {
            let mut output = match Command::new(kubectl)
                .arg("version")
                .arg("--client=true")
                .arg("-o")
                .arg("json")
                .output()
            {
                Ok(c) => c,
                Err(_) => return Err(anyhow!("Failed to execute kubectl")),
            };

            let stdout = String::from_utf8(output.stdout).expect("msg");

            let version: Version = match serde_json::from_str(stdout.as_str()) {
                Err(e) => return Err(anyhow!("kubectl failed to get client version: {}", e)),
                Ok(v) => v,
            };

            return Ok(version);
        }
    }
}
