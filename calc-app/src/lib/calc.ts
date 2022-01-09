import { invoke } from '@tauri-apps/api/tauri';

export type Address = string;
export type Value =
	| { type: 'Number'; value: string }
	| { type: 'String'; value: string }
	| { type: 'Error'; value: string };

export type AnyValue = Value | { type: 'Empty' };

export function getRowAddress(rowIndex: number): string {
	if (rowIndex < 1) throw new Error('rowIndex must be positive');

	return `${rowIndex}`;
}

export function getColAddress(colIndex: number): string {
	if (colIndex < 1) throw new Error('colIndex must be positive');

	function base26digit(num: number) {
		return String.fromCharCode('A'.charCodeAt(0) + num);
	}

	let address = '';

	while (colIndex > 0) {
		colIndex -= 1;
		address = base26digit(colIndex % 26) + address;
		colIndex = Math.floor(colIndex / 26);
	}

	return address;
}

export function getCellAddress(rowIndex: number, colIndex: number): string {
	return `${getColAddress(colIndex)}${getRowAddress(rowIndex)}`;
}

export async function getFormula(address: string): Promise<string> {
	return invoke('get_formula', { address });
}

export async function setFormula(
	address: string,
	formula: string,
): Promise<{ [address: Address]: AnyValue }> {
	return invoke('set_formula', { address, formula });
}
