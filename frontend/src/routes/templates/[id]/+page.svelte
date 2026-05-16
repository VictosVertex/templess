<script lang="ts">
	import { browser } from '$app/environment';
	import Modal from '$lib/components/Modal.svelte';
	import { SLOT_NAMES, defaultRealmTheme, realmTheme } from '$lib/constants';
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
	import { Backend } from '$lib/backend.svelte.js';
	import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';
	import Button from '$lib/components/Button.svelte';
	import { History, Save, SlidersHorizontal } from 'lucide-svelte';

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

	let theme = $derived(realmTheme[templateClass?.realm_id || 0] || defaultRealmTheme);
	let Icon = $derived(theme.icon);

	const itemDictionary = $derived(Object.fromEntries(data.items.map((item) => [item.id, item])));
	const gemDictionary = $derived(Object.fromEntries(data.gems.map((gem) => [gem.id, gem])));
	const storageKey = $derived(`template-draft:${data.template.id}`);

	let modelTimer = $state<number>(0);
	let modelTimerInterval: ReturnType<typeof setInterval> | null = null;

	let fullTimer = $state<number>(0);
	let fullTimerInterval: ReturnType<typeof setInterval> | null = null;

	function startModelTimer() {
		modelTimer = 0;
		if (modelTimerInterval) clearInterval(modelTimerInterval);
		let last = performance.now();
		modelTimerInterval = setInterval(() => {
			const now = performance.now();
			modelTimer += now - last;
			last = now;
		}, 16);
	}

	function resetModelTimer() {
		modelTimer = 0;
	}

	function stopModelTimer() {
		if (modelTimerInterval) {
			clearInterval(modelTimerInterval);
			modelTimerInterval = null;
		}
	}

	function startFullTimer() {
		fullTimer = 0;
		if (fullTimerInterval) clearInterval(fullTimerInterval);
		let last = performance.now();
		fullTimerInterval = setInterval(() => {
			const now = performance.now();
			fullTimer += now - last;
			last = now;
		}, 16);
	}

	function stopFullTimer() {
		if (fullTimerInterval) {
			clearInterval(fullTimerInterval);
			fullTimerInterval = null;
		}
	}

	const backend = new Backend((result: OptimizationResult) => {
		resetModelTimer();
		startModelTimer();
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

<div class="mx-auto flex w-fit flex-col items-center gap-2">
	<div class="my-4 w-full">
		<OrnamentHeader>
			<div class="flex flex-row gap-2">
				<b>{data.template.name}</b>
				<Icon size={20} class={theme.color} strokeWidth={1.5} />
				{templateClass.name}
			</div>
		</OrnamentHeader>
	</div>

	<div class="grid w-full grid-cols-1 items-start gap-20 lg:grid-cols-[1fr_auto_1fr]">
		<div class="flex w-80 flex-col gap-16">
			<Attributes title="Base Stats" stats={builder.buckets.baseStats} />
			<Attributes title="Resists" stats={builder.buckets.resists} />
		</div>

		<div class="relative flex w-full flex-col justify-center">
			<div class="absolute top-2 left-1/2 z-20 flex -translate-x-1/2 flex-col items-center gap-1">
				<div class="bg-surface/80 rounded px-3 py-1 shadow">
					<span class="font-mono text-base"
						>Full: {(() => {
							const totalMs = Math.floor(fullTimer);
							const minutes = Math.floor(totalMs / 60000);
							const seconds = Math.floor((totalMs % 60000) / 1000);
							const ms = Math.floor((totalMs % 1000) / 10);
							return `${minutes}:${seconds.toString().padStart(2, '0')}:${ms.toString().padStart(2, '0')}`;
						})()}</span
					>
				</div>
				<div class="bg-surface/80 rounded px-3 py-1 shadow">
					<span class="font-mono text-base text-red-500"
						>Model: {(() => {
							const totalMs = Math.floor(modelTimer);
							const minutes = Math.floor(totalMs / 60000);
							const seconds = Math.floor((totalMs % 60000) / 1000);
							const ms = Math.floor((totalMs % 1000) / 10);
							return `${minutes}:${seconds.toString().padStart(2, '0')}:${ms.toString().padStart(2, '0')}`;
						})()}</span
					>
				</div>
			</div>
			<Inventory
				onOpenSlot={handleOpenSlot}
				{builder}
				{backend}
				stats={data.stats}
				startTimer={startModelTimer}
				stopTimer={stopModelTimer}
				{startFullTimer}
				{stopFullTimer}
			/>
			<div class="flex items-center justify-center gap-6">
				<Button variant="pill" onClick={() => {}}>
					<Save size={14} /> Save
				</Button>

				<Button variant="pill" onClick={handleOpenPreferences}>
					<SlidersHorizontal size={14} /> Preferences
				</Button>

				<Button variant="pill" onClick={() => {}}>
					<History size={14} /> History
				</Button>
			</div>
		</div>

		<div class="flex h-full w-80 flex-col gap-16">
			<Attributes title="Skills" stats={builder.buckets.skills} />
			<Attributes
				title="Bonuses"
				stats={builder.buckets.bonuses.filter((stat) => stat.value !== 0)}
			/>
		</div>
	</div>

	<div class="my-4 mt-50 w-full">
		<OrnamentHeader>
			<div class="flex flex-row gap-2">Crafting Overview</div>
		</OrnamentHeader>
	</div>
	<div class="flex h-full w-80 flex-col gap-16">
		<Attributes title="Skills" stats={builder.buckets.skills} />
		<Attributes
			title="Bonuses"
			stats={builder.buckets.bonuses.filter((stat) => stat.value !== 0)}
		/>
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
