<script lang="ts">
	import { fade } from 'svelte/transition';
	import type { Snippet } from 'svelte';
	import { X } from 'lucide-svelte';

	let {
		isOpen,
		onClose,
		title,
		children
	}: {
		isOpen: boolean;
		onClose: () => void;
		title?: string;
		children: Snippet;
	} = $props();

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && isOpen) onClose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center"
		transition:fade={{ duration: 150 }}
	>
		<button
			class="absolute inset-0 block h-full w-full cursor-default border-none bg-surface-lowest/40 backdrop-blur-md outline-none"
			onclick={onClose}
			aria-label="Close dialog"
			tabindex="-1"
		></button>

		<div
			class="relative z-10 flex max-h-[85vh] w-full max-w-4xl flex-col overflow-hidden rounded-sm bg-surface-high outline outline-outline"
			role="dialog"
			aria-modal="true"
			aria-labelledby={title ? 'modal-title' : undefined}
		>
			{#if title}
				<div class="flex items-center justify-between px-6 py-4">
					<h2
						id="modal-title"
						class="font-technical text-sm font-bold tracking-widest text-foreground uppercase"
					>
						{title}
					</h2>
					<button
						class="text-foreground-secondary transition-colors hover:text-foreground"
						onclick={onClose}
						aria-label="Close"
					>
						<X size={20} />
					</button>
				</div>
			{/if}

			<div class="flex-1 overflow-y-auto">
				{@render children()}
			</div>
		</div>
	</div>
{/if}
