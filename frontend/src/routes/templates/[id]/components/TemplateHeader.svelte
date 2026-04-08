<script lang="ts">
	import { defaultRealmTheme, realmTheme } from '$lib/constants';
	import type { ClassResponse, Template } from '$lib/types';

	let { template, classes }: { template: Template; classes: ClassResponse[] } = $props();

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
</div>
