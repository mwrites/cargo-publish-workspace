use super::*;
use std::{
    collections::HashSet,
    env,
};

#[derive(Debug, clap::Parser)]
#[command(bin_name = "cargo")]
pub enum Command {
    PublishWorkspace(PublishWorkspaceArgs),
}

impl Command {
    pub fn exec(self) -> Result<()> {
        match self {
            Self::PublishWorkspace(args) => exec(args),
        }
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Command::command().debug_assert()
}

/// Publish a workspace's crates
#[derive(Debug, clap::Args)]
#[command(version)]
#[command(arg_required_else_help = true)]
pub struct PublishWorkspaceArgs {
    #[clap(
        short = 'p',
        long,
        required = true,
        help = "The prefix of the crates to publish, e.g. 'my-repo-crate-'"
    )]
    crate_prefix: String,

    #[arg(long, help = "Run without publishing, same as --show-order")]
    dry_run: bool,

    #[clap(long, help = "Only display the order of crates to be published")]
    show_order: bool,

    #[clap(
        long,
        help = "Specify the version to use instead of CI_TAG environment variable"
    )]
    target_version: Option<String>,

    #[clap(
        long,
        help = "Verify that every Cargo.toml version are aligned with the version to publish",
        default_value = "false"
    )]
    aligned_versions_only: bool,

    #[clap(
        long,
        help = "Specify the token to use instead of CRATES_IO_TOKEN environment variable"
    )]
    token: Option<String>,

    #[clap(
        long,
        help = "Crates to exclude and not modify (arg can be supplied multiple times)"
    )]
    exclude: Vec<String>,

    #[clap(
        long,
        help = "The number of retries to attempt when verifying the upload of a crate",
        default_value = "30"
    )]
    verify_upload_retries: u8,

    #[clap(
        last = true,
        name = "cargo-publish-args",
        help = "Additional arguments to pass to 'cargo publish'"
    )]
    cargo_publish_args: Vec<String>,
}

/// Main processing function. Allows us to return a `Result` so that `main` can print pretty error
/// messages.
fn exec(args: PublishWorkspaceArgs) -> Result<()> {
    let PublishWorkspaceArgs {
        crate_prefix,
        dry_run,
        show_order,
        target_version,
        aligned_versions_only,
        token,
        exclude,
        verify_upload_retries,
        cargo_publish_args,
        ..
    } = args;

    // Get the crates to publish in the correct order and filter out any ignored crates
    let crates = order_crates_for_publishing(HashSet::from_iter(exclude), &crate_prefix)?;
    if show_order || dry_run {
        print_status!("Finished", "show dependencies order");
        for (i, _crate) in crates.iter().enumerate() {
            println!("{}. {}", i, _crate.name);
        }
        print_warn!("aborting upload due to dry run");
        return Ok(());
    }

    // Get CI_TAG and CRATES_IO_TOKEN environment variables or command line arguments
    let version = target_version
        .unwrap_or_else(|| {
            env::var(ENV_VAR_CI_TAG)
                .expect("CI_TAG environment variable or --target-version argument not set")
        })
        .replacen('v', "", 1);
    let crates_io_token = token.unwrap_or_else(|| {
        env::var(ENV_VAR_CRATES_IO_TOKEN)
            .expect("CRATES_IO_TOKEN environment variable or --token argument not set")
    });

    print_status!("Publishing", &format!("crates with version: {}", version));
    publish_workspace(
        crates,
        &crates_io_token,
        &version,
        aligned_versions_only,
        dry_run,
        verify_upload_retries,
        cargo_publish_args,
    )?;
    print_status!("Finished", "publishing workspace");
    Ok(())
}
