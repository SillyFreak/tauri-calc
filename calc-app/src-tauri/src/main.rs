#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{InvokeError, State};

use std::collections::HashMap;
use std::sync::Mutex;

use calc::address::*;
use calc::cell::Cell;
use calc::sheet::Sheet;
use calc::value::Value;

#[tauri::command]
fn get_formula(sheet: State<Mutex<Sheet>>, address: CellAddress) -> String {
  let sheet = sheet.lock().unwrap();

  sheet.cell(&address).map_or("", Cell::input).to_string()
}

#[tauri::command]
fn set_formula(
  sheet: State<Mutex<Sheet>>,
  address: CellAddress,
  formula: String,
) -> Result<HashMap<CellAddress, Value>, InvokeError> {
  let mut sheet = sheet.lock().unwrap();

  sheet
    .set_cell(address, formula)
    .map_err(|error| error.to_string().into())
}

fn main() {
  tauri::Builder::default()
    .manage(Mutex::new(Sheet::new()))
    .invoke_handler(tauri::generate_handler![get_formula, set_formula])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
