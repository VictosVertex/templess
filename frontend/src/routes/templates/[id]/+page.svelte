<script lang="ts">
	import Modal from '$lib/components/Modal.svelte';
	import { SLOT_NAMES } from '$lib/constants';
	import { EquipSource, TemplateBuilder } from '$lib/template-builder.svelte';
	import { ItemSlot, type Item } from '$lib/types';
	import { onMount } from 'svelte';
	import Attributes from './components/Attributes.svelte';
	import Inventory from './components/Inventory.svelte';
	import ItemSelector from './components/ItemSelector.svelte';
	import TemplateHeader from './components/TemplateHeader.svelte';
	import { Backend } from '$lib/backend.svelte.js';

	let { data } = $props();

	let templateClass = $derived.by(() => {
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

	const backend = new Backend((rawItems) => {
		const inflatedItems: Partial<Record<ItemSlot, Item>> = {};

		for (const [slotStr, itemId] of Object.entries(rawItems)) {
			const slot = parseInt(slotStr, 10) as ItemSlot;
			const fullItem = itemDictionary[itemId];

			if (fullItem) {
				inflatedItems[slot] = fullItem;
			}
		}

		builder.applyOptimizationResult(inflatedItems);
	});

	onMount(() => {
		backend.connect();
		return () => backend.disconnect();
	});
</script>

<div class="mx-auto flex w-full max-w-7xl flex-col items-center gap-2 px-4 pb-12">
	<TemplateHeader template={data.template} classes={data.classes} />

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

			<Inventory {builder} {backend} />
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
	isOpen={builder.activeSlot !== null}
	close={() => builder.closeModal()}
	title="Select an Item for {SLOT_NAMES[builder.activeSlot || ItemSlot.Chest]}"
>
	<ItemSelector
		items={data.items.filter((item) => item.item_slot_id == builder.activeSlot) || []}
		stats={data.stats}
		onSelect={(item) => builder.equipItem(item, EquipSource.User)}
	/>
</Modal>
