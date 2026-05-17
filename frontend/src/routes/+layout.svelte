<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import Header from '$lib/components/Header.svelte';
	import DatabaseInitializer from './components/DatabaseInitializer.svelte';
	let { data, children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div
	class="flex min-h-screen w-full flex-col bg-background text-foreground selection:bg-primary/30"
>
	<div class="z-50 w-full bg-background lg:sticky lg:top-0">
		<Header />
	</div>

	<main class="flex h-full w-full flex-1 flex-col items-center justify-start">
		{#if !data.isInitialized}
			<DatabaseInitializer />
		{:else}
			{@render children()}
		{/if}
	</main>
</div>
