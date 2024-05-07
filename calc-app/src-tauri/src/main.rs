// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;

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
) -> Result<HashMap<CellAddress, Value>, String> {
    let mut sheet = sheet.lock().unwrap();

    sheet
      .set_cell(address, formula)
      .map_err(|error| error.to_string())
}

fn main() {
    fn sum(values: &[Value]) -> Value {
        use bigdecimal::BigDecimal;
        use calc::value::Error;

        fn inner(values: &[Value]) -> Result<Value, Error> {
            let mut sum = BigDecimal::default();
            for value in values {
                sum += value.as_number()?;
            }

            Ok(Value::Number(sum))
        }

        inner(values).unwrap_or_else(Value::Error)
    }

    let mut sheet = Sheet::new();
    sheet.set_function("sum", sum);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(sheet))
        .invoke_handler(tauri::generate_handler![get_formula, set_formula])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
