<script lang="ts">
	let { name, value, cap, min = 10 } = $props();

	let fillPercentage = $derived(Math.min((value / cap) * 100, 100));
	let minPercentage = $derived(Math.min((min / cap) * 100, 100));

	let colorClass = $derived.by(() => {
		if (value >= cap) return 'text-success';
		if (min > 0 && value >= min) return 'text-warning';
		return 'text-foreground-secondary';
	});

	let barColorClass = $derived.by(() => {
		if (min > 0 && value < min) return 'bg-error';
		if (min > 0 && value >= min) return 'bg-success';
		return 'bg-primary';
	});
</script>

<div class="flex flex-col gap-1">
	<div class="flex items-baseline justify-between">
		<span class="text-foreground capitalize">{name.replace(/_/g, ' ')}</span>
		<span class="font-mono text-xs {colorClass}">{value} / {cap}</span>
	</div>
	<div class="relative h-0.5 w-full rounded-full bg-surface-container">
		<div
			class="h-full rounded-full transition-all duration-300 {barColorClass}"
			style="width: {fillPercentage}%;"
		></div>
		{#if min > 0}
			<div
				class="absolute top-1/2 h-2.5 w-0.5 -translate-x-1/2 -translate-y-1/2
				rounded-full transition-all duration-300
				{value >= min ? 'w-1 bg-success' : 'bg-primary'}"
				style="left: {minPercentage}%;"
			></div>
		{/if}
	</div>
</div>
