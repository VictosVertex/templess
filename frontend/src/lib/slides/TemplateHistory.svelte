<script lang="ts">
	import type { ClassResponse, Template } from '$lib/types';
	import { api } from '$lib/api';
	import { realmTheme, defaultRealmTheme } from '$lib/constants';
	import { Trash2, ArrowDownAZ } from 'lucide-svelte';

	let {
		classes,
		templates,
		onSelect
	}: { classes: ClassResponse[]; templates: Template[]; onSelect: (id: number) => void } = $props();
	let sortBy = $state<'name' | 'realm' | 'class'>('name');

	function getClassName(classId: number) {
		return classes.find((c) => c.id === classId)?.name || 'Unknown Class';
	}

	let sortedTemplates = $derived(
		[...templates].sort((a, b) => {
			if (sortBy === 'name') return a.name.localeCompare(b.name);
			if (sortBy === 'realm') {
				const realmA = classes.find((c) => c.id === a.class_id)?.realm_id || 0;
				const realmB = classes.find((c) => c.id === b.class_id)?.realm_id || 0;
				return realmA - realmB || a.name.localeCompare(b.name);
			}
			if (sortBy === 'class') {
				return (
					getClassName(a.class_id).localeCompare(getClassName(b.class_id)) ||
					a.name.localeCompare(b.name)
				);
			}
			return 0;
		})
	);

	async function handleDelete(id: number) {
		if (confirm('Are you sure you want to delete this template?')) {
			await api.deleteTemplate(id);
			window.location.reload();
		}
	}
</script>

<div class="flex w-full flex-col">
	<div class="mx-auto w-full max-w-4xl px-4">
		<div class="flex items-center justify-end pb-2">
			{#if templates.length > 0}
				<div
					class="flex items-center gap-2 text-foreground-secondary transition-colors hover:text-primary"
				>
					<ArrowDownAZ size={14} strokeWidth={1.5} />
					<select
						id="sort"
						bind:value={sortBy}
						class="cursor-pointer appearance-none bg-transparent font-display text-[10px] font-bold tracking-[0.2em] uppercase transition-colors outline-none"
					>
						<option value="name">Sort by Name</option>
						<option value="realm">Group by Realm</option>
						<option value="class">Group by Class</option>
					</select>
				</div>
			{/if}
		</div>

		<div
			class="mb-2 h-px w-full bg-linear-to-r from-transparent via-primary/20 to-transparent"
		></div>

		<div class="flex w-full flex-col">
			{#if templates.length === 0}
				<div class="flex flex-col items-center justify-center py-20 text-center opacity-60">
					<p
						class="font-display text-sm font-bold tracking-[0.2em] text-foreground-secondary uppercase"
					>
						No tempaltes exist yet
					</p>
					<p
						class="mt-3 font-display text-[10px] tracking-widest text-foreground-secondary/70 uppercase"
					>
						Create a new template above to begin.
					</p>
				</div>
			{:else}
				<div
					class="hidden items-center justify-between border-b border-primary/5 px-4 py-2 sm:flex"
				>
					<div class="flex flex-1 items-center gap-6">
						<div class="w-8"></div>
						<div class="grid flex-1 grid-cols-[2fr_1fr_1fr_1fr] gap-4">
							<span
								class="font-display text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Name</span
							>
							<span
								class="font-display text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Class</span
							>
							<span
								class="font-display text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Utility</span
							>
							<span
								class="text-right font-display text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Modified</span
							>
						</div>
					</div>
					<div class="w-8 pl-6"></div>
				</div>

				{#each sortedTemplates as template (template.id)}
					{@const templateClass = classes.find((c) => c.id === template.class_id)}
					{@const theme = realmTheme[templateClass?.realm_id || 0] || defaultRealmTheme}
					{@const Icon = theme.icon}

					<div
						class="group flex items-stretch justify-between border-b border-primary/10 transition-all focus-within:bg-primary/2 hover:bg-primary/2"
					>
						<button
							type="button"
							onclick={() => onSelect(template.id)}
							class="flex min-w-0 flex-1 cursor-pointer items-center gap-6 px-4 py-4 text-left outline-none"
						>
							<div
								class="flex h-8 w-8 shrink-0 items-center justify-center opacity-80 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100"
							>
								<Icon size={20} class={theme.color} strokeWidth={1.5} />
							</div>

							<div
								class="grid min-w-0 flex-1 grid-cols-1 items-baseline gap-1 sm:grid-cols-[2fr_1fr_1fr_1fr] sm:gap-4"
							>
								<span
									class="truncate font-display text-sm font-bold tracking-widest text-foreground transition-colors group-focus-within:text-primary group-hover:text-primary"
								>
									{template.name}
								</span>
								<span
									class="truncate font-display text-[10px] tracking-[0.2em] text-foreground uppercase"
								>
									{getClassName(template.class_id)}
								</span>
								<span class="hidden font-display text-xs tracking-wider text-foreground sm:block">
									---
								</span>
								<span
									class="hidden text-right font-display text-[10px] tracking-widest text-foreground uppercase sm:block"
								>
									Today
								</span>
							</div>
						</button>

						<div class="flex items-center justify-center px-4">
							<button
								type="button"
								onclick={(e) => {
									e.stopPropagation();
									handleDelete(template.id);
								}}
								class="cursor-pointer rounded-sm text-foreground-secondary/40 opacity-100 transition-all hover:text-error
                   focus:text-error focus-visible:opacity-100 focus-visible:ring-2 focus-visible:ring-error
                   focus-visible:outline-none active:scale-95 lg:opacity-0 lg:group-focus-within:opacity-100 lg:group-hover:opacity-100"
								title="Delete Template"
							>
								<Trash2 size={14} strokeWidth={1.5} />
							</button>
						</div>
					</div>
				{/each}
			{/if}
		</div>

		<div
			class="mb-2 h-px w-full bg-linear-to-r from-transparent via-primary/20 to-transparent"
		></div>
	</div>
</div>
