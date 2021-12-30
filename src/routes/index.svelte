<script lang="ts">
	import { toBase26 } from '$lib/base26';

	const COLS = 4;
	const ROWS = 3;

	function rowAddress(rowIndex: number): string {
		return `${rowIndex + 1}`;
	}

	function colAddress(colIndex: number): string {
		return toBase26(colIndex);
	}

	function cellAddress(rowIndex: number, colIndex: number): string {
		return `${colAddress(colIndex)}${rowAddress(rowIndex)}`;
	}

	let currentCell = undefined;

	function setCurrentCell(rowIndex: number, colIndex: number) {
		const address = cellAddress(rowIndex, colIndex);
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
		<input type="text" class="flex-none w-24" readonly placeholder="Address" value={currentCell?.address}>
		<input type="text" class="flex-1" placeholder="Formula/Value" value={currentCell?.formula}>
	</div>
	<div class="flex-1">
		<table class="border-collapse border border-gray-400">
			<thead>
				<tr>
					<th class="w-8 border border-gray-300"></th>
					{#each { length: COLS } as _val, colIndex}
						<th class="w-32 border border-gray-300">{colAddress(colIndex)}</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each { length: ROWS } as _val, rowIndex}
				<tr>
					<th class="border border-gray-300">{rowAddress(rowIndex)}</th>
					{#each { length: COLS } as _val, colIndex}
						<td class="border border-gray-300" on:focusin={() => setCurrentCell(rowIndex, colIndex)}>
							<div tabindex="0" class="m-0.5">
								({cellAddress(rowIndex, colIndex)})
							</div>
						</td>
					{/each}
				</tr>
			{/each}
			</tbody>
		</table>
	</div>
</div>
