<script lang="ts">
	import * as calc from '$lib/calc';

	import Spreadsheet from '$lib/spreadsheet/Spreadsheet.svelte';

	const COLS = 4;
	const ROWS = 3;

	let currentCell = undefined;

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

<div class="flex h-full w-full flex-col">
	<Spreadsheet />
</div>
