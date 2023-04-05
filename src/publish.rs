use super::*;
use std::process::Command;

/// Publish a workspace to crates.io
pub fn publish_workspace(
    crates: Vec<CrateInfo>,
    crates_io_token: &str,
    version: &str,
    aligned_versions_only: bool,
    dry_run: bool,
    verify_upload_retries: u8,
    cargo_publish_args: Vec<String>,
) -> Result<()> {
    // Iterate through each Cargo.toml file
    for _crate in &crates {
        print_status!("Verifying", &format!("`{}`", _crate.name));

        // Check if the version in the Cargo.toml matches the expected crate version
        if aligned_versions_only && _crate.version != version {
            bail!("Error: {:?} version is not {}", _crate.name, version);
        }

        // Check if the crate version is already on crates.io
        if is_crate_version_uploaded(&_crate.name, &_crate.version) {
            print_note!(&format!(
                "`{}` version `{}` is already on crates.io",
                _crate.name, _crate.version
            ));
            continue;
        }

        // Publish the crate
        publish_crate(_crate, crates_io_token, cargo_publish_args.clone())
            .context(format!("Failed to publish `{}`", _crate.name))?;

        if dry_run {
            continue;
        }

        // Retry checking if the crate version is uploaded to crates.io and available for download
        print_status!(
            "Waiting",
            &format!("for `{}` to appear in crates.io", _crate.name)
        );
        for i in 1..=verify_upload_retries {
            println!("...attempt {} of {}", i, verify_upload_retries);
            if is_crate_version_uploaded(&_crate.name, &_crate.version) {
                print_status!(
                    "Found",
                    &format!("`{} {}` on crates.io", _crate.name, _crate.version)
                );
                break;
            } else {
                println!(
                    "...did not find `{} {}` on crates.io. Sleeping for 2 seconds.",
                    _crate.name, _crate.version
                );
                sleep(Duration::from_secs(2));
            }
        }
    }
    Ok(())
}

/// Publish a crate to crates.io
pub fn publish_crate(
    _crate: &CrateInfo,
    crates_io_token: &str,
    cargo_publish_args: Vec<String>,
) -> Result<()> {
    let crate_dir = _crate
        .manifest_path
        .parent()
        .context("Failed to find parent directory of Cargo.toml")?;

    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("publish");

    print_status!("Publishing", &format!("`{}` to crates.io", _crate.name));
    let output = cargo_command
        .arg(format!("--token={}", crates_io_token))
        .arg("--locked")
        .args(cargo_publish_args)
        .current_dir(crate_dir)
        .output()
        .context("Failed to execute 'cargo publish' command")?;

    if !output.status.success() {
        bail!(
            "Crate publishing failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

/// Check if a crate version is uploaded to crates.io
pub fn is_crate_version_uploaded(name: &str, version: &str) -> bool {
    let output = Command::new("curl")
        .arg(format!(
            "https://crates.io/api/v1/crates/{}/{}",
            name, version
        ))
        .output()
        .expect("Failed to execute curl");

    let response: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse JSON");
    response.get("version").is_some()
}
