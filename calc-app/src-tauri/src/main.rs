#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{collections::HashMap, num::NonZeroU32};

use calc::{address::*, value::Value};

#[tauri::command]
fn get_row_address(row_index: NonZeroU32) -> RowAddress {
  row_index.into()
}

#[tauri::command]
fn get_col_address(col_index: NonZeroU32) -> ColAddress {
  col_index.into()
}

#[tauri::command]
fn get_cell_address(row_index: NonZeroU32, col_index: NonZeroU32) -> CellAddress {
  CellAddress::new(row_index.into(), col_index.into())
}

#[tauri::command]
fn set_cell(address: CellAddress, input: &str) -> HashMap<CellAddress, Value> {
  HashMap::new()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_row_address,
      get_col_address,
      get_cell_address,
      set_cell,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
