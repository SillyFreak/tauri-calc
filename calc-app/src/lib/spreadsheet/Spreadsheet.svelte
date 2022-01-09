<script lang="ts">
	import * as calc from '$lib/calc';

	import Cell, { FocusCellEvent } from './Cell.svelte';
	import ColHeader from './ColHeader.svelte';
	import RowHeader from './RowHeader.svelte';

	const COLS = 4;
	const ROWS = 3;

	let formulaInput: HTMLInputElement;

	let currentCell = undefined;
	let cellValues: { [address: calc.Address]: calc.Value } = {};

	function* range(start, end, step = 1) {
		for (let i = start; i < end; i += step) {
			yield i;
		}
	}

	async function setCurrentCell(event: FocusCellEvent) {
		const { address } = event.detail;
		const formula = await calc.getFormula(address);
		currentCell = {
			address,
			formula,
		};
	}

	async function submitEdit(event: Event) {
		currentCell.formula = formulaInput.value;

		const { address, formula } = currentCell;
		const cells = await calc.setFormula(address, formula);

		for (const [address, value] of Object.entries(cells)) {
			if (value.type === 'Empty') {
				delete cellValues[address];
			} else {
				cellValues[address] = value;
			}
		}
	}
</script>

<div class="flex-none flex flex-row">
	<input
		type="text"
		class="flex-none w-24"
		readonly
		placeholder="Address"
		value={currentCell?.address}
	/>
	<input
		bind:this={formulaInput}
		type="text"
		class="flex-1"
		placeholder="Formula/Value"
		value={currentCell?.formula}
		on:change={submitEdit}
	/>
</div>
<div class="flex-1">
	<table class="border-collapse border border-gray-400">
		<thead>
			<tr>
				<th class="w-8 border border-gray-300" />
				{#each [...range(1, COLS + 1)] as colIndex}
					<ColHeader address={calc.getColAddress(colIndex)} />
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each [...range(1, ROWS + 1)] as rowIndex}
				<tr class="h-4">
					<RowHeader address={calc.getRowAddress(rowIndex)} />
					{#each [...range(1, COLS + 1)] as colIndex}
						<Cell
							address={calc.getCellAddress(rowIndex, colIndex)}
							value={cellValues[calc.getCellAddress(rowIndex, colIndex)]}
							on:focusCell={setCurrentCell}
						/>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>
