use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[test]
fn enforce_domain_boundaries() {
    let domains = load_domains();
    println!("🧪 INITIALIZING ARCH TEST");

    let crate_pattern = format!(
        r"\b(({})_(backend|api|frontend_core|core))\b",
        domains.join("|")
    );
    let crate_regex = Regex::new(&crate_pattern).unwrap();

    let mut violations = Vec::new();
    let mut files_scanned = 0;
    let domain_root = Path::new("../../domain");

    for entry in WalkDir::new(domain_root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().unwrap_or_default() != "rs" {
            continue;
        }

        let path_str = path.to_str().unwrap().replace("\\", "/");

        // --- ANCHORED OWNER DETECTION ---
        let owner = domains.iter().find(|&d| {
            path_str.contains(&format!("domain/{}/", d))
                && path_str.find(&format!("domain/{}/", d)) == path_str.find("domain/")
        });

        let owner = match owner {
            Some(d) => d,
            None => continue,
        };

        files_scanned += 1;
        let content = fs::read_to_string(path).expect("Failed to read file");

        for (line_no, line) in content.lines().enumerate() {
            // Check if this line mentions a crate from another domain
            if let Some(captures) = crate_regex.captures(line) {
                let full_crate_name = &captures[1];
                let target_domain = &captures[2];

                if owner == target_domain {
                    continue;
                }

                // --- SUBTRACTION LOGIC ---
                // We define exactly what a legal mention looks like
                let legal_prefix = format!("{}::integration::shared", full_crate_name);

                // We create a version of the line where all legal calls are deleted
                let sanitized_line = line.replace(&legal_prefix, "---CLEAN---");

                // If the crate name STILL exists in the sanitized line, it's a violation
                let is_illegal = sanitized_line.contains(full_crate_name);

                // Check folder origin
                let required_origin = format!("/integration/domain/{}/", target_domain);
                let is_in_correct_folder = path_str.contains(&required_origin);

                if is_illegal || !is_in_correct_folder {
                    let mut error_msg = format!(
                        "❌ ARCHITECTURE VIOLATION\n\
                         File: {}:{}\n\
                         Owner: `{}` | Target: `{}`\n\
                         Line: `{}`",
                        path_str,
                        line_no + 1,
                        owner,
                        full_crate_name,
                        line.trim()
                    );

                    if !is_in_correct_folder {
                        error_msg.push_str(&format!(
                            "\n  - Location Error: Access must be in `{}`",
                            required_origin
                        ));
                    }
                    if is_illegal {
                        error_msg.push_str("\n  - Visibility Error: Found illegal access path. You must only use `::integration::shared`.");
                    }

                    violations.push(error_msg);
                } else {
                    println!("  [OK] {} -> {} in {}", owner, full_crate_name, path_str);
                }
            }
        }
    }

    println!("📊 Scan Complete. Files checked: {}", files_scanned);

    if !violations.is_empty() {
        panic!(
            "\n🚨 BOUNDARY CHECK FAILED\n\n{}\n",
            violations.join("\n---\n")
        );
    }
}

fn load_domains() -> Vec<String> {
    let paths = fs::read_dir("../../domain").unwrap();
    paths
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .map(|e| e.file_name().into_string().unwrap())
        .filter(|n| n != "shared" && !n.starts_with('.'))
        .collect()
}
