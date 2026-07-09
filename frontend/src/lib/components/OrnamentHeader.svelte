<script lang="ts">
	let { children } = $props();
</script>

<div class="my-8 flex w-full items-center justify-center gap-3 opacity-90">
	<div class="h-px flex-1 bg-linear-to-r from-transparent to-secondary/60"></div>

	<div class="h-1.5 w-1.5 rotate-45 bg-secondary/80"></div>

	<h2 class="px-1 text-sm font-medium tracking-[0.25em] text-foreground uppercase">
		{@render children()}
	</h2>

	<div class="h-1.5 w-1.5 rotate-45 bg-secondary/80"></div>

	<div class="h-px flex-1 bg-linear-to-l from-transparent to-secondary/60"></div>
</div>
