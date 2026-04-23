<script lang="ts">
	import { ItemSlot, StatCategory, type Item, type Stat, type StatDefinition } from '$lib/types';
	import { fade } from 'svelte/transition';

	let {
		items,
		targetSlot,
		stats,
		onSelect
	}: {
		items: Item[];
		targetSlot: ItemSlot | null;
		stats: Record<number, StatDefinition>;
		onSelect: (item: Item) => void;
	} = $props();

	const slotAliases: Partial<Record<ItemSlot, ItemSlot>> = {
		[ItemSlot.Ring2]: ItemSlot.Ring,
		[ItemSlot.Bracer2]: ItemSlot.Bracer
	};

	const selectableItems = $derived.by(() => {
		if (targetSlot === null) return [];
		return items.filter(
			(item) => item.item_slot_id === targetSlot || item.item_slot_id === slotAliases[targetSlot]
		);
	});

	let previewItem = $state<Item | null>(null);
	let sortedItems = $derived.by(() => {
		return selectableItems.slice().sort((a, b) => (b.utility || 0) - (a.utility || 0));
	});

	let previewStats = $derived.by(() => {
		let preview: Stat[] = [];

		if (previewItem && previewItem.bonuses) {
			for (let [statIdStr, value] of Object.entries(previewItem.bonuses)) {
				const statId = parseInt(statIdStr, 10);
				const statDef = stats[statId];

				if (statDef) {
					preview.push({
						...statDef,
						value,
						currentCap: statDef.cap,
						min: 0,
						weight: 0
					});
				} else {
					preview.push({
						id: statId,
						name: `Unknown Stat (${statId})`,
						cap: 0,
						utility: 0,
						category_id: StatCategory.OtherStats,
						base_stat_id: null,
						value,
						currentCap: 0,
						min: 0,
						weight: 0
					});
				}
			}
		}

		return preview;
	});
</script>

<div class="flex h-[60vh] min-h-125 w-full">
	<div class="flex min-h-0 w-[45%] flex-col border-r border-outline bg-surface-lowest">
		<div class="base-scrollbar flex-1 overflow-y-auto p-2">
			<div class="flex flex-col gap-1">
				{#each sortedItems as item (item.id)}
					<button
						class="flex w-full items-center justify-between rounded-sm px-4 py-3 text-left transition-all duration-200
                        {previewItem?.id === item.id
							? 'bg-primary/20 text-primary ring-1 ring-primary/50'
							: 'text-foreground-secondary hover:bg-primary/10 hover:text-foreground'}"
						onclick={() => (previewItem = item)}
					>
						<span class="truncate font-medium">{item.name}</span>
					</button>
				{/each}
			</div>
		</div>
	</div>

	<div class="flex min-h-0 w-[55%] flex-col overflow-hidden bg-surface-low">
		{#if previewItem}
			<div class="flex min-h-0 flex-1 flex-col p-8" transition:fade={{ duration: 150 }}>
				<div class="mb-8 shrink-0 border-b border-foreground/10 pb-6">
					<h2 class="text-2xl font-bold tracking-wide text-foreground">{previewItem.name}</h2>
					<p class="mt-1 text-sm tracking-wider text-foreground-secondary uppercase">
						Utility: {previewItem.utility?.toFixed(1)} (full) | {previewItem.utility_single?.toFixed(
							1
						)} (single)
					</p>
				</div>

				<div class="flex min-h-0 flex-1 flex-col">
					<h3
						class="mb-4 shrink-0 text-xs font-bold tracking-widest text-foreground-secondary uppercase"
					>
						Magical Bonuses
					</h3>
					<div class="base-scrollbar flex flex-1 flex-col gap-3 overflow-y-auto pr-2 pb-2">
						{#if previewStats.length > 0}
							{#each previewStats as stat (stat.id)}
								<div class="flex shrink-0 items-center justify-between px-2 py-1">
									<span class="text-foreground-secondary capitalize">
										{stat.name.replace(/_/g, ' ')}
									</span>
									<span class="text-primary">{stat.value}</span>
								</div>
							{/each}
						{:else}
							<div class="shrink-0 text-sm text-foreground-secondary italic">
								No magical bonuses on this item.
							</div>
						{/if}
					</div>
				</div>

				<div class="mt-8 shrink-0 pt-4">
					<button
						class="w-full rounded-sm border border-primary/50 bg-primary/20 py-4 text-sm font-bold tracking-widest text-primary
                        uppercase transition-all hover:bg-primary/50 hover:text-foreground active:scale-[0.98]"
						onclick={() => onSelect(previewItem!)}
					>
						Equip Item
					</button>
				</div>
			</div>
		{:else}
			<div class="flex h-full flex-col items-center justify-center gap-4 text-foreground-secondary">
				<p class="text-sm tracking-widest uppercase">Select an item to view details</p>
			</div>
		{/if}
	</div>
</div>
