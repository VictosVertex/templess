<script lang="ts">
	import type { Snippet } from 'svelte';
	import OrnamentCorners from './OrnamentCorners.svelte';

	let {
		children,
		class: className = ''
	}: {
		children: Snippet;
		class?: string;
	} = $props();
</script>

<div class="relative w-full border border-outline bg-surface-lowest p-1 shadow-sm {className}">
	<div class="h-full border border-outline/50 p-4">
		{@render children()}
	</div>

	<OrnamentCorners />
</div>
