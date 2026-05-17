<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';
	import { API_BASE_URL, realmTheme } from '$lib/constants';
	import type { Realm, ClassResponse } from '$lib/types';
	import { LoaderCircle } from 'lucide-svelte';

	let { realms, classes }: { realms: Realm[]; classes: ClassResponse[] } = $props();

	let selectedRealmId = $state<number | null>(null);
	let selectedClassId = $state<number | null>(null);
	let templateName = $state('');
	let isCreating = $state(false);

	let filteredClasses = $derived(
		selectedRealmId ? classes.filter((c) => c.realm_id === Number(selectedRealmId)) : []
	);

	$effect(() => {
		if (selectedRealmId) selectedClassId = null;
	});

	async function handleCreate(e: Event) {
		e.preventDefault();
		isCreating = true;
		await fetch(`${API_BASE_URL}/templates`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ name: templateName, class_id: selectedClassId })
		});
		window.location.reload();
	}
</script>

<div class="flex w-full flex-col gap-10">
	<OrnamentHeader>Create A New Template</OrnamentHeader>

	<form onsubmit={handleCreate} class="flex w-full flex-col items-center gap-8 px-4">
		<div class="w-full max-w-4xl">
			<div class="grid grid-cols-1 gap-6 sm:grid-cols-3">
				{#each realms as realm (realm.id)}
					{@const Icon = realmTheme[realm.id].icon}
					{@const color = realmTheme[realm.id].color}
					<label class="group relative cursor-pointer">
						<input
							type="radio"
							name="realm"
							value={realm.id}
							bind:group={selectedRealmId}
							class="peer sr-only"
						/>
						<div
							class="flex flex-row items-center justify-center gap-4 rounded-sm border border-outline/40 bg-transparent px-6 py-5 transition-all peer-checked:border-primary/60 peer-checked:bg-primary/[0.03] hover:border-primary/40"
						>
							<Icon
								size={20}
								class="{color} transition-transform group-hover:scale-110"
								strokeWidth={1.5}
							/>
							<span
								class="font-technical text-sm font-bold tracking-widest text-foreground transition-colors peer-checked:text-primary"
							>
								{realm.name}
							</span>
							{#if selectedRealmId === realm.id}
								<div
									class="absolute top-1/2 right-4 h-1.5 w-1.5 -translate-y-1/2 rotate-45 bg-primary/80"
								></div>
							{/if}
						</div>
					</label>
				{/each}
			</div>
		</div>

		{#if selectedRealmId}
			<div class="w-full max-w-4xl pt-8">
				<div class="grid grid-cols-1 items-end gap-8 md:grid-cols-[1fr_1fr_240px]">
					<div class="flex flex-col gap-2">
						<label
							for="class-select"
							class="pl-1 font-technical text-[10px] font-bold tracking-[0.2em] text-foreground-secondary uppercase"
						>
							Select Class
						</label>
						<div class="relative">
							<select
								id="class-select"
								bind:value={selectedClassId}
								class="w-full appearance-none rounded-none border-b border-outline/40 bg-transparent px-2 py-2 font-technical text-sm tracking-wider text-foreground transition-colors focus:border-primary focus:outline-none"
							>
								<option value={null}>Choose a class...</option>
								{#each filteredClasses as cls (cls.id)}
									<option value={cls.id}>{cls.name}</option>
								{/each}
							</select>
							<div
								class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-foreground-secondary/50"
							>
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19 9l-7 7-7-7"
									></path></svg
								>
							</div>
						</div>
					</div>

					<div class="flex flex-col gap-2">
						<label
							for="template-name"
							class="pl-1 font-technical text-[10px] font-bold tracking-[0.2em] text-foreground-secondary uppercase"
						>
							Template Name
						</label>
						<input
							type="text"
							id="template-name"
							bind:value={templateName}
							class="w-full rounded-none border-b border-outline/40 bg-transparent px-2 py-2 font-technical text-sm tracking-wider text-foreground transition-colors placeholder:text-foreground-secondary/40 focus:border-primary focus:outline-none"
							placeholder="e.g. Solo Bard"
						/>
					</div>

					<Button
						disabled={!selectedClassId || !templateName.trim() || isCreating}
						class="h-9.5"
					>
						{#if isCreating}
							<LoaderCircle size={14} class="mr-2 animate-spin" />
							<span class="animate-pulse">Scribing...</span>
						{:else}
							<span>Create Template</span>
						{/if}
					</Button>
				</div>
			</div>
		{/if}
	</form>
</div>
