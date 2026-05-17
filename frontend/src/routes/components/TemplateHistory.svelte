<script lang="ts">
	import type { ClassResponse, Template } from '$lib/types';
	import { api } from '$lib/api';
	import { realmTheme, defaultRealmTheme } from '$lib/constants';
	import { Trash2, ArrowDownAZ } from 'lucide-svelte';
	import { resolve } from '$app/paths';
	import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';

	let { classes, templates }: { classes: ClassResponse[]; templates: Template[] } = $props();
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

<div class="flex w-full flex-col gap-10">
	<OrnamentHeader>Template History</OrnamentHeader>

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
						class="cursor-pointer appearance-none bg-transparent font-technical text-[10px] font-bold tracking-[0.2em] uppercase transition-colors outline-none"
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
						class="font-technical text-sm font-bold tracking-[0.2em] text-foreground-secondary uppercase"
					>
						The ledger is empty
					</p>
					<p
						class="mt-3 font-technical text-[10px] tracking-widest text-foreground-secondary/70 uppercase"
					>
						Scribe a new template above to begin.
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
								class="font-technical text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Name</span
							>
							<span
								class="font-technical text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Class</span
							>
							<span
								class="font-technical text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
								>Utility</span
							>
							<span
								class="text-right font-technical text-[9px] font-bold tracking-[0.2em] text-foreground-secondary/50 uppercase"
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

					<a
						href={resolve('/templates/[id]', { id: template.id.toString() })}
						class="group flex items-center justify-between border-b border-primary/10 px-4 py-4 transition-all hover:bg-primary/[0.02]"
					>
						<div class="flex min-w-0 flex-1 items-center gap-6">
							<div
								class="flex h-8 w-8 shrink-0 items-center justify-center opacity-80 transition-opacity group-hover:opacity-100"
							>
								<Icon size={20} class={theme.color} strokeWidth={1.5} />
							</div>

							<div
								class="grid min-w-0 flex-1 grid-cols-1 items-baseline gap-1 sm:grid-cols-[2fr_1fr_1fr_1fr] sm:gap-4"
							>
								<span
									class="truncate font-technical text-sm font-bold tracking-widest text-foreground transition-colors group-hover:text-primary"
								>
									{template.name}
								</span>
								<span
									class="truncate font-technical text-[10px] tracking-[0.2em] text-foreground uppercase"
								>
									{getClassName(template.class_id)}
								</span>
								<span class="hidden font-technical text-xs tracking-wider text-foreground sm:block">
									---
								</span>
								<span
									class="hidden text-right font-technical text-[10px] tracking-widest text-foreground uppercase sm:block"
								>
									Today
								</span>
							</div>
						</div>

						<div class="flex items-center pl-6">
							<button
								onclick={(e) => {
									e.preventDefault();
									handleDelete(template.id);
								}}
								class="text-foreground-secondary/40 opacity-100 transition-all hover:text-error active:scale-95 lg:opacity-0 lg:group-hover:opacity-100"
								title="Delete Template"
							>
								<Trash2 size={14} strokeWidth={1.5} />
							</button>
						</div>
					</a>
				{/each}
			{/if}
		</div>

		<div
			class="mb-2 h-px w-full bg-linear-to-r from-transparent via-primary/20 to-transparent"
		></div>
	</div>
</div>
