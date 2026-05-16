<script lang="ts">
	import OrnamentHeader from '$lib/components/OrnamentHeader.svelte';
	import Panel from '$lib/components/Panel.svelte';
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

	async function handleCreate(e: Event) {
		e.preventDefault();
		isCreating = true;
		await fetch(`${API_BASE_URL}/templates`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				name: templateName,
				class_id: selectedClassId
			})
		});

		window.location.reload();
	}
</script>

<div class="mx-auto mt-12 flex w-full max-w-xl flex-col items-center gap-6">
	<OrnamentHeader>Create A New Template</OrnamentHeader>

	<Panel>
		<form onsubmit={handleCreate} class="flex flex-col gap-8">
			<div class="flex flex-col gap-3">
				<p
					class="font-technical text-sm font-bold tracking-widest text-foreground-secondary uppercase"
				>
					1. Select Realm
				</p>

				<div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
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
								class="flex flex-col items-center gap-3 rounded-sm bg-surface-low p-3 outline outline-outline transition-all
								peer-checked:outline-2 peer-checked:outline-primary hover:outline-primary"
							>
								<Icon size={24} class={color} strokeWidth={1.5} />

								<span
									class="font-technical text-sm font-bold tracking-wider {selectedRealmId ===
									realm.id
										? 'text-primary'
										: 'text-foreground'}"
								>
									{realm.name}
								</span>

								{#if selectedRealmId === realm.id}
									<div class="absolute top-3 right-3 h-1.5 w-1.5 rotate-45 bg-primary"></div>
								{/if}
							</div>
						</label>
					{/each}
				</div>
			</div>

			<div class="flex flex-col gap-3">
				<label
					for="class-select"
					class="font-technical text-sm font-bold tracking-widest text-foreground-secondary uppercase"
				>
					2. Select Class
				</label>
				<div class="relative">
					<select
						id="class-select"
						bind:value={selectedClassId}
						disabled={!selectedRealmId}
						class="w-full appearance-none rounded-sm border border-outline bg-surface-low px-4 py-3 text-foreground transition-colors focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
					>
						<option value={null}>
							{selectedRealmId ? 'Choose a class...' : 'Select a realm first...'}
						</option>
						{#each filteredClasses as cls (cls.id)}
							<option value={cls.id}>{cls.name}</option>
						{/each}
					</select>
					<div
						class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-4 text-foreground-secondary"
					>
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
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

			<div class="flex flex-col gap-3">
				<label
					for="template-name"
					class="font-technical text-sm font-bold tracking-widest text-foreground-secondary uppercase"
				>
					3. Template Name
				</label>
				<input
					type="text"
					id="template-name"
					bind:value={templateName}
					class="w-full rounded-sm border border-outline bg-surface-low px-4 py-3 text-foreground transition-colors
					placeholder:text-foreground-secondary/50 focus:border-primary focus:bg-surface-low focus:ring-1
					focus:ring-primary focus:outline-none"
					placeholder="e.g. Solo Bard"
				/>
			</div>

			<button
				type="submit"
				disabled={!selectedClassId || !templateName.trim()}
				class="group relative mt-2 inline-flex w-full cursor-pointer items-center justify-center overflow-hidden rounded-sm bg-primary px-8 py-4 font-technical text-sm font-bold tracking-widest text-background uppercase transition-all hover:brightness-110 active:scale-[0.98] disabled:cursor-not-allowed disabled:opacity-50 disabled:saturate-50 disabled:hover:brightness-100"
			>
				{#if isCreating}
					<LoaderCircle size={18} class="mr-3 animate-spin" />
					<span class="animate-pulse">Creating...</span>
				{:else}
					<span>New Template</span>
				{/if}
			</button>
		</form>
	</Panel>
</div>
