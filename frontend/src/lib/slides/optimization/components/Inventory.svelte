<script lang="ts">
	import type { TemplateBuilder } from '$lib/template-builder.svelte';
	import { INVENTORY_GROUPS, SLOT_NAMES } from '$lib/constants';
	import InventorySlot from './InventorySlot.svelte';
	import type { Backend } from '$lib/backend.svelte';
	import {
		ClientMessageType,
		EquipSource,
		ItemSlot,
		OptimizationStatus,
		type ClientMessage,
		type StatDefinition
	} from '$lib/types';

	const {
		builder,
		backend,
		stats,
		hoveredStatId = null,
		onOpenSlot,
		startTimer,
		stopTimer,
		startFullTimer,
		stopFullTimer
	}: {
		builder: TemplateBuilder;
		backend: Backend;
		stats: Record<number, StatDefinition>;
		hoveredStatId?: number | null;
		onOpenSlot: (slot: ItemSlot) => void;
		startTimer: () => void;
		stopTimer: () => void;
		startFullTimer: () => void;
		stopFullTimer: () => void;
	} = $props();

	const innerRadiusPct = 21.875;
	const middleRadiusPct = 40.625;

	function getRadialPositions<T>(slots: T[], radiusPct: number, rotationOffset: number = 0) {
		const total = slots.length;

		return slots.map((slot, index) => {
			const angleDeg = ((index + rotationOffset) / total) * 360;
			const angleRad = angleDeg * (Math.PI / 180);

			return {
				slot,
				xPct: 50 + radiusPct * Math.cos(angleRad),
				yPct: 50 + radiusPct * Math.sin(angleRad)
			};
		});
	}

	const positionedJewelry = getRadialPositions(INVENTORY_GROUPS.jewelry, innerRadiusPct, 0.5);
	const positionedArmor = getRadialPositions(INVENTORY_GROUPS.armor, middleRadiusPct, 0.0);

	function handleToggleOptimization() {
		if (
			backend.optimizationStatus === OptimizationStatus.Ready ||
			backend.optimizationStatus === OptimizationStatus.Finished
		) {
			builder.clearOptimizationResults();

			const equipped_items: Record<number, number> = {};
			for (const [slotStr, state] of Object.entries(builder.template.equipped_items)) {
				if (state.source !== EquipSource.User) continue;
				equipped_items[parseInt(slotStr, 10)] = state.item_id;
			}

			const request: ClientMessage = {
				type: ClientMessageType.Start,
				data: {
					class_id: builder.templateClass.id,
					equipped_items,
					preferences: builder.template.preferences
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

	function getHighlightedValue(slot: ItemSlot): number | null {
		if (hoveredStatId === null) {
			return null;
		}

		const equipment = builder.resolvedEquipment[slot];
		if (!equipment) {
			return null;
		}

		const itemValue = equipment.item.bonuses[hoveredStatId] ?? 0;
		const gemValue = equipment.gems
			.filter((gem) => gem.stat_id === hoveredStatId)
			.reduce((sum, gem) => sum + gem.value, 0);

		const totalValue = itemValue + gemValue;
		return totalValue > 0 ? totalValue : null;
	}
</script>

<div
	class="@container relative mx-auto aspect-640/780 w-full max-w-[min(100%,640px,calc(75vh*640/780))]"
>
	<div class="absolute inset-x-0 top-0 aspect-square w-full">
		<div
			class="absolute bottom-[8%] left-1/2 flex -translate-x-1/2 flex-col items-center justify-center whitespace-nowrap"
		>
			<span
				class="mb-1 text-[clamp(9px,2cqi,10px)] font-bold tracking-[0.2em] text-foreground-secondary uppercase"
			>
				Total Utility
			</span>
			<span class="font-display text-[clamp(14px,4cqi,18px)] font-bold text-primary">
				{builder.totalUtility.toFixed(2)}
			</span>
		</div>

		<button
			class="absolute top-1/2 left-1/2 z-10 flex aspect-square w-[20cqi] -translate-x-1/2 -translate-y-1/2 transform
            cursor-pointer items-center justify-center rounded-full border-[clamp(2px,0.5cqi,6px)] {buttonColor}"
			onclick={handleToggleOptimization}
		>
			<div
				class="absolute inset-0.5 rounded-full border-2 {isRunning
					? 'animate-spin border-surface-lowest/10 border-t-surface-lowest/80 border-r-transparent'
					: 'border-surface-lowest/20'}"
			></div>
			<span class="text-[clamp(10px,2.5cqi,14px)] font-bold tracking-widest uppercase">
				{buttonText}
			</span>
		</button>

		{#each positionedJewelry as { slot, xPct, yPct } (slot)}
			<div
				class="absolute z-10 w-[9.375cqi] -translate-x-1/2 -translate-y-1/2 transition-all duration-200"
				style="left: {xPct}%; top: {yPct}%;"
			>
				<InventorySlot
					onclick={() => onOpenSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-full"
					width="w-full"
					height="aspect-square"
					highlightedValue={getHighlightedValue(slot)}
					itemSource={builder.resolvedEquipment[slot]?.item.source ?? null}
					equipSource={builder.resolvedEquipment[slot]?.source ?? null}
				/>
			</div>
		{/each}

		{#each positionedArmor as { slot, xPct, yPct } (slot)}
			<div
				class="absolute z-10 w-[12.5cqi] -translate-x-1/2 -translate-y-1/2 transition-all duration-200"
				style="left: {xPct}%; top: {yPct}%;"
			>
				<InventorySlot
					onclick={() => onOpenSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-b-full"
					width="w-full"
					height="aspect-square"
					highlightedValue={getHighlightedValue(slot)}
					itemSource={builder.resolvedEquipment[slot]?.item.source ?? null}
					equipSource={builder.resolvedEquipment[slot]?.source ?? null}
				/>
			</div>
		{/each}
	</div>

	<div class="absolute bottom-0 left-0 flex w-full items-center justify-center gap-[4cqi]">
		{#each INVENTORY_GROUPS.weapons as slot (slot)}
			<div class="w-[12.5cqi] shrink-0">
				<InventorySlot
					onclick={() => onOpenSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-none"
					width="w-full"
					height="aspect-square"
					highlightedValue={getHighlightedValue(slot)}
					itemSource={builder.resolvedEquipment[slot]?.item.source ?? null}
					equipSource={builder.resolvedEquipment[slot]?.source ?? null}
				/>
			</div>
		{/each}
	</div>
</div>
