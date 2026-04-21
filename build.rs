fn main() {
    #[cfg(target_os = "windows")]
    {
        let package_name = env!("CARGO_PKG_NAME");
        let description = non_empty(
            option_env!("CARGO_PKG_DESCRIPTION"),
            "StoryVote Server",
        );
        let company_name = first_author(option_env!("CARGO_PKG_AUTHORS")).unwrap_or("In-House OSS");
        let original_filename = format!("{package_name}.exe");
        let version = windows_resource_version();

        let mut res = winres::WindowsResource::new();
        res.set("CompanyName", company_name)
            .set("FileDescription", description)
            .set("InternalName", package_name)
            .set("LegalCopyright", "Copyright (c) 2026")
            .set("OriginalFilename", &original_filename)
            .set("ProductName", description)
            .set("ProductVersion", &version)
            .set("FileVersion", &version);

        if let Err(error) = res.compile() {
            panic!("failed to compile Windows resources: {error}");
        }
    }
}

#[cfg(target_os = "windows")]
fn windows_resource_version() -> String {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let mut parts = pkg_version.split('.').take(4).collect::<Vec<_>>();
    while parts.len() < 4 {
        parts.push("0");
    }
    parts.join(".")
}

#[cfg(target_os = "windows")]
fn non_empty<'a>(value: Option<&'a str>, fallback: &'a str) -> &'a str {
    match value {
        Some(v) if !v.trim().is_empty() => v,
        _ => fallback,
    }
}

#[cfg(target_os = "windows")]
fn first_author(authors: Option<&str>) -> Option<&str> {
    authors
        .and_then(|value| value.split(':').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}
