<script lang="ts" context="module">
	type FocusCellPayload = { address: string };

	export type FocusCellEvent = CustomEvent<FocusCellPayload>;
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import type { Value } from '$lib/calc';

	export let address: string;
	export let value: Value;

	const dispatch = createEventDispatcher<{
		focusCell: FocusCellPayload;
	}>();

	function focusCell() {
		dispatch('focusCell', { address });
	}
</script>

<td class="h-inherit border border-gray-300" on:focusin={focusCell}>
	<div tabindex="0" class="m-0.5 h-full">
		{#if value !== undefined}
			{value.value}
		{/if}
	</div>
</td>
