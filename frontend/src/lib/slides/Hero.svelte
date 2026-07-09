<script lang="ts">
	import Button from '$lib/components/Button.svelte';

	let { onBegin }: { onBegin: () => void } = $props();
</script>

<div class="flex h-full w-full items-center justify-center p-4">
	<div
		class="relative flex w-full max-w-5xl flex-col items-center justify-center px-8 py-16 md:px-16 md:py-24"
	>
		<div
			class="pointer-events-none absolute top-0 left-0 h-12 w-12 border-t border-l border-primary/40 md:h-20 md:w-20"
		></div>

		<div
			class="pointer-events-none absolute right-0 bottom-0 h-12 w-12 border-r border-b border-primary/40 md:h-20 md:w-20"
		></div>

		<h1
			class="font-display text-5xl font-bold tracking-[0.2em] text-primary uppercase sm:text-7xl md:tracking-[0.3em] lg:text-8xl"
		>
			TempLess
		</h1>

		<div class="mt-8 mb-6 flex w-full max-w-xl items-center gap-2 opacity-80">
			<div
				class="h-px flex-1 bg-linear-to-r from-transparent via-secondary/20 to-secondary/80"
			></div>
			<div class="h-1.5 w-1.5 rotate-45 border border-secondary opacity-60"></div>
			<div class="h-2 w-2 rotate-45 bg-secondary"></div>
			<div class="h-1.5 w-1.5 rotate-45 border border-secondary opacity-60"></div>
			<div
				class="h-px flex-1 bg-linear-to-l from-transparent via-secondary/20 to-secondary/80"
			></div>
		</div>

		<div class="mb-12 flex flex-col items-center">
			<p
				class="font-reading text-[10px] font-bold tracking-[0.4em] text-foreground-secondary uppercase sm:text-xs"
			>
				ASP Powered Template Optimization
			</p>
		</div>

		<Button onClick={onBegin}>Begin</Button>
	</div>
</div>
