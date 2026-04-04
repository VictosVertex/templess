<script lang="ts">
	import type { Stat } from '$lib/types';
	import Attribute from './Attribute.svelte';

	let { title, stats }: { title: string; stats: Stat[] } = $props();
</script>

<div class="rounded-lg border border-white/5 bg-black/20 p-4 backdrop-blur-md">
	<h3 class="mb-4 text-xs font-bold tracking-wider text-white/40 uppercase">{title}</h3>

	<div class="flex flex-col gap-3">
		{#each stats as stat (stat)}
			<Attribute name={stat.name} value={stat.value} cap={stat.cap} />
		{/each}
	</div>
</div>
