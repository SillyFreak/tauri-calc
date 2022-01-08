<script lang="ts">
	import * as calc from '$lib/calc';

	export let rowIndex = undefined;
	export let colIndex = undefined;
</script>

<!-- on:focusin={() => setCurrentCell(rowIndex, colIndex)} -->
<td class="border border-gray-300">
	<div tabindex="0" class="m-0.5">...</div>
</td>
