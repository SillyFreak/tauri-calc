<script lang="ts">
	import * as calc from '$lib/calc';

	export let colIndex = undefined;
</script>

<th class="w-32 border border-gray-300">
	{#await calc.getColAddress(colIndex)}
		...
	{:then value}
		{value}
	{/await}
</th>
