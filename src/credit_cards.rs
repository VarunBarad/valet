use std::fs;
use std::path::Path;
use regex::Regex;

fn store_statement_hdfc(root_dir: &Path) -> std::io::Result<()> {
  let card_name_pattern = Regex::new(r"5589").expect("HDFC: Invalid card name regex");
  
  // Find any matching files in downloads folder
  let mut matches = Vec::new();
  
  for entry in fs::read_dir(root_dir)? {
    let path = entry?.path();
    if path.is_file() {
      if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
        if card_name_pattern.is_match(file_name) {
          matches.push(path);
        }
      }
    }
  }
  
  for m in matches {
    println!("{}", m.display().to_string());
  }
  
  // For each file
  
  // Unencrypt the file
  
  // Get the date of statement
  
  // Rename the file
  
  // Ensure correct directory exists
  
  // Copy renamed statement over to the target directory
  
  Ok(())
}

fn store_statement_icici() -> std::io::Result<()> {
  Ok(())
}

pub fn store_credit_card_statements(root_dir: &Path) -> std::io::Result<()> {
  store_statement_hdfc(root_dir)?;
  store_statement_icici()?;
  Ok(())
}