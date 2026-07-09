<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		children,
		onClick,
		title = '',
		variant = 'primary',
		disabled = false,
		class: className = ''
	}: {
		children: Snippet;
		onClick?: (event: MouseEvent) => void;
		title?: string;
		variant?: 'primary' | 'secondary' | 'pill';
		disabled?: boolean;
		class?: string;
	} = $props();

	const baseStyles =
		'group relative inline-flex cursor-pointer items-center justify-center gap-2 overflow-hidden font-display text-xs font-bold tracking-widest uppercase transition-all active:scale-[0.98] disabled:cursor-not-allowed disabled:opacity-40';

	let variantStyles = $derived.by(() => {
		switch (variant) {
			case 'primary':
				return 'rounded-sm border border-primary bg-primary px-8 py-3.5 text-background hover:brightness-110 disabled:hover:brightness-100';

			case 'secondary':
				return 'rounded-sm border border-outline/30 bg-transparent px-8 py-3.5 text-foreground-secondary hover:border-primary/40 hover:text-primary hover:bg-primary/[0.02] disabled:hover:bg-transparent disabled:hover:text-foreground-secondary';

			case 'pill':
				return 'rounded-full border border-outline/30 bg-transparent px-5 py-2 text-foreground-secondary hover:border-primary/40 hover:text-primary hover:bg-primary/5';

			default:
				return '';
		}
	});
</script>

<button class="{baseStyles} {variantStyles} {className}" onclick={onClick} {title} {disabled}>
	{@render children()}
</button>
