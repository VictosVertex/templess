<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import DatabaseInitializer from './components/DatabaseInitializer.svelte';
	let { data, children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="min-h-screen w-full bg-background text-foreground selection:bg-primary/30">
	{#if !data.isInitialized}
		<main class="flex min-h-screen w-full flex-col items-center justify-center">
			<DatabaseInitializer />
		</main>
	{:else}
		{@render children()}
	{/if}
</div>
