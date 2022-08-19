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

    let kubelet_status = match checks::kubelet_status::check() {
        Ok(c) => c,
        Err(e) => panic!("❌ Failed to check kubelet status: {}", e),
    };

    println!(
        "✅ kubelet status {}",
        kubelet_status.status
    );
}
