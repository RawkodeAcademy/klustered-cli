mod checks;

fn main() {
    let cli_version = match checks::cli_version::check() {
        Ok(c) => c,
        Err(e) => panic!("❌ Failed to check kubectl client version: {}", e),
    };

    println!(
        "✅ kubectl version {}.{}",
        cli_version.client_version.major, cli_version.client_version.minor
    );

    let kubelet_version = match checks::kubelet_version::check() {
        Ok(c) => c,
        Err(e) => panic!("❌ Failed to check kubelet version: {}", e")
    }

    println!(
        "✅ kubelet version {}.{}",
        kubelet_version.kubelet_version.major, kubelet_version.kubelet_version.minor
    );
}
