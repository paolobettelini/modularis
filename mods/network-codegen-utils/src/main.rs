use std::error::Error;
use std::path::PathBuf;

fn main() {
    if let Err(error) = run() {
        eprintln!("network codegen failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);

    let command = args.next().ok_or("missing command: expected 'generate'")?;
    if command != "generate" {
        return Err(format!("unknown command '{command}': expected 'generate'").into());
    }

    let mut project_dir = None;
    let mut output_crate_dir = None;
    let mut dev_crate_dir = None;
    let mut package = "generated-network-messages".to_string();
    let mut version = "0.1.0".to_string();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--project" => {
                project_dir = Some(PathBuf::from(args.next().ok_or("--project needs a path")?));
            }
            "--output-crate" | "--crate-dir" => {
                output_crate_dir = Some(PathBuf::from(
                    args.next().ok_or("--output-crate needs a path")?,
                ));
            }
            "--dev-crate" => {
                dev_crate_dir = Some(PathBuf::from(
                    args.next().ok_or("--dev-crate needs a path")?,
                ));
            }
            "--package" => {
                package = args.next().ok_or("--package needs a value")?;
            }
            "--version" => {
                version = args.next().ok_or("--version needs a value")?;
            }
            "--mods-folder" | "--modpacks-folder" | "--modpack" => {
                let _ = args.next().ok_or(format!("{arg} needs a path"))?;
            }
            _ => return Err(format!("unknown argument '{arg}'").into()),
        }
    }

    let project_dir = project_dir.ok_or("missing --project <composed-template-dir>")?;
    let output_crate_dir = output_crate_dir.unwrap_or_else(|| {
        project_dir
            .parent()
            .map(|parent| parent.join("generated-network-messages"))
            .unwrap_or_else(|| PathBuf::from("generated-network-messages"))
    });

    network_codegen_utils::generate_network_messages(
        network_codegen_utils::GenerateNetworkMessagesOptions {
            project_dir,
            output_crate_dir: output_crate_dir.clone(),
            dev_crate_dir,
            package,
            version,
        },
    )?;

    println!(
        "generated network messages crate at {}",
        output_crate_dir.display()
    );

    Ok(())
}
