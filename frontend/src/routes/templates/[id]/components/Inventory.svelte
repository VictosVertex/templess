<script lang="ts">
	import type { TemplateBuilder } from '$lib/template-builder.svelte';
	import { INVENTORY_GROUPS, SLOT_NAMES } from '$lib/constants';
	import InventorySlot from './InventorySlot.svelte';

	const { builder }: { builder: TemplateBuilder } = $props();

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

	// Some fake shit for now to see how the button looks
	let uiState = $state<'Idle' | 'Preparing' | 'Solving'>('Idle');

	function handleToggleOptimization() {
		if (uiState === 'Idle') {
			uiState = 'Preparing';
			setTimeout(() => (uiState = 'Solving'), 1000);
			setTimeout(() => (uiState = 'Idle'), 3000);
		} else {
			uiState = 'Idle';
		}
	}

	let buttonColor = $derived.by(() => {
		if (uiState === 'Idle') return 'border-primary text-primary bg-primary/20 hover:bg-primary/40';
		if (uiState === 'Preparing') return 'border-warning text-warning bg-warning/20';
		return 'border-success text-success bg-success/20 animate-pulse';
	});

	let buttonText = $derived.by(() => {
		if (uiState === 'Idle') return 'OPTIMIZE';
		if (uiState === 'Preparing') return 'PREPARING';
		return 'OPTIMIZING';
	});
</script>

<div class="mx-auto flex w-full max-w-4xl flex-col items-center">
	<div class="relative h-175 w-175">
		<button
			class="absolute top-1/2 left-1/2 z-10 flex h-32 w-32 -translate-x-1/2 -translate-y-1/2 transform cursor-pointer items-center justify-center rounded-full border-2 transition-all duration-300 ease-in-out {buttonColor}"
			onclick={handleToggleOptimization}
			disabled={uiState === 'Preparing'}
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
					hasItem={!!builder.equippedItems[slot]}
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
					hasItem={!!builder.equippedItems[slot]}
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
				hasItem={!!builder.equippedItems[slot]}
			/>
		{/each}
	</div>
</div>
