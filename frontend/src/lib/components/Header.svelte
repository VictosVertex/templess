<script lang="ts">
	import { defaultRealmTheme, realmTheme } from '$lib/constants';

	let { templateName = null, className = null, realmId = null, showContext = false } = $props();

	let theme = $derived(realmTheme[realmId || 0] || defaultRealmTheme);
	let Icon = $derived(theme.icon);
</script>

<header class="relative z-10 w-full border-b border-primary/10 bg-primary shadow-md">
	<div class="mx-auto flex h-16 w-full max-w-400 items-center justify-between px-4 sm:px-8">
		<div class="flex items-center gap-5">
			<h1 class="font-display text-xl font-bold tracking-[0.25em] text-background uppercase">
				TempLess
			</h1>

			<div class="h-4 w-px bg-background/60"></div>

			<h2
				class="mt-0.5 hidden font-display text-[9px] font-bold tracking-[0.25em] text-background/90 uppercase sm:block"
			>
				ASP Powered Template Optimization
			</h2>
		</div>

		<div
			class="flex items-center gap-4 transition-all duration-500 {showContext && templateName
				? 'translate-y-0 opacity-100'
				: 'pointer-events-none -translate-y-2 opacity-0'}"
		>
			<span class="text-xs font-bold tracking-[0.2em] text-background uppercase"
				>{templateName}</span
			>

			{#if Icon}
				<Icon size={14} class="text-background" strokeWidth={1.5} />
			{/if}

			<span class="text-[10px] tracking-[0.25em] text-background/80 uppercase">{className}</span>
		</div>
	</div>
</header>
