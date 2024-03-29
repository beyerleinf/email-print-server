use printers;

use crate::config_store::ConfigStoreState;
use tauri::State;

#[derive(serde::Serialize)]
pub struct Printer {
  pub id: String,
  pub name: String,
}

#[derive(serde::Serialize)]
pub struct Settings {
  pub printer_id: Option<String>,
}

#[tauri::command]
pub fn get_settings(state: State<ConfigStoreState>) -> Settings {
  let printer_id = state.0.lock().unwrap().get_value("printer_id");

  Settings { printer_id }
}

#[tauri::command]
pub fn get_printers() -> Vec<Printer> {
  let printers = printers::get_printers();

  return printers
    .iter()
    .map(|p| {
      let printer = p.clone();
      Printer {
        name: printer.name,
        id: printer.id,
      }
    })
    .collect();
}

#[tauri::command]
pub fn set_printer(printer_id: &str, state: State<ConfigStoreState>) {
  state.0.lock().unwrap().store_value("printer_id", printer_id)
}

#[tauri::command]
pub fn print_test_page(state: State<ConfigStoreState>) {
  let printer_id = state.0.lock().unwrap().get_value::<String>("printer_id");

  if let Some(p_id) = printer_id {
    let printer = printers::get_printer_by_id(p_id.as_str()).unwrap();

    let test_str = format!(
      "E-Mail Printer Server Test Page\n\nPrinter: {}",
      printer.name
    );

    printers::print(&printer, test_str.as_bytes());
  }
}
