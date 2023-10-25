// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config_store;

use std::sync::Mutex;

use commands::settings::{get_printers, get_settings, print_test_page, set_printer};
use config_store::{ConfigStore, ConfigStoreState};
use tauri::{App, Manager};

fn main() {
  tauri::Builder::default()
    .setup(|app: &mut App| {
      let store = ConfigStore::init();

      app.manage(ConfigStoreState(Mutex::new(store)));

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_printers,
      set_printer,
      get_settings,
      print_test_page
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
