use crate::credit_cards::store_credit_card_statements;
use security_framework::os::macos::keychain::SecKeychain;
use security_framework::os::macos::passwords::{
    find_internet_password, SecAuthenticationType, SecProtocolType,
};
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
    std::fs::create_dir_all(Path::new(&config.temp_dir)).unwrap();

    std::fs::create_dir_all(Path::new(&config.storage_mount_path_local)).unwrap();
    Command::new("mount_smbfs")
        .arg(config.storage_mount_path_remote.as_str())
        .arg(config.storage_mount_path_local.as_str())
        .output()
        .unwrap();

    return Ok(());
}

fn cleanup(config: &Config) -> std::io::Result<()> {
    Command::new("umount")
        .arg(config.storage_mount_path_local.as_str())
        .output()
        .unwrap();

    return Ok(());
}

fn get_storage_mount_path_remote(network_share_password: String) -> String {
    let encoded_password =
        form_urlencoded::byte_serialize(network_share_password.as_bytes()).collect::<String>();
    let actual_url = format!(
        "//varun:{}@delphinus/Legal Documents",
        encoded_password.as_str()
    );
    let prefixed_url = format!("https:{}", actual_url);
    let parsed_url = Url::parse(prefixed_url.as_str()).unwrap();

    return parsed_url
        .to_string()
        .strip_prefix("https:")
        .unwrap()
        .to_string();
}

fn get_network_share_password() -> std::io::Result<String> {
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
    let config = Config {
        storage_mount_path_local: String::from("/Volumes/Legal Documents"),
        storage_mount_path_remote: get_storage_mount_path_remote(
            get_network_share_password().unwrap(),
        ),
        temp_dir: String::from("/Users/varunb/temp/valet"),
        inbox_dir: String::from("/Users/varunb/Downloads"),
    };
    
    setup(&config).unwrap();
    
    store_credit_card_statements(&config).unwrap();
    
    cleanup(&config).unwrap();
    
    println!("Finished parking files");
}
