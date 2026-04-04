<script lang="ts">
	let { name, value, cap } = $props();

	let fillPercentage = $derived.by(() => Math.min((value / cap) * 100, 100));

	let colorClass = $derived(
		value > cap ? 'text-red-400' : value === cap ? 'text-emerald-400' : 'text-gray-400'
	);

	let barColorClass = $derived(
		value > cap ? 'bg-red-500' : value === cap ? 'bg-emerald-400' : 'bg-indigo-500'
	);
</script>

<div class="flex flex-col gap-1">
	<div class="flex items-baseline justify-between">
		<span class="text-gray-300">{name}</span>
		<span class="font-mono text-xs {colorClass}">{value} / {cap}</span>
	</div>
	<div class="h-0.5 w-full overflow-hidden rounded-full bg-white/5">
		<div
			class="h-full rounded-full transition-all duration-300 {barColorClass}"
			style="width: {fillPercentage}%;"
		></div>
	</div>
</div>
