use super::*;
use indexmap::IndexMap;
use std::{
    collections::HashSet,
    path::PathBuf,
    process::Command,
};

/// Crate Information
#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub dependencies: Vec<String>,
}

/// Return the order in which crates should be published
pub fn order_crates_for_publishing(
    ignore_crates: HashSet<String>,
    crate_prefix: &str,
) -> Result<Vec<CrateInfo>> {
    let metadata = load_metadata()?;
    let packages = metadata["packages"]
        .as_array()
        .context("Failed to parse packages array")?;

    let mut crates_map = IndexMap::new();
    let mut dependency_graph = IndexMap::new();
    for pkg in packages {
        let pkg_name = pkg["name"]
            .as_str()
            .context("Failed to parse package name")?
            .to_string();
        let pkg_version = pkg["version"]
            .as_str()
            .context("Failed to parse package version")?
            .to_string();
        let pkg_manifest_path = pkg["manifest_path"]
            .as_str()
            .context("Failed to parse package manifest path")?
            .to_string();
        let pkg_dependencies = pkg["dependencies"]
            .as_array()
            .context("Failed to parse package dependencies")?;

        // Check if the crate is marked as unpublishable and skip to the next crate if so
        if pkg["publish"].as_array().is_some() || ignore_crates.contains(&pkg_name) {
            print_note!(&format!(
                "ignoring {} as it is marked with 'published = false'",
                pkg_name
            ));
            continue;
        }

        let solana_dependencies: Vec<String> = pkg_dependencies
            .iter()
            .map(|x| {
                x["name"]
                    .as_str()
                    .context("Failed to parse dependency name")
                    .map(|s| s.to_string())
            })
            .collect::<Result<Vec<String>>>()?
            .into_iter()
            .filter(|x| x.starts_with(crate_prefix))
            .collect();

        let crate_info = CrateInfo {
            name: pkg_name.clone(),
            version: pkg_version,
            manifest_path: PathBuf::from(pkg_manifest_path),
            dependencies: solana_dependencies.clone(),
        };

        crates_map.insert(pkg_name.clone(), crate_info);
        dependency_graph.insert(pkg_name, solana_dependencies);
    }

    let mut sorted_dependency_graph = Vec::new();
    while !dependency_graph.is_empty() {
        let mut deleted_packages = HashSet::new();
        for (package, dependencies) in &dependency_graph {
            if dependencies
                .iter()
                .all(|dep| !dependency_graph.contains_key(dep))
            {
                sorted_dependency_graph.push(crates_map[package].clone());
                deleted_packages.insert(package.clone());
            }
        }

        if deleted_packages.is_empty() {
            anyhow::bail!(
                "Error: Circular dependency suspected between these packages:\n{}",
                dependency_graph
                    .keys()
                    .map(|pkg| format!("{}\n", pkg))
                    .collect::<String>()
            );
        }
        dependency_graph.retain(|package, _| !deleted_packages.contains(package));
    }

    Ok(sorted_dependency_graph)
}

fn load_metadata() -> Result<serde_json::Value> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version=1")
        .output()
        .context("Failed to execute cargo metadata")?;
    let stdout =
        String::from_utf8(output.stdout).context("Failed to convert metadata to string")?;
    let metadata = serde_json::from_str(&stdout).context("Failed to parse metadata JSON")?;
    Ok(metadata)
}
