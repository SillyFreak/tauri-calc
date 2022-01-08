<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import * as calc from '$lib/calc';

	export let rowIndex = undefined;
	export let colIndex = undefined;

	const dispatch = createEventDispatcher<{
		focusCell: { rowIndex: number; colIndex: number };
	}>();

	function focusCell(rowIndex: number, colIndex: number) {
		dispatch('focusCell', { rowIndex, colIndex });
	}
</script>

<td class="border border-gray-300" on:focusin={() => focusCell(rowIndex, colIndex)}>
	<div tabindex="0" class="m-0.5">...</div>
</td>
