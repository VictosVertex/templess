<script lang="ts">
	import { Database, LoaderCircle } from 'lucide-svelte';
	import { api } from '$lib/api.js';
	import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';
	import Button from '$lib/components/Button.svelte';

	let isInitializing = $state(false);

	async function handleInitialize() {
		isInitializing = true;
		await api.initialize();
		window.location.reload();
	}
</script>

<div class="relative mt-20 flex w-full max-w-3xl flex-col items-center justify-center gap-10 px-4">
	<div class="flex items-center justify-center text-primary/60">
		<Database size={56} strokeWidth={1} />
	</div>

	<div class="flex w-full flex-col gap-8 text-center">
		<OrnamentHeader>Database Initialization</OrnamentHeader>

		<div class="mt-2 flex flex-col gap-3">
			<p
				class="font-technical text-sm font-bold tracking-[0.2em] text-foreground-secondary uppercase"
			>
				The database is currently uninitialized.
			</p>
			<p class="font-technical text-sm tracking-wider text-foreground-secondary/70">
				Make sure you have the necessary raw data in the
				<code class="border-b border-primary/20 px-1 pb-0.5 font-mono text-primary/80"
					>data/raw/</code
				>
				directory of the backend before initializing.
			</p>
		</div>
	</div>

	<Button onClick={handleInitialize} disabled={isInitializing} class="w-64">
		{#if isInitializing}
			<LoaderCircle size={16} class="mr-3 animate-spin" />
			<span class="animate-pulse">Parsing Records...</span>
		{:else}
			<span>Initialize Database</span>
		{/if}
	</Button>
</div>
