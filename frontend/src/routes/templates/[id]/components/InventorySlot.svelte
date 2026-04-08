<script lang="ts">
	import { Plus, X } from 'lucide-svelte';

	let {
		name,
		shapeClass = 'rounded-b-full',
		width = 'w-[70px]',
		height = 'h-[70px]',
		hasItem = false,
		onclick,
		onremove
	}: {
		name: string;
		shapeClass?: string;
		width?: string;
		height?: string;
		hasItem?: boolean;
		onclick?: () => void;
		onremove?: (e: MouseEvent) => void;
	} = $props();

	let stateClasses = $derived(
		hasItem
			? 'bg-primary/50 hover:bg-primary/20 hover:border-primary'
			: 'bg-surface-lowest hover:bg-surface-container hover:border-primary'
	);
</script>

<div class="group flex flex-col items-center justify-center">
	<span class="mb-1 font-technical text-sm tracking-wider text-foreground-secondary capitalize">
		{name}
	</span>

	<div class="relative">
		<button
			{onclick}
			class="flex cursor-pointer items-center justify-center border border-outline transition-all duration-200 {shapeClass} {stateClasses} {width} {height}"
			aria-label="Modify {name} slot"
		>
			<Plus size={24} class="text-foreground-secondary/50" />
		</button>

		{#if hasItem}
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
