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

<div class="w-full h-full flex flex-col">
	<div class="flex-none flex flex-row">
		<input
			type="text"
			class="flex-none w-24"
			readonly
			placeholder="Address"
			value={currentCell?.address}
		/>
		<input type="text" class="flex-1" placeholder="Formula/Value" value={currentCell?.formula} />
	</div>
	<div class="flex-1">
		<Spreadsheet />
	</div>
</div>
