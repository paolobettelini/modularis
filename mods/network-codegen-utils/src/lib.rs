use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use codegen_utils::{
    GeneratedCrate, GeneratedDependency, GeneratedDependencySource, GeneratedFile,
    crate_ident_for_type, normalize_crate_name, variant_name_for_type, write_generated_crate,
};
use toml::Value;

const NETWORK_MESSAGES_PACKAGE: &str = "generated-network-messages";
const NETWORK_MESSAGES_VERSION: &str = "0.1.0";

#[derive(Debug, Default)]
struct NetworkMessages {
    clientbound: Vec<String>,
    serverbound: Vec<String>,
    dependencies: BTreeMap<String, GeneratedDependency>,
}

#[derive(Debug)]
pub struct GenerateNetworkMessagesOptions {
    pub project_dir: PathBuf,
    pub output_crate_dir: PathBuf,
    pub dev_crate_dir: Option<PathBuf>,
    pub package: String,
    pub version: String,
}

pub fn generate_network_messages_crate<P: AsRef<Path>>(
    project_dir: P,
) -> Result<PathBuf, Box<dyn Error>> {
    let project_dir = project_dir.as_ref().canonicalize()?;
    let messages_crate_dir = default_messages_crate_dir(&project_dir)?;
    generate_network_messages_crate_at(&project_dir, &messages_crate_dir)?;
    Ok(messages_crate_dir)
}

pub fn generate_network_messages_crate_at<P: AsRef<Path>, Q: AsRef<Path>>(
    project_dir: P,
    messages_crate_dir: Q,
) -> Result<(), Box<dyn Error>> {
    generate_network_messages(GenerateNetworkMessagesOptions {
        project_dir: project_dir.as_ref().to_path_buf(),
        output_crate_dir: messages_crate_dir.as_ref().to_path_buf(),
        dev_crate_dir: None,
        package: NETWORK_MESSAGES_PACKAGE.to_string(),
        version: NETWORK_MESSAGES_VERSION.to_string(),
    })
}

pub fn generate_network_messages(
    options: GenerateNetworkMessagesOptions,
) -> Result<(), Box<dyn Error>> {
    let project_dir = options.project_dir.canonicalize()?;
    let messages = read_messages_from_project(&project_dir)?;

    write_messages_crate(
        &options.output_crate_dir,
        &messages,
        &options.package,
        &options.version,
    )?;

    if let Some(dev_crate_dir) = &options.dev_crate_dir {
        write_messages_crate(dev_crate_dir, &messages, &options.package, &options.version)?;
    }

    Ok(())
}

fn default_messages_crate_dir(project_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let parent = project_dir
        .parent()
        .ok_or("composed project directory has no parent")?;

    Ok(parent.join(NETWORK_MESSAGES_PACKAGE))
}

fn read_messages_from_project(project_dir: &Path) -> Result<NetworkMessages, Box<dyn Error>> {
    let project_manifest = read_toml(&project_dir.join("Cargo.toml"))?;
    let mut messages = NetworkMessages::default();

    let Some(dependencies) = project_manifest
        .get("dependencies")
        .and_then(Value::as_table)
    else {
        return Ok(messages);
    };

    for dependency in dependencies.values() {
        let Some(mod_dir) = dependency_path(project_dir, dependency)? else {
            continue;
        };

        let manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        read_message_metadata(&manifest, &mod_dir, &mut messages)?;
    }

    Ok(messages)
}

fn dependency_path(base_dir: &Path, dependency: &Value) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let path = dependency
        .as_table()
        .and_then(|table| table.get("path"))
        .and_then(Value::as_str);

    let Some(path) = path else {
        return Ok(None);
    };

    let path = PathBuf::from(path);
    let path = if path.is_absolute() {
        path
    } else {
        base_dir.join(path)
    };

    Ok(Some(path.canonicalize()?))
}

fn read_message_metadata(
    manifest: &Value,
    mod_dir: &Path,
    messages: &mut NetworkMessages,
) -> Result<(), Box<dyn Error>> {
    let Some(network) = manifest
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("network"))
    else {
        return Ok(());
    };

    let Some(message_table) = network.get("messages").and_then(Value::as_table) else {
        return Ok(());
    };

    for ty in read_type_array(message_table.get("clientbound"))? {
        collect_type_dependency(manifest, mod_dir, &ty, messages)?;
        messages.clientbound.push(ty);
    }

    for ty in read_type_array(message_table.get("serverbound"))? {
        collect_type_dependency(manifest, mod_dir, &ty, messages)?;
        messages.serverbound.push(ty);
    }

    Ok(())
}

fn read_type_array(value: Option<&Value>) -> Result<Vec<String>, Box<dyn Error>> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };

    let Some(values) = value.as_array() else {
        return Err("network message metadata must be an array of type paths".into());
    };

    let mut types = Vec::new();
    for value in values {
        let Some(ty) = value.as_str() else {
            return Err("network message metadata contains a non-string type path".into());
        };
        types.push(ty.to_string());
    }

    Ok(types)
}

fn collect_type_dependency(
    manifest: &Value,
    mod_dir: &Path,
    ty: &str,
    messages: &mut NetworkMessages,
) -> Result<(), Box<dyn Error>> {
    let crate_ident = crate_ident_for_type(ty)?;

    if declaring_package_matches_crate(manifest, &crate_ident) {
        return Err(format!(
            "network message type '{ty}' is declared in the mod crate itself; put payload types in a leaf types crate to avoid a dependency cycle with {NETWORK_MESSAGES_PACKAGE}"
        )
        .into());
    }

    let Some(dependencies) = manifest.get("dependencies").and_then(Value::as_table) else {
        return Err(format!(
            "network message type '{ty}' uses crate '{crate_ident}', but the declaring mod has no dependencies table"
        )
        .into());
    };

    for (key, dependency) in dependencies {
        if normalize_crate_name(key) != crate_ident {
            continue;
        }

        let generated_dependency = generated_dependency(mod_dir, key, dependency)?;
        if let Some(existing) = messages.dependencies.get(key) {
            if !same_dependency(existing, &generated_dependency) {
                return Err(format!(
                    "network message dependency '{key}' resolves to both '{}' and '{}'",
                    dependency_source_display(existing),
                    dependency_source_display(&generated_dependency)
                )
                .into());
            }
        } else {
            messages
                .dependencies
                .insert(key.clone(), generated_dependency);
        }
        return Ok(());
    }

    Err(format!(
        "network message type '{ty}' uses crate '{crate_ident}', but that crate is not a path dependency of the declaring mod"
    )
    .into())
}

fn same_dependency(left: &GeneratedDependency, right: &GeneratedDependency) -> bool {
    left.package == right.package
        && dependency_source_display(left) == dependency_source_display(right)
}

fn dependency_source_display(dependency: &GeneratedDependency) -> String {
    match &dependency.source {
        GeneratedDependencySource::Version(version) => version.clone(),
        GeneratedDependencySource::Path(path) => path.display().to_string(),
    }
}

fn declaring_package_matches_crate(manifest: &Value, crate_ident: &str) -> bool {
    manifest
        .get("package")
        .and_then(Value::as_table)
        .and_then(|package| package.get("name"))
        .and_then(Value::as_str)
        .map(normalize_crate_name)
        .as_deref()
        == Some(crate_ident)
}

fn generated_dependency(
    mod_dir: &Path,
    key: &str,
    dependency: &Value,
) -> Result<GeneratedDependency, Box<dyn Error>> {
    let Some(table) = dependency.as_table() else {
        return Err(format!("network message dependency '{key}' must be a path dependency").into());
    };

    let Some(path) = table.get("path").and_then(Value::as_str) else {
        return Err(format!("network message dependency '{key}' must be a path dependency").into());
    };

    let package = table
        .get("package")
        .and_then(Value::as_str)
        .map(str::to_string);

    let path = PathBuf::from(path);
    let path = if path.is_absolute() {
        path
    } else {
        mod_dir.join(path)
    };

    let dependency = GeneratedDependency::path(key, path.canonicalize()?);
    Ok(if let Some(package) = package {
        dependency.with_package(package)
    } else {
        dependency
    })
}

fn write_messages_crate(
    messages_crate_dir: &Path,
    messages: &NetworkMessages,
    package: &str,
    version: &str,
) -> Result<(), Box<dyn Error>> {
    let mut dependencies = vec![
        GeneratedDependency::version("bevy", "0.17.3"),
        GeneratedDependency::version("serde", "1.0").with_feature("derive"),
        GeneratedDependency::version("serde_cbor", "0.11"),
    ];
    dependencies.extend(messages.dependencies.values().cloned());

    write_generated_crate(
        messages_crate_dir,
        &GeneratedCrate {
            package: package.to_string(),
            version: version.to_string(),
            dependencies,
            lib_rs: generate_lib_rs(),
            generated_files: vec![GeneratedFile::new(
                "generated/messages.rs",
                generate_messages_rs(messages)?,
            )],
        },
    )
}

fn generate_lib_rs() -> String {
    r#"pub use serde;
pub use serde_cbor;

pub mod generated {
    #![allow(dead_code)]

    include!("generated/messages.rs");
}

pub use generated::*;
"#
    .to_string()
}

fn generate_messages_rs(messages: &NetworkMessages) -> Result<String, Box<dyn Error>> {
    let clientbound = build_variants(&messages.clientbound)?;
    let serverbound = build_variants(&messages.serverbound)?;

    let mut code = String::new();
    code.push_str(&generate_enum("ClientBoundMessage", &clientbound));
    code.push('\n');
    code.push_str(&generate_enum("ServerBoundMessage", &serverbound));
    code.push_str(&generate_cbor_helpers());
    code.push_str(&generate_event_dispatch(&clientbound, &serverbound));

    Ok(code)
}

fn build_variants(types: &[String]) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut seen = HashSet::new();
    let mut variants = Vec::new();

    for ty in types {
        let variant = variant_name_for_type(ty)?;
        if !seen.insert(variant.clone()) {
            return Err(format!("duplicate generated network message variant '{variant}'").into());
        }
        variants.push((variant, ty.clone()));
    }

    Ok(variants)
}

fn generate_enum(enum_name: &str, variants: &[(String, String)]) -> String {
    let variants = variants
        .iter()
        .map(|(variant, ty)| format!("    {variant}({ty}),"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(crate = "::serde")]
pub enum {enum_name} {{
{variants}
}}
"#
    )
}

fn generate_cbor_helpers() -> String {
    r#"
impl ClientBoundMessage {
    pub fn encode_cbor(&self) -> Result<Vec<u8>, ::serde_cbor::Error> {
        ::serde_cbor::to_vec(self)
    }

    pub fn decode_cbor(bytes: &[u8]) -> Result<Self, ::serde_cbor::Error> {
        ::serde_cbor::from_slice(bytes)
    }
}

impl ServerBoundMessage {
    pub fn encode_cbor(&self) -> Result<Vec<u8>, ::serde_cbor::Error> {
        ::serde_cbor::to_vec(self)
    }

    pub fn decode_cbor(bytes: &[u8]) -> Result<Self, ::serde_cbor::Error> {
        ::serde_cbor::from_slice(bytes)
    }
}

pub fn encode_clientbound_cbor(
    message: &ClientBoundMessage,
) -> Result<Vec<u8>, ::serde_cbor::Error> {
    message.encode_cbor()
}

pub fn decode_clientbound_cbor(
    bytes: &[u8],
) -> Result<ClientBoundMessage, ::serde_cbor::Error> {
    ClientBoundMessage::decode_cbor(bytes)
}

pub fn encode_serverbound_cbor(
    message: &ServerBoundMessage,
) -> Result<Vec<u8>, ::serde_cbor::Error> {
    message.encode_cbor()
}

pub fn decode_serverbound_cbor(
    bytes: &[u8],
) -> Result<ServerBoundMessage, ::serde_cbor::Error> {
    ServerBoundMessage::decode_cbor(bytes)
}
"#
    .to_string()
}

fn generate_event_dispatch(
    clientbound: &[(String, String)],
    serverbound: &[(String, String)],
) -> String {
    let client_events = clientbound
        .iter()
        .map(|(variant, ty)| {
            format!(
                "#[derive(Debug, Clone, ::bevy::prelude::Message)]\npub struct {variant}Received(pub {ty});"
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let server_events = serverbound
        .iter()
        .map(|(variant, ty)| {
            format!(
                "#[derive(Debug, Clone, ::bevy::prelude::Message)]\npub struct {variant}Received {{ pub source: ::std::net::SocketAddr, pub message: {ty} }}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let registrations = clientbound
        .iter()
        .chain(serverbound.iter())
        .map(|(variant, _)| format!("            .add_message::<{variant}Received>()"))
        .collect::<Vec<_>>()
        .join("\n");
    let client_writers = clientbound
        .iter()
        .map(|(variant, _)| {
            format!(
                "    {}_writer: ::bevy::prelude::MessageWriter<'w, {variant}Received>,",
                snake_case(variant)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let server_writers = serverbound
        .iter()
        .map(|(variant, _)| {
            format!(
                "    {}_writer: ::bevy::prelude::MessageWriter<'w, {variant}Received>,",
                snake_case(variant)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let client_matches = clientbound
        .iter()
        .map(|(variant, _)| {
            let writer = snake_case(variant);
            format!(
                "            ClientBoundMessage::{variant}(message) => {{ writers.{writer}_writer.write({variant}Received(message.clone())); }}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let server_matches = serverbound
        .iter()
        .map(|(variant, _)| {
            let writer = snake_case(variant);
            format!(
                "            ServerBoundMessage::{variant}(message) => {{ writers.{writer}_writer.write({variant}Received {{ source: packet.source, message: message.clone() }}); }}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"
use ::bevy::prelude::IntoScheduleConfigs;

#[derive(Debug, Clone, ::bevy::prelude::Message)]
pub struct ClientPacketReceived(pub ClientBoundMessage);

#[derive(Debug, Clone, ::bevy::prelude::Message)]
pub struct ServerPacketReceived {{
    pub source: ::std::net::SocketAddr,
    pub message: ServerBoundMessage,
}}

#[derive(::bevy::prelude::SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkMessageSet {{
    ReceivePackets,
    DispatchPackets,
}}

{client_events}

{server_events}

#[derive(::bevy::ecs::system::SystemParam)]
struct ClientboundMessageWriters<'w, 's> {{
    _marker: ::bevy::prelude::Local<'s, ()>,
{client_writers}
}}

#[derive(::bevy::ecs::system::SystemParam)]
struct ServerboundMessageWriters<'w, 's> {{
    _marker: ::bevy::prelude::Local<'s, ()>,
{server_writers}
}}

pub struct NetworkMessageEventsPlugin;

impl ::bevy::prelude::Plugin for NetworkMessageEventsPlugin {{
    fn build(&self, app: &mut ::bevy::prelude::App) {{
        app.add_message::<ClientPacketReceived>()
            .add_message::<ServerPacketReceived>()
{registrations}
            .configure_sets(
                ::bevy::prelude::Update,
                (
                    NetworkMessageSet::ReceivePackets,
                    NetworkMessageSet::DispatchPackets,
                )
                    .chain(),
            )
            .add_systems(
                ::bevy::prelude::Update,
                dispatch_clientbound_packets.in_set(NetworkMessageSet::DispatchPackets),
            )
            .add_systems(
                ::bevy::prelude::Update,
                dispatch_serverbound_packets.in_set(NetworkMessageSet::DispatchPackets),
            );
    }}
}}

fn dispatch_clientbound_packets(
    mut packets: ::bevy::prelude::MessageReader<ClientPacketReceived>,
    mut writers: ClientboundMessageWriters,
) {{
    for packet in packets.read() {{
        match &packet.0 {{
{client_matches}
        }}
    }}
}}

fn dispatch_serverbound_packets(
    mut packets: ::bevy::prelude::MessageReader<ServerPacketReceived>,
    mut writers: ServerboundMessageWriters,
) {{
    for packet in packets.read() {{
        match &packet.message {{
{server_matches}
        }}
    }}
}}
"#
    )
}

fn snake_case(input: &str) -> String {
    let mut output = String::new();
    for (index, character) in input.chars().enumerate() {
        if character.is_ascii_uppercase() {
            if index > 0 {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else {
            output.push(character);
        }
    }
    output
}

fn read_toml(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}
