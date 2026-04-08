<script lang="ts">
	let { name, value, cap } = $props();

	let fillPercentage = $derived.by(() => Math.min((value / cap) * 100, 100));

	let colorClass = $derived(
		value > cap ? 'text-error' : value === cap ? 'text-success' : 'text-foreground-secondary'
	);

	let barColorClass = $derived(
		value > cap ? 'bg-error' : value === cap ? 'bg-success' : 'bg-primary'
	);
</script>

<div class="flex flex-col gap-1">
	<div class="flex items-baseline justify-between">
		<span class="text-foreground capitalize">{name.replace(/_/g, ' ')}</span>
		<span class="font-mono text-xs {colorClass}">{value} / {cap}</span>
	</div>
	<div class="h-0.5 w-full overflow-hidden rounded-full bg-surface-container">
		<div
			class="h-full rounded-full transition-all duration-300 {barColorClass}"
			style="width: {fillPercentage}%;"
		></div>
	</div>
</div>
