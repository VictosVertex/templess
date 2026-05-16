<script lang="ts">
	import { EquipSource } from '$lib/template-builder.svelte';
	import type { TemplateBuilder } from '$lib/template-builder.svelte';
	import { INVENTORY_GROUPS, SLOT_NAMES } from '$lib/constants';
	import InventorySlot from './InventorySlot.svelte';
	import type { Backend } from '$lib/backend.svelte';
	import {
		ClientMessageType,
		ItemSlot,
		OptimizationStatus,
		type ClientMessage,
		type StatDefinition
	} from '$lib/types';

	const {
		builder,
		backend,
		stats,
		onOpenSlot,
		startTimer,
		stopTimer,
		startFullTimer,
		stopFullTimer
	}: {
		builder: TemplateBuilder;
		backend: Backend;
		stats: Record<number, StatDefinition>;
		onOpenSlot: (slot: ItemSlot) => void;
		startTimer: () => void;
		stopTimer: () => void;
		startFullTimer: () => void;
		stopFullTimer: () => void;
	} = $props();

	const dimensions = 640;

	const center = { x: dimensions / 2, y: dimensions / 2 };
	const innerRadius = 140;
	const middleRadius = 260;

	type Point = { x: number; y: number };

	function getRadialPositions<T>(
		slots: T[],
		radius: number,
		center: Point,
		rotationOffset: number = 0
	) {
		const total = slots.length;

		return slots.map((slot, index) => {
			const angleDeg = ((index + rotationOffset) / total) * 360;
			const angleRad = angleDeg * (Math.PI / 180);

			return {
				slot,
				x: center.x + radius * Math.cos(angleRad),
				y: center.y + radius * Math.sin(angleRad)
			};
		});
	}

	const positionedJewelry = getRadialPositions(INVENTORY_GROUPS.jewelry, innerRadius, center, 0.5);
	const positionedArmor = getRadialPositions(INVENTORY_GROUPS.armor, middleRadius, center, 0.0);

	function handleToggleOptimization() {
		if (
			backend.optimizationStatus === OptimizationStatus.Ready ||
			backend.optimizationStatus === OptimizationStatus.Finished
		) {
			const equipped_items: Record<number, number> = {};
			for (const [slot, equippedItem] of Object.entries(builder.equippedItems)) {
				if (equippedItem?.source !== EquipSource.User) {
					continue;
				}

				equipped_items[parseInt(slot, 10)] = equippedItem.item.id;
			}

			const request: ClientMessage = {
				type: ClientMessageType.Start,
				data: {
					class_id: builder.getTemplateClass().id,
					equipped_items,
					preferences: builder.preferences
				}
			};

			if (startTimer) startTimer();
			if (startFullTimer) startFullTimer();
			backend.send(request);
		} else {
			if (stopTimer) stopTimer();
			if (stopFullTimer) stopFullTimer();
			backend.send({ type: ClientMessageType.Cancel });
		}
	}

	let buttonColor = $derived.by(() => {
		if (
			backend.optimizationStatus === OptimizationStatus.Ready ||
			backend.optimizationStatus === OptimizationStatus.Finished
		) {
			return 'bg-primary border-primary/20 text-surface-lowest hover:brightness-110 hover:shadow-lg';
		}

		if (
			backend.optimizationStatus === OptimizationStatus.Setup ||
			backend.optimizationStatus === OptimizationStatus.Grounding
		) {
			return 'bg-secondary border-secondary/20 text-foreground animate-pulse shadow-md';
		}

		return 'bg-error border-error/20 text-surface-lowest hover:brightness-110 hover:shadow-lg';
	});

	let buttonText = $derived.by(() => {
		if (backend.optimizationStatus === OptimizationStatus.Setup) return 'SETUP';
		if (backend.optimizationStatus === OptimizationStatus.Grounding) return 'GROUNDING';
		if (backend.optimizationStatus === OptimizationStatus.Solving) return 'CANCEL';
		if (backend.optimizationStatus === OptimizationStatus.Finished) return 'FINISHED';
		return 'OPTIMIZE';
	});

	let isRunning = $derived.by(() => {
		return (
			backend.optimizationStatus === OptimizationStatus.Setup ||
			backend.optimizationStatus === OptimizationStatus.Grounding ||
			backend.optimizationStatus === OptimizationStatus.Solving
		);
	});
</script>

<div class="flex w-full flex-col gap-12 pb-12">
	<div class="relative h-160 w-160">
		<div
			class="absolute bottom-8 left-1/2 mt-2 flex -translate-x-1/2 flex-col items-center justify-center"
		>
			<span class="mb-1 text-[10px] font-bold tracking-[0.2em] text-foreground-secondary uppercase">
				Total Utility
			</span>
			<span class="font-technical text-lg font-bold text-primary">
				{builder.totalUtility.toFixed(2)}
			</span>
		</div>
		<button
			class="absolute top-1/2 left-1/2 z-10 flex h-32 w-32 -translate-x-1/2 -translate-y-1/2 transform
			cursor-pointer items-center justify-center rounded-full border-[6px] {buttonColor}"
			onclick={handleToggleOptimization}
		>
			<div
				class="absolute inset-0.5 rounded-full border-2
				{isRunning
					? 'animate-spin border-surface-lowest/10 border-t-surface-lowest/80 border-r-transparent'
					: 'border-surface-lowest/20'}"
			></div>

			<span class="text-sm font-bold tracking-widest uppercase">
				{buttonText}
			</span>
		</button>
		{#each positionedJewelry as { slot, x, y } (slot)}
			<div
				class="absolute z-10 transition-all duration-200"
				style="left: 0; top: 0; transform: translate(calc({x}px - 50%), calc({y}px - 50%));"
			>
				<InventorySlot
					onclick={() => onOpenSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-full"
					width="w-[60px]"
					height="h-[60px]"
					itemSource={builder.equippedItems[slot]?.item.source ?? null}
					equipSource={builder.equippedItems[slot]?.source ?? null}
					gems={builder.equippedItems[slot]?.gems ?? []}
					{stats}
				/>
			</div>
		{/each}

		{#each positionedArmor as { slot, x, y } (slot)}
			<div
				class="absolute z-10 transition-all duration-200"
				style="left: 0; top: 0; transform: translate(calc({x}px - 50%), calc({y}px - 50%));"
			>
				<InventorySlot
					onclick={() => onOpenSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-b-full"
					width="w-[80px]"
					height="h-[80px]"
					itemSource={builder.equippedItems[slot]?.item.source ?? null}
					equipSource={builder.equippedItems[slot]?.source ?? null}
					gems={builder.equippedItems[slot]?.gems ?? []}
					{stats}
				/>
			</div>
		{/each}
	</div>

	<div class="flex items-center justify-center gap-12">
		{#each INVENTORY_GROUPS.weapons as slot (slot)}
			<InventorySlot
				onclick={() => onOpenSlot(slot)}
				onremove={() => builder.unequipItem(slot)}
				name={SLOT_NAMES[slot]}
				shapeClass="rounded-none"
				width="w-[80px]"
				height="h-[80px]"
				itemSource={builder.equippedItems[slot]?.item.source ?? null}
				equipSource={builder.equippedItems[slot]?.source ?? null}
				gems={builder.equippedItems[slot]?.gems ?? []}
				{stats}
			/>
		{/each}
	</div>
</div>
