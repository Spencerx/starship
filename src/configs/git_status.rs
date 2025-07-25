use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub stashed: &'a str,
    pub ahead: &'a str,
    pub behind: &'a str,
    pub up_to_date: &'a str,
    pub diverged: &'a str,
    pub conflicted: &'a str,
    pub deleted: &'a str,
    pub renamed: &'a str,
    pub modified: &'a str,
    pub staged: &'a str,
    pub untracked: &'a str,
    pub typechanged: &'a str,
    pub ignore_submodules: bool,
    pub disabled: bool,
    pub use_git_executable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_starship: Option<&'a str>,
}

impl Default for GitStatusConfig<'_> {
    fn default() -> Self {
        Self {
            format: "([\\[$all_status$ahead_behind\\]]($style) )",
            style: "red bold",
            stashed: "\\$",
            ahead: "⇡",
            behind: "⇣",
            up_to_date: "",
            diverged: "⇕",
            conflicted: "=",
            deleted: "✘",
            renamed: "»",
            modified: "!",
            staged: "+",
            untracked: "?",
            typechanged: "",
            ignore_submodules: false,
            disabled: false,
            use_git_executable: false,
            windows_starship: None,
        }
    }
}
