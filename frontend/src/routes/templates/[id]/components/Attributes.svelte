<script lang="ts">
	import type { Stat } from '$lib/types';
	import Attribute from './Attribute.svelte';

	let { title, stats }: { title: string; stats: Stat[] } = $props();
</script>

<div class="rounded-sm bg-surface-lowest p-4 outline outline-outline backdrop-blur-md">
	<h3 class="mb-4 text-xs font-bold tracking-wider text-foreground-secondary/50 uppercase">
		{title}
	</h3>

	<div class="flex flex-col gap-3">
		{#each stats as stat (stat)}
			<Attribute name={stat.name} value={stat.value} cap={stat.currentCap} min={stat.min} />
		{/each}
	</div>
</div>
