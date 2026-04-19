<script lang="ts">
	import { defaultRealmTheme, realmTheme } from '$lib/constants';
	import type { ClassResponse, Template } from '$lib/types';
	import { SlidersHorizontal } from 'lucide-svelte';

	let {
		template,
		classes,
		onOpenPreferences
	}: { template: Template; classes: ClassResponse[]; onOpenPreferences: () => void } = $props();

	let templateClass = $derived(classes.find((c) => c.id === template.class_id));

	let theme = $derived(realmTheme[templateClass?.realm_id || 0] || defaultRealmTheme);
	let Icon = $derived(theme.icon);
</script>

<div
	class="flex w-full items-center justify-between gap-4 rounded-sm bg-surface-container p-6 outline outline-outline"
>
	<div
		class="flex h-10 w-10 items-center justify-center rounded-sm border border-outline {theme.bg}"
	>
		<Icon size={20} class={theme.color} strokeWidth={1.5} />
	</div>
	<div class="flex w-full flex-col">
		<h1 class="font-technical text-2xl font-bold tracking-tight text-foreground uppercase">
			{template.name}
		</h1>
		<p class="mt-1 font-technical text-[10px] tracking-widest text-foreground-secondary uppercase">
			{templateClass?.name}
		</p>
	</div>
	<button
		class="ml-auto flex cursor-pointer items-center gap-2 rounded bg-primary/20 px-4 py-2 text-primary outline outline-primary/50 transition-all hover:bg-primary/50 hover:outline-primary/50"
		onclick={onOpenPreferences}
		title="Edit Optimization Goals"
	>
		<SlidersHorizontal size={18} />
		<span class="font-technical text-sm tracking-widest uppercase"> Preferences </span>
	</button>
</div>
