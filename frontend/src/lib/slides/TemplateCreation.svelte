<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import { API_BASE_URL, realmTheme } from '$lib/constants';
	import type { Realm, ClassResponse } from '$lib/types';
	import { nav, AppSlide } from '$lib/navigation.svelte';
	import { LoaderCircle } from 'lucide-svelte';
	import OrnamentSeparator from '$lib/components/OrnamentSeparator.svelte';

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

<div
	class="flex w-full max-w-5xl flex-col items-center justify-center gap-12 max-[400px]:gap-3 lg:gap-24"
>
	<p
		class="max-w-lg text-center font-reading text-[10px] leading-relaxed font-bold tracking-[0.3em] text-foreground-secondary uppercase sm:text-xs"
	>
		Create a new optimization template by selecting a realm, class and name.
	</p>
	<form onsubmit={handleCreate} class="flex w-full flex-col gap-2 md:gap-10">
		<div class="grid w-full grid-cols-3 gap-3 sm:grid-cols-3 md:gap-6">
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
						class="
                    flex flex-col items-center justify-center gap-3 rounded-xl border border-outline bg-transparent px-6 py-4 transition-all
                    peer-checked:border-primary peer-checked:bg-primary/10 hover:border-primary/50
                    sm:h-36 sm:gap-4 sm:px-4 sm:py-6 md:h-44
                    "
					>
						<Icon size={26} class="{color} opacity-60 sm:size-10" strokeWidth={1.5} />

						<span
							class="font-display text-base tracking-widest text-foreground-secondary uppercase {selectedRealmId ===
							realm.id
								? 'text-primary'
								: 'text-foreground'}"
						>
							<div class="flex max-[400px]:hidden">
								{realm.name}
							</div>
							<div class="hidden max-[400px]:flex">
								{realm.name.slice(0, 3).toUpperCase()}
							</div>
						</span>

						{#if selectedRealmId === realm.id}
							<div class="absolute top-3 right-3 h-2 w-2 rotate-45 bg-primary"></div>
						{/if}
					</div>
				</label>
			{/each}
		</div>

		<div class="w-full max-[400px]:hidden">
			<OrnamentSeparator />
		</div>

		<div class="grid w-full grid-cols-1 items-end gap-6 lg:grid-cols-[1fr_1fr_auto] lg:gap-8">
			<div class="flex flex-col gap-2">
				<label
					for="class-select"
					class="font-reading text-[10px] font-bold tracking-[0.2em] text-primary/70 uppercase"
				>
					Class
				</label>
				<div class="relative">
					<select
						id="class-select"
						bind:value={selectedClassId}
						disabled={!selectedRealmId}
						class="w-full appearance-none rounded-sm border border-outline bg-transparent px-4 py-3
                    text-primary transition-colors focus:border-primary focus:ring-0 focus:ring-primary
                    focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
					>
						<option value={null}>
							{selectedRealmId ? 'Choose a class...' : 'Select a realm first...'}
						</option>
						{#each filteredClasses as cls (cls.id)}
							<option value={cls.id}>{cls.name}</option>
						{/each}
					</select>
				</div>
			</div>
			<div class="flex flex-col gap-2">
				<label
					for="template-name"
					class="font-reading text-[10px] font-bold tracking-[0.2em] text-primary/70 uppercase"
				>
					Template Name
				</label>
				<input
					type="text"
					id="template-name"
					bind:value={templateName}
					maxlength="60"
					class="w-full rounded-sm border-outline bg-transparent px-4 py-3 text-primary
                transition-colors placeholder:text-primary/20
                focus:border-primary focus:ring-0 focus:ring-primary focus:outline-none max-[400px]:text-base"
					placeholder="e.g. Solo Bard"
				/>
			</div>
			<div class="w-full pt-2 md:w-auto md:pt-0">
				<Button
					disabled={!selectedClassId || !templateName.trim() || isCreating}
					class="w-full px-10 py-3.5"
				>
					{#if isCreating}
						<LoaderCircle size={16} class="mr-3 animate-spin" />
						<span class="animate-pulse tracking-[0.2em]">Creating...</span>
					{:else}
						<span class="tracking-[0.2em]">NEW TEMPLATE</span>
					{/if}
				</Button>
			</div>
		</div>
	</form>

	<div class="mt-2 flex w-full items-center justify-center">
		<button
			type="button"
			onclick={() => nav.scrollTo(AppSlide.History)}
			class="group relative flex items-center justify-center gap-4 bg-transparent px-8 py-2 transition-all"
		>
			<div
				class="h-px w-6 bg-primary/40 transition-all duration-300 group-hover:w-10 group-hover:bg-primary"
			></div>
			<span
				class="font-reading text-[10px] font-bold tracking-[0.2em] text-foreground-secondary uppercase transition-colors group-hover:text-primary"
			>
				Continue with Existing Templates
			</span>
			<div
				class="h-px w-6 bg-primary/40 transition-all duration-300 group-hover:w-10 group-hover:bg-primary"
			></div>
		</button>
	</div>
</div>
