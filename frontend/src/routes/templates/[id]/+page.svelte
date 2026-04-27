<script lang="ts">
	import { browser } from '$app/environment';
	import Modal from '$lib/components/Modal.svelte';
	import { SLOT_NAMES } from '$lib/constants';
	import {
		EquipSource,
		TemplateBuilder,
		type TemplateBuilderSnapshot
	} from '$lib/template-builder.svelte';
	import { ItemSlot, type Item, type OptimizationResult, type StatPreference } from '$lib/types';
	import { onMount } from 'svelte';
	import Attributes from './components/Attributes.svelte';
	import Inventory from './components/Inventory.svelte';
	import ItemSelector from './components/ItemSelector.svelte';
	import Preferences from './components/Preferences.svelte';
	import TemplateHeader from './components/TemplateHeader.svelte';
	import { Backend } from '$lib/backend.svelte.js';

	let { data } = $props();

	let activeSlot = $state<ItemSlot | null>(null);
	let isEditingPreferences = $state<boolean>(false);

	const templateClass = $derived.by(() => {
		const foundClass = data.classes.find((c) => c.id === data.template.class_id);

		if (!foundClass) {
			throw new Error('Class not found for template');
		}

		return foundClass;
	});

	const builder = new TemplateBuilder(
		() => data.stats,
		() => templateClass
	);

	const itemDictionary = $derived(Object.fromEntries(data.items.map((item) => [item.id, item])));
	const gemDictionary = $derived(Object.fromEntries(data.gems.map((gem) => [gem.id, gem])));
	const storageKey = $derived(`template-draft:${data.template.id}`);

	const backend = new Backend((result: OptimizationResult) => {
		const inflatedItems: Partial<Record<ItemSlot, Item>> = {};
		const inflatedGems: Partial<Record<ItemSlot, typeof data.gems>> = {};

		for (const [slotStr, itemId] of Object.entries(result.equipped_items)) {
			const slot = parseInt(slotStr, 10) as ItemSlot;
			const fullItem = itemDictionary[itemId];

			if (fullItem) {
				inflatedItems[slot] = fullItem;
			}

			inflatedGems[slot] = (result.equipped_gems[itemId] ?? [])
				.map((gemId) => gemDictionary[gemId])
				.filter((gem) => gem !== undefined);
		}

		builder.applyOptimizationResult(inflatedItems, inflatedGems);
	});

	onMount(() => {
		if (browser) {
			const rawSnapshot = window.localStorage.getItem(storageKey);

			if (rawSnapshot) {
				try {
					builder.restoreSnapshot(
						JSON.parse(rawSnapshot) as TemplateBuilderSnapshot,
						itemDictionary
					);
				} catch (error) {
					console.warn('Failed to restore template draft from localStorage', error);
				}
			}
		}

		backend.connect();
		return () => backend.disconnect();
	});

	$effect(() => {
		if (!browser) {
			return;
		}

		window.localStorage.setItem(storageKey, JSON.stringify(builder.toSnapshot()));
	});

	function handleOpenSlot(slot: ItemSlot) {
		activeSlot = slot;
	}

	function handleCloseItemSelection() {
		activeSlot = null;
	}

	function handleItemSelection(item: Item) {
		if (activeSlot !== null) {
			builder.equipItem(activeSlot, item, EquipSource.User);
			handleCloseItemSelection();
		}
	}

	function handleClosePreferences() {
		isEditingPreferences = false;
	}

	function handleOpenPreferences() {
		isEditingPreferences = true;
	}

	function handleSavePreferences(preferences: Record<number, StatPreference>) {
		builder.setPreferences(preferences);
		handleClosePreferences();
	}
</script>

<div class="mx-auto flex w-full max-w-7xl flex-col items-center gap-2 px-4 pb-12">
	<TemplateHeader
		template={data.template}
		classes={data.classes}
		onOpenPreferences={handleOpenPreferences}
	/>

	<div class="mt-8 grid w-full grid-cols-1 items-start gap-8 lg:grid-cols-[1fr_auto_1fr]">
		<div class="flex w-full flex-col gap-6">
			<Attributes title="Base Stats" stats={builder.buckets.baseStats} />
			<Attributes title="Resists" stats={builder.buckets.resists} />
		</div>

		<div class="relative flex w-full justify-center">
			<div
				class="pointer-events-none absolute top-1/2 left-1/2 h-150 w-150 -translate-x-1/2
            -translate-y-1/2 rounded-full bg-primary/5 blur-[100px]"
			></div>

			<Inventory onOpenSlot={handleOpenSlot} {builder} {backend} stats={data.stats} />
		</div>

		<div class="flex w-full flex-col gap-6">
			<Attributes title="Skills" stats={builder.buckets.skills} />
			<Attributes
				title="Bonuses"
				stats={builder.buckets.bonuses.filter((stat) => stat.value !== 0)}
			/>
		</div>
	</div>
</div>

<Modal
	isOpen={activeSlot !== null}
	onClose={handleCloseItemSelection}
	title="Select an Item for {SLOT_NAMES[activeSlot || ItemSlot.Chest]}"
>
	<ItemSelector
		items={data.items}
		targetSlot={activeSlot}
		currentItem={activeSlot !== null ? (builder.equippedItems[activeSlot]?.item ?? null) : null}
		stats={data.stats}
		onSelect={handleItemSelection}
	/>
</Modal>

<Modal
	isOpen={isEditingPreferences}
	onClose={handleClosePreferences}
	title="Customize stat preferences"
>
	<Preferences
		stats={Object.values(data.stats)}
		template_class={builder.getTemplateClass()}
		onSave={handleSavePreferences}
		initialPreferences={builder.preferences}
	/>
</Modal>
