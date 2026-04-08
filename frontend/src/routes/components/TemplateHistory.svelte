<script lang="ts">
	import type { ClassResponse, Template } from '$lib/types';
	import { api } from '$lib/api';
	import { realmTheme, defaultRealmTheme } from '$lib/constants';
	import { Trash2, ArrowDownAZ } from 'lucide-svelte';
	import { resolve } from '$app/paths';

	let { classes, templates }: { classes: ClassResponse[]; templates: Template[] } = $props();

	let sortBy = $state<'name' | 'realm' | 'class'>('name');

	function getClassName(classId: number) {
		return classes.find((c) => c.id === classId)?.name || 'Unknown Class';
	}

	let sortedTemplates = $derived(
		[...templates].sort((a, b) => {
			if (sortBy === 'name') {
				return a.name.localeCompare(b.name);
			}
			if (sortBy === 'realm') {
				const classA = classes.find((c) => c.id === a.class_id);
				const classB = classes.find((c) => c.id === b.class_id);

				const realmA = classA?.realm_id || 0;
				const realmB = classB?.realm_id || 0;

				return realmA - realmB || a.name.localeCompare(b.name);
			}
			if (sortBy === 'class') {
				const classA = getClassName(a.class_id);
				const classB = getClassName(b.class_id);
				return classA.localeCompare(classB) || a.name.localeCompare(b.name);
			}
			return 0;
		})
	);

	async function handleDelete(id: number) {
		await api.deleteTemplate(id);
		window.location.reload();
	}
</script>

<div class="mx-auto mt-12 flex w-full max-w-xl flex-col items-center gap-6">
	<div class="w-full space-y-2">
		<h1 class="font-technical text-3xl font-bold tracking-tight text-foreground uppercase">
			Template History
		</h1>
		<p class="text-sm text-foreground-secondary">
			View already finished templates or continue to optimize existing ones.
		</p>
	</div>

	<div
		class="flex w-full flex-col gap-4 rounded-sm bg-surface-container p-8 outline outline-outline sm:p-10"
	>
		{#if templates.length > 0}
			<div class="mb-2 flex items-center justify-end gap-2 border-b border-outline pb-4">
				<ArrowDownAZ size={16} class="text-foreground-secondary" strokeWidth={1.5} />
				<label for="sort" class="sr-only">Sort by</label>
				<select
					id="sort"
					bind:value={sortBy}
					class="cursor-pointer appearance-none bg-background font-technical text-sm font-bold
					tracking-widest text-foreground-secondary uppercase transition-colors
					outline-none"
				>
					<option value="name">Sort by Name</option>
					<option value="realm">Group by Realm</option>
					<option value="class">Group by Class</option>
				</select>
			</div>
		{/if}

		{#if templates.length === 0}
			<div
				class="flex flex-col items-center justify-center rounded-sm border border-dashed border-outline bg-surface-lowest py-12 text-center"
			>
				<p
					class="font-technical text-sm font-bold tracking-widest text-foreground-secondary uppercase"
				>
					No Templates Found
				</p>
				<p class="mt-1 text-xs text-foreground-secondary/50">
					Initialize a workspace to get started.
				</p>
			</div>
		{:else}
			{#each sortedTemplates as template (template.id)}
				{@const templateClass = classes.find((c) => c.id === template.class_id)}
				{@const theme = realmTheme[templateClass?.realm_id || 0] || defaultRealmTheme}
				{@const Icon = theme.icon}

				<a
					href={resolve('/templates/[id]', {
						id: template.id.toString()
					})}
					class="group flex items-center justify-between rounded-sm bg-surface-lowest p-4
                outline outline-outline transition-colors hover:outline-primary"
				>
					<div class="flex items-center gap-4">
						<div
							class="flex h-10 w-10 items-center justify-center rounded-sm border border-outline {theme.bg}"
						>
							<Icon size={20} class={theme.color} strokeWidth={1.5} />
						</div>

						<div class="flex flex-col">
							<span
								class="font-technical text-sm font-bold tracking-wide text-foreground transition-colors group-hover:text-primary"
							>
								{template.name}
							</span>
							<span
								class="font-technical text-xs tracking-widest text-foreground-secondary uppercase"
							>
								{getClassName(template.class_id)}
							</span>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<button
							onclick={() => handleDelete(template.id)}
							class="p-2 text-foreground-secondary transition-colors hover:text-red-400 active:scale-95"
							title="Delete Template"
						>
							<Trash2 size={16} strokeWidth={1.5} />
						</button>
					</div>
				</a>
			{/each}
		{/if}
	</div>
</div>
