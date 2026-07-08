<script lang="ts">
	import { SLOT_NAMES } from '$lib/constants';
	import type { TemplateBuilder } from '$lib/template-builder.svelte';
	import { type Gem, type ItemSlot } from '$lib/types';
	import { SvelteMap } from 'svelte/reactivity';

	let { builder }: { builder: TemplateBuilder } = $props();

	const GOLD_CURRENCY = 1;

	type ResolvedEntry = {
		slot: ItemSlot;
		slotName: string;
		itemName: string;
		price: number;
		currency: number;
		currencyLabel: string;
		gems: Gem[];
	};

	function isResolvedEntry(entry: ResolvedEntry | null): entry is ResolvedEntry {
		return entry !== null;
	}

	function formatCurrencyAmount(amount: number, currency: number): string {
		if (currency !== GOLD_CURRENCY) {
			return amount.toLocaleString();
		}

		const copper = amount % 100;
		const totalSilver = Math.floor(amount / 100);
		const silver = totalSilver % 100;
		const totalGold = Math.floor(totalSilver / 100);
		const gold = totalGold % 1000;
		const platinum = Math.floor(totalGold / 1000);

		return [
			`${platinum.toString().padStart(2, '0')}p`,
			`${gold.toString().padStart(3, '0')}g`,
			`${silver.toString().padStart(2, '0')}s`,
			`${copper.toString().padStart(2, '0')}c`
		].join(' ');
	}

	let checkoutEntries = $derived.by(() =>
		Object.entries(builder.resolvedEquipment)
			.map(([slotStr, equipment]) => {
				const slot = parseInt(slotStr, 10) as ItemSlot;
				if (!equipment) {
					return null;
				}

				return {
					slot,
					slotName: SLOT_NAMES[slot],
					itemName: equipment.item.name,
					price: equipment.item.price,
					currency: equipment.item.currency,
					currencyLabel: equipment.item.currency_label,
					gems: [...equipment.gems].sort((left, right) => left.id - right.id)
				} satisfies ResolvedEntry;
			})
			.filter(isResolvedEntry)
			.sort((left, right) => left.slot - right.slot)
	);

	let currencySummary = $derived.by(() => {
		const totals = new SvelteMap<number, { currency: number; label: string; total: number; items: ResolvedEntry[] }>();

		for (const entry of checkoutEntries) {
			if (!entry.price || entry.currency === 0) {
				continue;
			}

			const existing = totals.get(entry.currency) ?? {
				currency: entry.currency,
				label: entry.currencyLabel,
				total: 0,
				items: []
			};

			existing.total += entry.price;
			existing.items.push(entry);
			totals.set(entry.currency, existing);
		}

		return [...totals.values()].sort((left, right) => left.currency - right.currency);
	});

	let gemSummary = $derived.by(() =>
		checkoutEntries
			.filter((entry) => entry.gems.length > 0)
			.map((entry) => ({
				slot: entry.slot,
				slotName: entry.slotName,
				itemName: entry.itemName,
				gems: entry.gems.map((gem) => ({
					id: gem.id,
					label: (builder.stats[gem.stat_id]?.name ?? `stat_${gem.stat_id}`).replace(/_/g, ' '),
					value: gem.value,
					tier: gem.tier,
					ipCost: gem.ip_cost
				}))
			}))
	);
</script>

<div class="mx-auto flex w-full max-w-6xl flex-col gap-12 px-4 py-8 sm:px-8">
	<div class="grid w-full grid-cols-1 gap-12 xl:grid-cols-2 xl:gap-16">
		<section class="flex w-full flex-col gap-5">
			<div class="flex items-center gap-3 opacity-80">
				<div class="h-px flex-1 bg-linear-to-r from-transparent to-primary/40"></div>
				<h3 class="font-display text-[11px] font-bold tracking-[0.25em] text-primary uppercase">
					Currency
				</h3>
				<div class="h-px flex-1 bg-linear-to-l from-transparent to-primary/40"></div>
			</div>

			{#if currencySummary.length > 0}
				<div class="flex flex-col gap-6 px-4 sm:px-8">
					{#each currencySummary as currency (currency.currency)}
						<div class="flex flex-col gap-3">
							<div class="flex items-end justify-between gap-4 border-b border-outline/30 pb-2">
								<div>
									<p class="font-display text-sm tracking-widest text-foreground uppercase">
										{currency.label}
									</p>
									<p class="mt-1 text-xs text-foreground-secondary">
										{currency.items.length} {currency.items.length === 1 ? 'item' : 'items'}
									</p>
								</div>
								<p class="font-display text-xl text-primary">
									{formatCurrencyAmount(currency.total, currency.currency)}
								</p>
							</div>

							<div class="flex flex-col gap-2">
								{#each currency.items as item (item.slot)}
									<div class="flex items-center justify-between gap-4 text-sm">
										<div>
											<p class="font-display tracking-wide text-foreground uppercase">
												{item.slotName}
											</p>
											<p class="text-foreground-secondary">{item.itemName}</p>
										</div>
										<p class="font-display text-primary">
											{formatCurrencyAmount(item.price, item.currency)}
										</p>
									</div>
								{/each}
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="px-4 text-sm text-foreground-secondary sm:px-8">
					No equipped items currently contribute a priced currency requirement.
				</div>
			{/if}
		</section>

		<section class="flex w-full flex-col gap-5">
			<div class="flex items-center gap-3 opacity-80">
				<div class="h-px flex-1 bg-linear-to-r from-transparent to-primary/40"></div>
				<h3 class="font-display text-[11px] font-bold tracking-[0.25em] text-primary uppercase">
					Gems
				</h3>
				<div class="h-px flex-1 bg-linear-to-l from-transparent to-primary/40"></div>
			</div>

			{#if gemSummary.length > 0}
				<div class="flex flex-col gap-6 px-4 sm:px-8">
					{#each gemSummary as entry (entry.slot)}
						<div class="flex flex-col gap-3">
							<div class="border-b border-outline/30 pb-2">
								<p class="font-display text-sm tracking-widest text-foreground uppercase">
									{entry.slotName}
								</p>
								<p class="mt-1 text-sm text-foreground-secondary">{entry.itemName}</p>
							</div>

							<div class="flex flex-col gap-2">
								{#each entry.gems as gem (gem.id)}
									<div class="flex items-center justify-between gap-4 text-sm">
										<p class="text-foreground">
											<span class="font-display tracking-wide text-primary">+{gem.value}</span>
											 {gem.label}
										</p>
										<p class="font-reading text-xs tracking-[0.14em] text-foreground-secondary uppercase">
											Tier {gem.tier + 1}
										</p>
									</div>
								{/each}
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="px-4 text-sm text-foreground-secondary sm:px-8">
					No gems are currently used in the equipped template.
				</div>
			{/if}
		</section>
	</div>
</div>