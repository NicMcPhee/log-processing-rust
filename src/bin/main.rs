use std::env;

use log_processing_rust::{failed_login::{FailedLogin}, html_report::HtmlReport};

fn main() {
    // We skip the first argument because that's the name of the program.
    let log_file_names: Vec<String> = env::args().skip(1).collect();
    let failed_logins: Vec<FailedLogin> = FailedLogin::process_log_files(&log_file_names);
    let report: HtmlReport = HtmlReport::new(failed_logins);
    report.save("failed_logins.html");
}
