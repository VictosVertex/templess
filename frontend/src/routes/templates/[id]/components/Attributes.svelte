<script lang="ts">
	import Panel from '$lib/components/Panel.svelte';
	import type { Stat } from '$lib/types';
	import Attribute from './Attribute.svelte';

	let { title, stats }: { title: string; stats: Stat[] } = $props();
</script>

<Panel>
	<h3 class="mb-4 text-xs font-bold tracking-wider text-foreground-secondary/50 uppercase">
		{title}
	</h3>

	<div class="flex flex-col gap-4">
		{#each stats as stat (stat)}
			<Attribute {stat} />
		{/each}
	</div>
</Panel>
