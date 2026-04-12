<script lang="ts">
	import type { StatDefinition, StatPreference } from '$lib/types';

	let {
		stat,
		capStat,
		preference,
		capPreference,
		onChange
	}: {
		stat: StatDefinition;
		capStat?: StatDefinition;
		preference?: StatPreference;
		capPreference?: StatPreference;
		onChange: (updates: { id: number; min: number; weight: number }[]) => void;
	} = $props();

	let combinedMax = $derived(stat.cap + (capStat?.cap ?? 0));
	
		let initialMin = (preference?.min ?? 0) + (capPreference?.min ?? 0);

	let currentMin = $state(initialMin);
	let currentPriority = $state(preference?.weight ?? 0);
	let isIgnored = $derived(currentPriority === 0);

	function handleChange() {
		const baseMin = Math.min(currentMin, stat.cap);
		const overcapMin = capStat ? Math.max(0, currentMin - stat.cap) : 0;
		const updates = [{ id: stat.id, min: baseMin, weight: currentPriority }];

		if (capStat) {
			updates.push({ id: capStat.id, min: overcapMin, weight: currentPriority });
		}

		onChange(updates);
	}
</script>

<div
	class="flex items-center justify-between gap-4 rounded-lg border border-surface-container bg-surface-lowest p-3 transition-colors hover:border-primary/50"
>
	<div class="w-1/3">
		<span class="font-medium text-foreground capitalize {isIgnored ? 'opacity-50' : ''}">
			{stat.name.replace(/_/g, ' ')}
		</span>
	</div>

	<div class="flex w-1/3 flex-col gap-1">
		<div class="flex justify-between text-xs text-foreground-secondary">
			<span>Min Target</span>
			<span class="font-mono">{currentMin} / {combinedMax}</span>
		</div>
		<input
			type="range"
			min="0"
			max={combinedMax}
			bind:value={currentMin}
			onchange={handleChange}
			class="w-full accent-primary"
			disabled={isIgnored}
		/>
	</div>

	<div class="flex w-1/3 flex-col gap-1">
		<div class="flex justify-between text-xs text-foreground-secondary">
			<span>Priority (Weight)</span>
			<span class="font-mono">{currentPriority}</span>
		</div>
		<input
			type="range"
			min="0"
			max="10"
			bind:value={currentPriority}
			onchange={handleChange}
			class="w-full accent-warning"
		/>
	</div>
</div>
