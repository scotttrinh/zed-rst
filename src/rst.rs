use zed_extension_api::{self as zed, LanguageServerId, Result};

struct reStructuredTextExtension;

impl zed::Extension for reStructuredTextExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("esbonio").ok_or_else(|| {
            "The Esbonio reStructuredText language server is not available in your environment (PATH).
                You can install it from https://github.com/swyddfa/esbonio."
                .to_string()
        })?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(NixExtension);
