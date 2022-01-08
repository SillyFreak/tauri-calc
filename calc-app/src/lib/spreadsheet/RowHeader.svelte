<script lang="ts">
	import * as calc from '$lib/calc';

	export let rowIndex = undefined;
</script>

<th class="border border-gray-300">
	{#await calc.getRowAddress(rowIndex)}
		...
	{:then value}
		{value}
	{/await}
</th>
