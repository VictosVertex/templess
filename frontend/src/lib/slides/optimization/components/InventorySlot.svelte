<script lang="ts">
	import { EquipSource, ItemSource } from '$lib/types';
	import { Plus, X } from 'lucide-svelte';

	let {
		name,
		shapeClass = 'rounded-b-full',
		width = 'w-[70px]',
		height = 'h-[70px]',
		itemSource = null,
		equipSource = null,
		highlightedValue = null,
		onclick,
		onremove
	}: {
		name: string;
		shapeClass?: string;
		width?: string;
		height?: string;
		itemSource?: ItemSource | null;
		equipSource?: EquipSource | null;
		highlightedValue?: number | null;
		onclick?: () => void;
		onremove?: (e: MouseEvent) => void;
	} = $props();

	let slotStyles = $derived.by(() => {
		if (equipSource === EquipSource.User) {
			return 'bg-primary border-2 border-primary outline outline-1 outline-offset-[-4px] outline-surface-lowest/30 shadow-md shadow-black/20 text-surface-lowest hover:brightness-110';
		}

		if (equipSource === EquipSource.Optimizer) {
			if (itemSource === ItemSource.Crafted) {
				return 'bg-success border-2 border-success outline-dashed outline-2 outline-offset-[-4px] outline-surface-lowest/70 shadow-md shadow-black/20 text-surface-lowest hover:brightness-110';
			}
			return 'bg-success border-2 border-success outline outline-1 outline-offset-[-4px] outline-surface-lowest/30 shadow-md shadow-black/20 text-surface-lowest hover:brightness-110';
		}

		return 'bg-surface-container border border-outline/50 shadow-[inset_0_4px_8px_rgba(0,0,0,0.08)] text-foreground-secondary/40 hover:border-primary/50 hover:text-primary';
	});
</script>

<div class="group flex flex-col items-center justify-center">
	<span
		class="mb-1 font-display text-[clamp(10px,2cqi,14px)] tracking-wider whitespace-nowrap text-foreground-secondary capitalize"
	>
		{name}
	</span>

	<div class="relative {width} {height}">
		<button
			{onclick}
			class="flex h-full w-full cursor-pointer items-center justify-center transition-all duration-200 {shapeClass} {slotStyles}"
			aria-label="Modify {name} slot"
		>
			{#if highlightedValue !== null}
				<span class="font-display text-[clamp(12px,2.8cqi,18px)] font-bold tracking-wide">
					{highlightedValue}
				</span>
			{:else if equipSource === null}
				<Plus size={24} />
			{/if}
		</button>

		{#if equipSource !== null}
			<div class="absolute -top-1 -right-1 z-10 hidden group-hover:flex">
				<button
					class="flex cursor-pointer items-center justify-center rounded-full border border-outline bg-surface-lowest p-1 transition-colors hover:text-error"
					onclick={onremove}
					title="Unequip Item"
					aria-label="Unequip {name}"
				>
					<X size={12} strokeWidth={3} />
				</button>
			</div>
		{/if}
	</div>
</div>
