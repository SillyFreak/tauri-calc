<script lang="ts">
	import * as calc from '$lib/calc';

	import Cell from './Cell.svelte';
	import ColHeader from './ColHeader.svelte';
	import RowHeader from './RowHeader.svelte';

	const COLS = 4;
	const ROWS = 3;

	let formulaInput: HTMLInputElement;

	let currentCell = undefined;

	function* range(start, end, step = 1) {
		for (let i = start; i < end; i += step) {
			yield i;
		}
	}

	async function setCurrentCell({
		detail: { rowIndex, colIndex },
	}: CustomEvent<{ rowIndex: number; colIndex: number }>) {
		const [address, formula] = await calc.getCell(rowIndex, colIndex);
		currentCell = {
			rowIndex,
			colIndex,
			address,
			formula,
		};
	}

	async function submitEdit(event: Event) {
		currentCell.formula = formulaInput.value;

		const { address, formula } = currentCell;
		console.log(address, formula);
		const cells = await calc.setCell(address, formula);
		console.log(cells);
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
					<ColHeader {colIndex} />
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each [...range(1, ROWS + 1)] as rowIndex}
				<tr>
					<RowHeader {rowIndex} />
					{#each [...range(1, COLS + 1)] as colIndex}
						<Cell {rowIndex} {colIndex} on:focusCell={setCurrentCell} />
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>
