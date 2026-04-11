<script lang="ts">
	import type { TemplateBuilder } from '$lib/template-builder.svelte';
	import { INVENTORY_GROUPS, SLOT_NAMES } from '$lib/constants';
	import InventorySlot from './InventorySlot.svelte';
	import type { Backend } from '$lib/backend.svelte';
	import { ClientMessageType, OptimizationStatus, type ClientMessage } from '$lib/types';

	const { builder, backend }: { builder: TemplateBuilder; backend: Backend } = $props();

	const center = { x: 350, y: 350 };
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
			builder.userEquippedItems().forEach((item) => {
				equipped_items[item.item_slot_id] = item.id;
			});

			const request: ClientMessage = {
				type: ClientMessageType.Start,
				data: {
					class_id: builder.getTemplateClass().id,
					equipped_items
				}
			};

			backend.send(request);
		} else {
			backend.send({ type: ClientMessageType.Cancel });
		}
	}

	let buttonColor = $derived.by(() => {
		if (
			backend.optimizationStatus === OptimizationStatus.Ready ||
			backend.optimizationStatus === OptimizationStatus.Finished
		) {
			return 'border-primary text-primary bg-primary/20 hover:bg-primary/40';
		}

		if (
			backend.optimizationStatus === OptimizationStatus.Setup ||
			backend.optimizationStatus === OptimizationStatus.Grounding
		) {
			return 'border-warning text-warning bg-warning/20 animate-pulse';
		}

		return 'border-error text-error bg-error/20 hover:bg-error/30';
	});

	let buttonText = $derived.by(() => {
		if (backend.optimizationStatus === OptimizationStatus.Setup) return 'SETUP';
		if (backend.optimizationStatus === OptimizationStatus.Grounding) return 'GROUNDING';
		if (backend.optimizationStatus === OptimizationStatus.Solving) return 'CANCEL';
		if (backend.optimizationStatus === OptimizationStatus.Finished) return 'FINISHED';
		return 'OPTIMIZE';
	});
</script>

<div class="mx-auto flex w-full max-w-4xl flex-col items-center">
	<div class="relative h-175 w-175">
		<button
			class="absolute top-1/2 left-1/2 z-10 flex h-32 w-32 -translate-x-1/2 -translate-y-1/2 transform cursor-pointer items-center justify-center rounded-full border-2 transition-all duration-300 ease-in-out {buttonColor}"
			onclick={handleToggleOptimization}
		>
			<span class="text-sm font-bold tracking-widest uppercase">{buttonText}</span>
		</button>
		{#each positionedJewelry as { slot, x, y } (slot)}
			<div
				class="absolute z-10 transition-all duration-200"
				style="left: 0; top: 0; transform: translate(calc({x}px - 50%), calc({y}px - 50%));"
			>
				<InventorySlot
					onclick={() => builder.openSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-full"
					width="w-[60px]"
					height="h-[60px]"
					source={builder.equippedItems[slot]?.source ?? null}
				/>
			</div>
		{/each}

		{#each positionedArmor as { slot, x, y } (slot)}
			<div
				class="absolute z-10 transition-all duration-200"
				style="left: 0; top: 0; transform: translate(calc({x}px - 50%), calc({y}px - 50%));"
			>
				<InventorySlot
					onclick={() => builder.openSlot(slot)}
					onremove={() => builder.unequipItem(slot)}
					name={SLOT_NAMES[slot]}
					shapeClass="rounded-b-full"
					width="w-[80px]"
					height="h-[80px]"
					source={builder.equippedItems[slot]?.source ?? null}
				/>
			</div>
		{/each}
	</div>

	<div class="flex items-center justify-center gap-8">
		{#each INVENTORY_GROUPS.weapons as slot (slot)}
			<InventorySlot
				onclick={() => builder.openSlot(slot)}
				onremove={() => builder.unequipItem(slot)}
				name={SLOT_NAMES[slot]}
				shapeClass="rounded-none"
				width="w-[80px]"
				height="h-[80px]"
				source={builder.equippedItems[slot]?.source ?? null}
			/>
		{/each}
	</div>
</div>
