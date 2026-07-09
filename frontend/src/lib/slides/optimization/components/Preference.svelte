<script lang="ts">
	import type { StatDefinition, StatPreference } from '$lib/types';

	let {
		stat,
		capStat,
		preference,
		onChange
	}: {
		stat: StatDefinition;
		capStat?: StatDefinition;
		preference?: StatPreference;
		onChange: (updates: { id: number; min: number; weight: number }[]) => void;
	} = $props();

	let combinedMax = $derived(stat.cap + (capStat?.cap ?? 0));

	let currentMin = $derived(preference?.min ?? 0);
	let currentPriority = $derived(preference?.weight ?? 0);
	let isIgnored = $derived(currentPriority === 0);

	function handleChange() {
		const baseMin = currentMin;
		const overcapMin = capStat ? Math.max(0, currentMin - stat.cap) : 0;
		const updates = [{ id: stat.id, min: baseMin, weight: currentPriority }];

		if (capStat) {
			updates.push({ id: capStat.id, min: overcapMin, weight: currentPriority });
		}

		onChange(updates);
	}
</script>

<div
	class="flex items-center justify-between gap-6 border-b border-outline/40 bg-surface-lowest py-4 transition-colors last:border-b-0 hover:bg-surface-low/50"
>
	<div class="w-1/3">
		<span
			class="font-display text-sm font-bold tracking-wider text-foreground capitalize {isIgnored
				? 'opacity-40'
				: ''}"
		>
			{stat.name.replace(/_/g, ' ')}
		</span>
	</div>

	<div class="flex w-1/3 flex-col gap-2">
		<div
			class="flex justify-between font-display text-xs font-bold tracking-wider text-foreground-secondary uppercase"
		>
			<span>Min Target</span>
			<span class="text-foreground {isIgnored ? 'opacity-40' : ''}">
				{currentMin} / {combinedMax}
			</span>
		</div>
		<!-- <input
            type="range"
            min="0"
            max={combinedMax}
            bind:value={currentMin}
            onchange={handleChange}
            class="w-full accent-primary {isIgnored ? 'cursor-not-allowed opacity-40' : 'cursor-pointer'}"
            disabled={isIgnored}
        /> -->
		<input
			type="range"
			min="0"
			max={combinedMax}
			bind:value={currentMin}
			onchange={handleChange}
			disabled={isIgnored}
			class="slider-parchment slider-primary {isIgnored ? 'cursor-not-allowed opacity-40' : ''}"
			style="--progress: {combinedMax > 0 ? (currentMin / combinedMax) * 100 : 0}%"
		/>
	</div>

	<div class="flex w-1/3 flex-col gap-2">
		<div
			class="flex justify-between font-display text-xs font-bold tracking-wider text-foreground-secondary uppercase"
		>
			<span>Priority <span class="hidden xl:inline">(Weight)</span></span>
			<span class="text-foreground">{currentPriority}</span>
		</div>
		<input
			type="range"
			min="0"
			max="10"
			bind:value={currentPriority}
			onchange={handleChange}
			class="slider-parchment slider-success"
			style="--progress: {(currentPriority / 10) * 100}%"
		/>
	</div>
</div>
