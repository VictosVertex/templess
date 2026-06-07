<script lang="ts">
	import Modal from '$lib/components/Modal.svelte';
	import { SLOT_NAMES } from '$lib/constants';
	import { TemplateBuilder } from '$lib/template-builder.svelte';
	import { ItemSlot, type Item, type StatPreference } from '$lib/types';
	import Attributes from './components/Attributes.svelte';
	import Inventory from './components/Inventory.svelte';
	import ItemSelector from './components/ItemSelector.svelte';
	import Preferences from './components/Preferences.svelte';
	import { Backend } from '$lib/backend.svelte.js';
	import Button from '$lib/components/Button.svelte';
	import { History, Save, SlidersHorizontal } from 'lucide-svelte';

	let { builder, backend }: { builder: TemplateBuilder; backend: Backend } = $props();

	let activeSlot = $state<ItemSlot | null>(null);
	let isEditingPreferences = $state<boolean>(false);

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

	function handleOpenSlot(slot: ItemSlot) {
		activeSlot = slot;
	}

	function handleCloseItemSelection() {
		activeSlot = null;
	}

	function handleItemSelection(item: Item) {
		if (activeSlot !== null) {
			builder.equipItem(activeSlot, item.id);
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

<div class="mx-auto flex w-full max-w-400 flex-col items-center gap-10 px-4 py-8 sm:px-8">
	<div
		class="grid w-full grid-cols-1 items-start gap-12 lg:grid-cols-[320px_minmax(0,1fr)_320px] lg:gap-8 xl:gap-16"
	>
		<div class="flex w-full flex-col gap-12">
			<Attributes title="Base Stats" stats={builder.buckets.baseStats} />
			<Attributes title="Resists" stats={builder.buckets.resists} />
		</div>

		<div class="relative flex w-full flex-col items-center justify-start pt-2">
			<div class="absolute mb-8 flex items-center gap-6 opacity-90">
				<div
					class="flex items-center gap-3 rounded-sm border border-outline/30 bg-surface-low/50 px-4 py-1.5 backdrop-blur-sm"
				>
					<span
						class="font-display text-[10px] font-bold tracking-widest text-foreground-secondary uppercase"
						>Full</span
					>
					<span class="font-mono text-xs tracking-wider text-foreground">
						{(() => {
							const totalMs = Math.floor(fullTimer);
							const minutes = Math.floor(totalMs / 60000);
							const seconds = Math.floor((totalMs % 60000) / 1000);
							const ms = Math.floor((totalMs % 1000) / 10);
							return `${minutes}:${seconds.toString().padStart(2, '0')}:${ms.toString().padStart(2, '0')}`;
						})()}
					</span>
				</div>
				<div
					class="flex items-center gap-3 rounded-sm border border-primary/20 bg-primary/5 px-4 py-1.5 backdrop-blur-sm"
				>
					<span class="font-display text-[10px] font-bold tracking-widest text-primary/70 uppercase"
						>Model</span
					>
					<span class="font-mono text-xs tracking-wider text-primary">
						{(() => {
							const totalMs = Math.floor(modelTimer);
							const minutes = Math.floor(totalMs / 60000);
							const seconds = Math.floor((totalMs % 60000) / 1000);
							const ms = Math.floor((totalMs % 1000) / 10);
							return `${minutes}:${seconds.toString().padStart(2, '0')}:${ms.toString().padStart(2, '0')}`;
						})()}
					</span>
				</div>
			</div>

			<Inventory
				onOpenSlot={handleOpenSlot}
				{builder}
				{backend}
				stats={builder.stats}
				startTimer={startModelTimer}
				stopTimer={stopModelTimer}
				{startFullTimer}
				{stopFullTimer}
			/>

			<div class="mt-8 flex items-center justify-center gap-6">
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

		<div class="flex w-full flex-col gap-12">
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
		items={Object.values(builder.items)}
		targetSlot={activeSlot}
		currentItem={activeSlot !== null ? (builder.resolvedEquipment[activeSlot]?.item ?? null) : null}
		stats={builder.stats}
		onSelect={handleItemSelection}
	/>
</Modal>

<Modal
	isOpen={isEditingPreferences}
	onClose={handleClosePreferences}
	title="Customize stat preferences"
>
	<Preferences
		stats={Object.values(builder.stats)}
		template_class={builder.templateClass}
		onSave={handleSavePreferences}
		initialPreferences={builder.template.preferences}
	/>
</Modal>
