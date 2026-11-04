// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake_agent_context::{Definition, Installer};
use flate2::read::GzDecoder;
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn packaged_crate_installs_both_skills_in_a_consumer() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("target");
    let output = Command::new(env!("CARGO"))
        .current_dir(root)
        .args([
            "package",
            "--package",
            "base-security",
            "--allow-dirty",
            "--no-verify",
            "--locked",
            "--offline",
            "--target-dir",
        ])
        .arg(&target)
        .output()
        .expect("package base-security");
    assert!(
        output.status.success(),
        "packaging failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let archive_path = fs::read_dir(target.join("package"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "crate")
        })
        .expect("Cargo produced a crate archive");
    let unpacked = directory.path().join("unpacked");
    let decoder = GzDecoder::new(fs::File::open(&archive_path).unwrap());
    tar::Archive::new(decoder).unpack(&unpacked).unwrap();
    let package = unpacked.join(archive_path.file_stem().unwrap());
    assert!(!package.join("bake").exists());
    assert!(!package.join("design").exists());
    assert!(!package.join("security").exists());
    assert!(!package.join(".agents").exists());

    let consumer = directory.path().join("consumer");
    fs::create_dir_all(consumer.join("src")).unwrap();
    fs::write(consumer.join("src/lib.rs"), "").unwrap();
    fs::write(
        consumer.join("Cargo.toml"),
        format!(
            "[package]\nname = \"security-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n\n[dependencies]\nbase-security = {{ path = {package:?} }}\n"
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO"))
        .current_dir(&consumer)
        .args(["generate-lockfile", "--offline"])
        .output()
        .expect("resolve consumer dependencies");
    assert!(
        output.status.success(),
        "consumer resolution failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let installer = Installer::discover(&consumer).unwrap();
    let provider = installer.find_provider("base-security").unwrap().unwrap();
    assert_eq!(provider.specification.context_path, package.join("context"));
    assert_eq!(provider.definitions.len(), 2);

    let installed = installer.install(Some("base-security")).unwrap();
    assert!(installed.context.is_empty());
    assert_eq!(
        installed.skills,
        ["base-security-advisory", "base-security-audit"]
    );
    for definition in &provider.definitions {
        let Definition::Skill(skill) = definition else {
            panic!("base-security should only distribute skills");
        };
        let path = consumer
            .join(".agents/skills")
            .join(&skill.name)
            .join("SKILL.md");
        assert_eq!(
            fs::read_to_string(path).unwrap(),
            skill.installation_content()
        );
    }
}
