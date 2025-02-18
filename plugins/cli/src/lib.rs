// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{
    plugin::{Builder, PluginApi, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

mod config;
mod error;
mod parser;

use config::{Arg, Config};
pub use error::Error;
type Result<T> = std::result::Result<T, Error>;

pub struct Cli<R: Runtime>(PluginApi<R, Config>);

impl<R: Runtime> Cli<R> {
    pub fn matches(&self) -> Result<parser::Matches> {
        parser::get_matches(self.0.config(), self.0.app().package_info())
    }
}

pub trait CliExt<R: Runtime> {
    fn cli(&self) -> &Cli<R>;
}

impl<R: Runtime, T: Manager<R>> CliExt<R> for T {
    fn cli(&self) -> &Cli<R> {
        self.state::<Cli<R>>().inner()
    }
}

#[tauri::command]
fn cli_matches<R: Runtime>(_app: AppHandle<R>, cli: State<'_, Cli<R>>) -> Result<parser::Matches> {
    cli.matches()
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::new("cli")
        .js_init_script(include_str!("api-iife.js").to_string())
        .invoke_handler(tauri::generate_handler![cli_matches])
        .setup(|app, api| {
            app.manage(Cli(api));
            Ok(())
        })
        .build()
}
