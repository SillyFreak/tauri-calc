import { invoke } from '@tauri-apps/api/tauri';

export type Address = string;
export type Value = any;

export async function getRowAddress(rowIndex: number): Promise<string> {
	return invoke('get_row_address', { rowIndex });
}

export async function getColAddress(colIndex: number): Promise<string> {
	return invoke('get_col_address', { colIndex });
}

export async function getCellAddress(rowIndex: number, colIndex: number): Promise<string> {
	return invoke('get_cell_address', { rowIndex, colIndex });
}

export async function setCell(
	address: string,
	input: string,
): Promise<{ [address: Address]: Value }> {
	return invoke('set_cell', { address, input });
}
