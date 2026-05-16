<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		children,
		onClick,
		title = '',
		variant = 'primary',
		class: className = ''
	}: {
		children: Snippet;
		onClick: (event: MouseEvent) => void;
		title?: string;
		variant?: 'primary' | 'secondary' | 'pill';
		class?: string;
	} = $props();

	const baseStyles =
		'flex cursor-pointer items-center justify-center gap-2 font-technical text-xs font-bold tracking-widest uppercase transition-all active:scale-[0.97]';

	let variantStyles = $derived.by(() => {
		switch (variant) {
			case 'primary':
				return 'rounded-sm outline outline-primary/50 bg-primary px-4 py-2.5 text-background hover:bg-primary hover:text-surface-lowest';
			case 'secondary':
				return 'rounded-sm border border-outline bg-surface-low px-4 py-2.5 text-foreground hover:border-primary/50 hover:text-primary';
			case 'pill':
				return 'rounded-full border border-outline/60 bg-surface-lowest px-4 py-1.5 text-foreground-secondary shadow-[inset_0_1px_3px_rgba(0,0,0,0.04)] hover:border-primary/50 hover:text-primary hover:shadow-sm';
			default:
				return '';
		}
	});
</script>

<button class="{baseStyles} {variantStyles} {className}" onclick={onClick} {title}>
	{@render children()}
</button>
