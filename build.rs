fn main() {
    let credit_card_hdfc_password = std::env::var("CREDIT_CARD_HDFC_PASSWORD").expect("Please provide password to use for opening HDFC Bank Credit Card statements using environment variable \"CREDIT_CARD_HDFC_PASSWORD\"");
    println!(
        "cargo:rustc-env=CREDIT_CARD_HDFC_PASSWORD={}",
        credit_card_hdfc_password
    );
    let credit_card_icici_password = std::env::var("CREDIT_CARD_ICICI_PASSWORD").expect("Please provide password to use for opening HDFC Bank Credit Card statements using environment variable \"CREDIT_CARD_ICICI_PASSWORD\"");
    println!(
        "cargo:rustc-env=CREDIT_CARD_ICICI_PASSWORD={}",
        credit_card_icici_password
    );

    // Rebuild when these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CREDIT_CARD_HDFC_PASSWORD");
    println!("cargo:rerun-if-env-changed=CREDIT_CARD_ICICI_PASSWORD");
}
