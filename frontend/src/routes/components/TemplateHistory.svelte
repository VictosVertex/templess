<script lang="ts">
	import type { ClassResponse, Template } from '$lib/types';
	import { api } from '$lib/api';
	import { realmTheme, defaultRealmTheme } from '$lib/constants';
	import { Trash2, ArrowDownAZ } from 'lucide-svelte';
	import { resolve } from '$app/paths';
	import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';
	import Panel from '$lib/components/Panel.svelte';

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
	<OrnamentHeader>Template History</OrnamentHeader>

	<Panel>
		<div class="flex w-full flex-col">
			{#if templates.length > 0}
				<div class="mb-2 flex items-center justify-end gap-2 border-b border-outline/50 pb-4">
					<ArrowDownAZ size={16} class="text-foreground-secondary" strokeWidth={1.5} />
					<label for="sort" class="sr-only">Sort by</label>
					<select
						id="sort"
						bind:value={sortBy}
						class="cursor-pointer appearance-none bg-transparent font-technical text-sm font-bold tracking-widest text-foreground-secondary uppercase transition-colors outline-none hover:text-primary focus:text-primary"
					>
						<option value="name">Sort by Name</option>
						<option value="realm">Group by Realm</option>
						<option value="class">Group by Class</option>
					</select>
				</div>
			{/if}

			{#if templates.length === 0}
				<div class="flex flex-col items-center justify-center py-12 text-center opacity-60">
					<p
						class="font-technical text-sm font-bold tracking-widest text-foreground-secondary uppercase"
					>
						No Templates Found
					</p>
					<p
						class="mt-2 font-technical text-xs tracking-wider text-foreground-secondary/70 uppercase"
					>
						Initialize a workspace to get started.
					</p>
				</div>
			{:else}
				{#each sortedTemplates as template (template.id)}
					{@const templateClass = classes.find((c) => c.id === template.class_id)}
					{@const theme = realmTheme[templateClass?.realm_id || 0] || defaultRealmTheme}
					{@const Icon = theme.icon}

					<a
						href={resolve('/templates/[id]', { id: template.id.toString() })}
						class="group -mx-2 flex items-center justify-between border-b border-outline/40 px-2 py-4 transition-colors last:border-b-0 hover:bg-surface-low/50"
					>
						<div class="flex items-center gap-4">
							<div
								class="flex h-10 w-10 shrink-0 items-center justify-center rounded-sm border border-outline/50 bg-surface-container shadow-[inset_0_1px_3px_rgba(0,0,0,0.06)]"
							>
								<Icon size={20} class={theme.color} strokeWidth={1.5} />
							</div>

							<div class="flex flex-col">
								<span
									class="font-technical text-sm font-bold tracking-wider text-foreground transition-colors group-hover:text-primary"
								>
									{template.name}
								</span>
								<span
									class="mt-0.5 font-technical text-xs tracking-widest text-foreground-secondary uppercase"
								>
									{getClassName(template.class_id)}
								</span>
							</div>
						</div>

						<div class="flex items-center gap-2">
							<button
								onclick={(e) => {
									e.preventDefault(); // Stop the <a> tag from navigating
									handleDelete(template.id);
								}}
								class="p-2 text-foreground-secondary opacity-0 transition-all group-hover:opacity-100 hover:text-error active:scale-95"
								title="Delete Template"
							>
								<Trash2 size={18} strokeWidth={1.5} />
							</button>
						</div>
					</a>
				{/each}
			{/if}
		</div>
	</Panel>
</div>
