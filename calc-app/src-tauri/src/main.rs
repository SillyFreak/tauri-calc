#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{InvokeError, State};

use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use calc::{address::*, cell::Cell, sheet::Sheet, value::Value};

#[tauri::command]
fn get_row_address(row_index: NonZeroU32) -> RowAddress {
  row_index.into()
}

#[tauri::command]
fn get_col_address(col_index: NonZeroU32) -> ColAddress {
  col_index.into()
}

#[tauri::command]
fn get_cell(
  sheet: State<Mutex<Sheet>>,
  row_index: NonZeroU32,
  col_index: NonZeroU32,
) -> (CellAddress, String) {
  let sheet = sheet.lock().unwrap();

  let address = CellAddress::new(row_index.into(), col_index.into());
  let value = sheet.cell(&address).map_or("", Cell::input).to_string();

  (address, value)
}

#[tauri::command]
fn set_cell(
  sheet: State<Mutex<Sheet>>,
  address: CellAddress,
  input: String,
) -> Result<HashMap<CellAddress, Value>, InvokeError> {
  let mut sheet = sheet.lock().unwrap();

  sheet
    .set_cell(address, input)
    .map_err(|error| error.to_string().into())
}

fn main() {
  tauri::Builder::default()
    .manage(Mutex::new(Sheet::new()))
    .invoke_handler(tauri::generate_handler![
      get_row_address,
      get_col_address,
      get_cell,
      set_cell,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
