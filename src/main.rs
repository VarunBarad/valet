use crate::credit_cards::store_credit_card_statements;
use log::{error, info};
use security_framework::os::macos::keychain::SecKeychain;
use security_framework::os::macos::passwords::{find_internet_password, SecAuthenticationType, SecProtocolType};
use std::path::Path;
use std::process::Command;
use url::{form_urlencoded, Url};

mod credit_cards;

#[derive(Debug)]
pub struct Config {
	pub storage_mount_path_local: String,
	pub storage_mount_path_remote: String,
	pub temp_dir: String,
	pub inbox_dir: String,
}

fn setup(config: &Config) -> std::io::Result<()> {
	info!("Creating temporary directory: {}", config.temp_dir);
	std::fs::create_dir_all(Path::new(&config.temp_dir)).unwrap();

	info!("Creating mount point: {}", config.storage_mount_path_local);
	std::fs::create_dir_all(Path::new(&config.storage_mount_path_local)).unwrap();

	info!("Mounting network share: {} -> {}", config.storage_mount_path_remote, config.storage_mount_path_local);
	let mount_result = Command::new("mount_smbfs")
		.arg(config.storage_mount_path_remote.as_str())
		.arg(config.storage_mount_path_local.as_str())
		.output()
		.unwrap();

	if !mount_result.status.success() {
		error!("Failed to mount network share: {}", String::from_utf8_lossy(&mount_result.stderr));
	}

	return Ok(());
}

fn cleanup(config: &Config) -> std::io::Result<()> {
	info!("Unmounting network share: {}", config.storage_mount_path_local);
	let umount_result = Command::new("umount")
		.arg(config.storage_mount_path_local.as_str())
		.output()
		.unwrap();

	if !umount_result.status.success() {
		error!("Failed to unmount network share: {}", String::from_utf8_lossy(&umount_result.stderr));
	}

	return Ok(());
}

fn get_storage_mount_path_remote(network_share_password: String) -> String {
	info!("Constructing network share mount path");
	let encoded_password = form_urlencoded::byte_serialize(network_share_password.as_bytes()).collect::<String>();
	let actual_url = format!("//varun:{}@delphinus/Legal Documents", encoded_password.as_str());
	let prefixed_url = format!("https:{}", actual_url);
	let parsed_url = Url::parse(prefixed_url.as_str()).unwrap();

	return parsed_url.to_string().strip_prefix("https:").unwrap().to_string();
}

fn get_network_share_password() -> std::io::Result<String> {
	info!("Retrieving network share password from keychain");
	let keychain = SecKeychain::open("/Users/varunb/Library/Keychains/login.keychain-db").unwrap();
	let (password, _) = find_internet_password(
		Some(&[keychain]),
		"delphinus",
		None,
		"varun",
		"",
		None,
		SecProtocolType::SMB,
		SecAuthenticationType::Any,
	)
	.unwrap();

	return Ok(String::from_utf8(password.to_vec()).unwrap());
}

fn main() {
	env_logger::init();
	info!("Starting valet file processor");

	let config = Config {
		storage_mount_path_local: String::from("/tmp/Volumes/Legal Documents"),
		storage_mount_path_remote: get_storage_mount_path_remote(get_network_share_password().unwrap()),
		temp_dir: String::from("/Users/varunb/temp/valet"),
		inbox_dir: String::from("/Users/varunb/Downloads"),
	};

	setup(&config).unwrap();

	info!("Processing credit card statements");
	store_credit_card_statements(&config).unwrap();

	cleanup(&config).unwrap();

	println!("Finished parking files");
	info!("Finished parking files");
}
