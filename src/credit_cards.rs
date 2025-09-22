use crate::Config;
use chrono::{Datelike, NaiveDate};
use log::{error, info};
use regex::{Regex, RegexBuilder};
use std::fs;
use std::path::Path;
use std::process::{exit, Command};

fn store_statement_hdfc(config: &Config) -> std::io::Result<()> {
	info!("Processing HDFC credit card statements");
	let card_name_pattern = Regex::new(r"5589").expect("HDFC: Invalid card name regex");

	// Find any matching files in downloads folder
	let mut matches = Vec::new();

	for entry in fs::read_dir(Path::new(&config.inbox_dir))? {
		let path = entry?.path();
		if path.is_file() {
			if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
				if card_name_pattern.is_match(file_name) {
					matches.push(path);
				}
			}
		}
	}

	info!("Found {} HDFC statement files to process", matches.len());

	// For each file
	for encrypted_file in matches {
		info!("Processing HDFC file: {}", encrypted_file.display());
		let temp_file = Path::new(&config.temp_dir).join("unencrypted_hdfc.pdf");

		// Unencrypt the file
		let unencryption_result = Command::new("qpdf")
			.arg("--decrypt")
			.arg(format!("--password={}", env!("CREDIT_CARD_HDFC_PASSWORD")).as_str())
			.arg(encrypted_file.display().to_string().as_str())
			.arg(temp_file.display().to_string().as_str())
			.output()?;

		let status_code = unencryption_result.status.code().unwrap();
		if status_code != 0 && status_code != 2 && status_code != 3 {
			error!(
                "Error unencrypting HDFC credit-card statement. File: {}. Status code: {}. Command stdout: {}. Command stderr: {}.",
                encrypted_file.display().to_string().as_str(),
                status_code,
                String::from_utf8_lossy(&unencryption_result.stdout),
                String::from_utf8_lossy(&unencryption_result.stderr),
            );
			exit(1);
		}
		info!("Successfully decrypted HDFC file");

		// Get the date of statement
		let input_date = encrypted_file
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.split("_")
            .nth(1)
			.unwrap();
		let parsed_input_date = NaiveDate::parse_from_str(input_date, "%d-%m-%Y").unwrap();

		let statement_year = parsed_input_date.year().to_string();

		let output_file_name = format!("{}.pdf", parsed_input_date.format("%Y-%m-%d"));
		info!("HDFC statement date: {}, output file: {}", parsed_input_date.format("%Y-%m-%d"), output_file_name);

		let output_file_directory = Path::new(&config.storage_mount_path_local)
			.join("Bank Accounts")
			.join("Varun - HDFC")
			.join("Credit Card statement")
			.join(statement_year.as_str());

		// Ensure correct directory exists
		fs::create_dir_all(&output_file_directory).unwrap();

		// Copy renamed statement over to the target directory
		info!("Copying HDFC file to: {}", output_file_directory.join(&output_file_name).display());
		let copy_result = fs::copy(&temp_file, output_file_directory.join(&output_file_name));
		if copy_result.is_err() {
			error!("Error copying file: {}", copy_result.unwrap_err());
			exit(1);
		}

		// Delete the temporary file
		let temporary_deletion_result = fs::remove_file(&temp_file);
		if temporary_deletion_result.is_err() {
			error!(
				"Error deleting temporary file: {}",
				temporary_deletion_result.unwrap_err()
			);
		}

		// Delete the file from inbox
		info!("Deleting processed HDFC file from inbox");
		let deletion_result = fs::remove_file(&encrypted_file);
		if deletion_result.is_err() {
			error!("HDFC: Failed to delete file: {}", deletion_result.unwrap_err());
			exit(1);
		}
		info!("Successfully processed HDFC file: {}", output_file_name);
	}

	Ok(())
}

fn icici_get_statement_date(statement_file: &Path) -> Option<NaiveDate> {
	let pdf_text = Command::new("pdftotext")
		.arg(statement_file.display().to_string().as_str())
		.arg("-")
		.output()
		.unwrap();

	let pattern_statement_date = RegexBuilder::new(r"^\s*Statement Date\s*$")
		.case_insensitive(true)
		.build()
		.unwrap();

	let extracted_text = pdf_text.stdout.to_vec();
	let all_text = String::from_utf8_lossy(&extracted_text);
	let lines = all_text.lines().collect::<Vec<&str>>();

	let mut date_string_from_statement: Option<String> = None;
	let mut i = 0;
	while i < lines.len() {
		if pattern_statement_date.is_match(lines[i]) {
			if i + 1 < lines.len() {
				date_string_from_statement = Some(lines[i + 1].to_string())
			}
		}
		i += 1;
	}

	return match date_string_from_statement {
		None => None,
		Some(date_string) => Some(NaiveDate::parse_from_str(&date_string, "%B %d, %Y").unwrap()),
	};
}

fn store_statement_icici(config: &Config) -> std::io::Result<()> {
	info!("Processing ICICI credit card statements");
	let card_name_pattern = Regex::new(r"5241").expect("ICICI: Invalid card name regex");

	// Find any matching files in downloads folder
	let mut matches = Vec::new();

	for entry in fs::read_dir(Path::new(&config.inbox_dir))? {
		let path = entry?.path();
		if path.is_file() {
			if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
				if card_name_pattern.is_match(file_name) {
					matches.push(path);
				}
			}
		}
	}

	info!("Found {} ICICI statement files to process", matches.len());

	// For each file
	for encrypted_file in matches {
		info!("Processing ICICI file: {}", encrypted_file.display());
		let temp_file = Path::new(&config.temp_dir).join("unencrypted_icici.pdf");

		// Unencrypt the file
		let unencryption_result = Command::new("qpdf")
			.arg("--decrypt")
			.arg(format!("--password={}", env!("CREDIT_CARD_ICICI_PASSWORD")).as_str())
			.arg(encrypted_file.display().to_string().as_str())
			.arg(temp_file.display().to_string().as_str())
			.output()?;

		let status_code = unencryption_result.status.code().unwrap();

		if status_code != 0 && status_code != 3 {
			error!(
                "Error unencrypting ICICI credit-card statement. File: {}. Status code: {}. Command stdout: {}. Command stderr: {}.",
                encrypted_file.display().to_string().as_str(),
                status_code,
                String::from_utf8_lossy(&unencryption_result.stdout),
                String::from_utf8_lossy(&unencryption_result.stderr),
            );
			exit(1);
		}
		info!("Successfully decrypted ICICI file");

		// Get the date of statement
		let parsed_input_date = icici_get_statement_date(&temp_file).unwrap();

		let statement_year = parsed_input_date.year().to_string();

		let output_file_name = format!("{}.pdf", parsed_input_date.format("%Y-%m-%d"));
		info!("ICICI statement date: {}, output file: {}", parsed_input_date.format("%Y-%m-%d"), output_file_name);

		let output_file_directory = Path::new(&config.storage_mount_path_local)
			.join("Bank Accounts")
			.join("Varun - ICICI")
			.join("Credit Card statement")
			.join(statement_year.as_str());

		// Ensure correct directory exists
		fs::create_dir_all(&output_file_directory).unwrap();

		// Copy renamed statement over to the target directory
		info!("Copying ICICI file to: {}", output_file_directory.join(&output_file_name).display());
		let copy_result = fs::copy(&temp_file, output_file_directory.join(&output_file_name));
		if copy_result.is_err() {
			error!("Error copying file: {}", copy_result.unwrap_err());
			exit(1);
		}

		// Delete the temporary file
		let temporary_deletion_result = fs::remove_file(&temp_file);
		if temporary_deletion_result.is_err() {
			error!(
				"Error deleting temporary file: {}",
				temporary_deletion_result.unwrap_err()
			);
		}

		// Delete the file from inbox
		info!("Deleting processed ICICI file from inbox");
		let deletion_result = fs::remove_file(&encrypted_file);
		if deletion_result.is_err() {
			error!("ICICI: Failed to delete file: {}", deletion_result.unwrap_err());
			exit(1);
		}
		info!("Successfully processed ICICI file: {}", output_file_name);
	}

	Ok(())
}

pub fn store_credit_card_statements(config: &Config) -> std::io::Result<()> {
	info!("Starting credit card statement processing");
	store_statement_hdfc(config)?;
	store_statement_icici(config)?;
	info!("Completed credit card statement processing");
	Ok(())
}
