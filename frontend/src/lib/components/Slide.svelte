<script lang="ts">
	import { viewport } from '$lib/viewport';
	import { nav, AppSlide } from '$lib/navigation.svelte';
	import type { Snippet } from 'svelte';

	let {
		index,
		children
	}: {
		index: AppSlide;
		children: Snippet;
	} = $props();

	let baseClasses =
		'snap-section flex h-[100dvh] w-full snap-start flex-col items-center justify-center p-4 md:p-40';
	let borderedClasses = 'border-t border-primary/10 pt-16';

	let style = $derived.by(() => {
		if (index === AppSlide.Hero) return baseClasses;
		if (index > AppSlide.Hero) return `${baseClasses} ${borderedClasses}`;

		return `snap-section flex h-[100dvh] w-full snap-start flex-col items-center justify-center`;
	});
</script>

<section id="slide-{index}" use:viewport={() => nav.updateFromScroll(index)} class={style}>
	{@render children()}
</section>
