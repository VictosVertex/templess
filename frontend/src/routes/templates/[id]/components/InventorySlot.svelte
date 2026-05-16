<script lang="ts">
	import { EquipSource } from '$lib/template-builder.svelte';
	import { ItemSource, type Gem, type StatDefinition } from '$lib/types';
	import { Plus, X } from 'lucide-svelte';

	let {
		name,
		shapeClass = 'rounded-b-full',
		width = 'w-[70px]',
		height = 'h-[70px]',
		itemSource = null,
		equipSource = null,
		gems = [],
		stats,
		onclick,
		onremove
	}: {
		name: string;
		shapeClass?: string;
		width?: string;
		height?: string;
		itemSource?: ItemSource | null;
		equipSource?: EquipSource | null;
		gems?: Gem[];
		stats: Record<number, StatDefinition>;
		onclick?: () => void;
		onremove?: (e: MouseEvent) => void;
	} = $props();

	let gemSummaries = $derived.by(() =>
		gems.map((gem) => ({
			key: `${gem.id}-${gem.tier}`,
			label: (stats[gem.stat_id]?.name ?? `stat_${gem.stat_id}`).replace(/_/g, ' '),
			value: gem.value,
			tier: gem.tier
		}))
	);

	let slotStyles = $derived.by(() => {
		if (equipSource === EquipSource.User) {
			return 'bg-primary border-2 border-primary outline outline-1 outline-offset-[-4px] outline-surface-lowest/30 shadow-md shadow-black/20 text-surface-lowest hover:brightness-110';
		}

		if (equipSource === EquipSource.Optimizer) {
			if (itemSource === ItemSource.Crafted) {
				return 'bg-success border-2 border-success outline-dashed outline-2 outline-offset-[-4px] outline-surface-lowest/70 shadow-md shadow-black/20 text-surface-lowest hover:brightness-110';
			}
			return 'bg-success border-2 border-success outline outline-1 outline-offset-[-4px] outline-surface-lowest/30 shadow-md shadow-black/20 text-surface-lowest hover:brightness-110';
		}

		return 'bg-surface-container border border-outline/50 shadow-[inset_0_4px_8px_rgba(0,0,0,0.08)] text-foreground-secondary/40 hover:border-primary/50 hover:text-primary';
	});
</script>

<div class="group flex flex-col items-center justify-center">
	<span class="mb-1 font-technical text-sm tracking-wider text-foreground-secondary capitalize">
		{name}
	</span>

	<div class="relative">
		<button
			{onclick}
			class="flex cursor-pointer items-center justify-center transition-all duration-200 {shapeClass} {slotStyles} {width} {height}"
			aria-label="Modify {name} slot"
		>
			<Plus size={24} />
		</button>

		{#if equipSource !== null}
			<div class="absolute -top-1 -right-1 z-10 hidden group-hover:flex">
				<button
					class="flex cursor-pointer items-center justify-center rounded-full border border-outline bg-surface-lowest p-1 transition-colors hover:text-error"
					onclick={onremove}
					title="Unequip Item"
					aria-label="Unequip {name}"
				>
					<X size={12} strokeWidth={3} />
				</button>
			</div>
		{/if}
	</div>

	{#if gemSummaries.length > 0}
		<div class="mt-2 flex max-w-28 flex-wrap justify-center gap-1">
			{#each gemSummaries as gem (gem.key)}
				<span
					class="max-w-full truncate rounded-full border border-success/40 bg-success/10 px-2 py-0.5 font-technical text-[10px] tracking-wide text-success uppercase"
					title={`+${gem.value} ${gem.label} (tier ${gem.tier + 1})`}
				>
					+{gem.value}
					{gem.label}
				</span>
			{/each}
		</div>
	{/if}
</div>
