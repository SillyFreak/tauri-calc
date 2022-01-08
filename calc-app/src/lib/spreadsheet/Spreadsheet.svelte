<script lang="ts">
	import * as calc from '$lib/calc';

	import Cell from './Cell.svelte';
	import ColHeader from './ColHeader.svelte';
	import RowHeader from './RowHeader.svelte';

	const COLS = 4;
	const ROWS = 3;

	let currentCell = undefined;

	function* range(start, end, step = 1) {
		for (let i = start; i < end; i += step) {
			yield i;
		}
	}

	function setCurrentCell(rowIndex: number, colIndex: number) {
		const address = calc.getCellAddress(rowIndex, colIndex);
		const formula = '';
		currentCell = {
			rowIndex,
			colIndex,
			address,
			formula,
		};
	}
</script>

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
					<Cell {rowIndex} {colIndex} />
				{/each}
			</tr>
		{/each}
	</tbody>
</table>
