//! Opening web links in the browser. Al Majlis Color offers none of the upstream project's
//! links (help pages, community, website, source code, feedback).

/// Al Majlis Color shows no update checks or release notes: these commands stay (scripts and the
/// control channel know them) but no menu, button or shortcut list offers them.
pub const HIDDEN: &[&str] = &["app.checkForUpdates", "app.whatsNew"];

/// Whether the command `id` is kept out of sight (see [`HIDDEN`]).
pub fn hidden(id: &str) -> bool {
    HIDDEN.contains(&id)
}

/// Open `url` in the user's browser (through the host's `open_url` service).
pub fn open(app: &mut crate::LightcraftApp, url: &str) -> Result<serde_json::Value, String> {
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_upstream_project_links_are_gone() {
        for id in ["app.help", "app.discord", "app.website", "app.github", "app.artcraft", "app.feedback"] {
            assert!(!crate::menus::UI_COMMANDS.iter().any(|c| c.0 == id), "{id}");
        }
    }
}
