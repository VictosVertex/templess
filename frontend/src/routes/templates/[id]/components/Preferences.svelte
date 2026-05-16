<script lang="ts">
	import { Search } from 'lucide-svelte';
	import {
		StatCategory,
		type ClassResponse,
		type StatDefinition,
		type StatPreference
	} from '$lib/types';
	import Preference from './Preference.svelte';
	import { untrack } from 'svelte';
	import { STAT_CATEGORIES } from '$lib/constants';

	let {
		stats,
		initialPreferences,
		template_class,
		onSave
	}: {
		stats: StatDefinition[];
		initialPreferences: Record<number, StatPreference>;
		template_class: ClassResponse;
		onSave: (prefs: Record<number, StatPreference>) => void;
	} = $props();

	let draftPreferences = $state<Record<number, StatPreference>>(
		untrack(() => structuredClone($state.snapshot(initialPreferences)))
	);

	let searchQuery = $state('');
	let activeTab = $state(STAT_CATEGORIES.PHYSICAL);

	const tabs = [
		{ id: STAT_CATEGORIES.PHYSICAL, label: 'Base Stats' },
		{ id: STAT_CATEGORIES.RESISTS, label: 'Resists' },
		{ id: STAT_CATEGORIES.SKILLS, label: 'Skills' },
		{ id: STAT_CATEGORIES.TOA_BONUSES, label: 'TOA Bonuses' }
	] as const;

	let filteredStats = $derived.by(() => {
		return stats.filter((stat) => {
			if (
				stat.category_id === StatCategory.PhysicalStatCaps ||
				stat.category_id === StatCategory.AcuityStatCaps
			) {
				return false;
			}

			const matchesSearch = stat.name.toLowerCase().includes(searchQuery.toLowerCase());
			if (!matchesSearch) return false;

			switch (activeTab) {
				case STAT_CATEGORIES.PHYSICAL:
					return (
						stat.category_id === StatCategory.PhysicalStats ||
						stat.id === template_class.acuity_stat_id
					);
				case STAT_CATEGORIES.RESISTS:
					return stat.category_id === StatCategory.Resists;
				case STAT_CATEGORIES.SKILLS:
					return template_class.skill_line_ids.includes(stat.id);
				case STAT_CATEGORIES.TOA_BONUSES:
					return stat.category_id === StatCategory.ToaBonuses;
				default:
					return false;
			}
		});
	});

	function handleApply() {
		onSave($state.snapshot(draftPreferences));
	}

	function getCapStatFor(baseStatId: number): StatDefinition | undefined {
		return stats.find(
			(s) =>
				(s.category_id === StatCategory.PhysicalStatCaps ||
					s.category_id === StatCategory.AcuityStatCaps) &&
				s.base_stat_id === baseStatId
		);
	}

	function handleUpdateDraft(updates: { id: number; min: number; weight: number }[]) {
		for (const update of updates) {
			draftPreferences[update.id] = { min: update.min, weight: update.weight };
		}
	}
</script>

<div class="flex h-200 w-full flex-col bg-surface-lowest">
	<div
		class="flex flex-col gap-4 border-b border-outline/50 px-6 pt-4 sm:flex-row sm:items-end sm:justify-between"
	>
		<div class="flex gap-2">
			{#each tabs as tab (tab.id)}
				<button
					class="border-b-2 px-4 py-3 text-sm font-bold tracking-wide transition-all {activeTab ===
					tab.id
						? 'border-primary text-primary'
						: 'border-transparent text-foreground-secondary hover:border-outline hover:text-foreground'}"
					onclick={() => (activeTab = tab.id)}
				>
					{tab.label}
				</button>
			{/each}
		</div>

		<div class="relative mb-2 flex w-full max-w-xs items-center sm:mb-2">
			<Search size={18} class="absolute left-3 text-foreground-secondary" />
			<input
				type="text"
				placeholder="Search stats..."
				bind:value={searchQuery}
				class="w-full rounded-sm border border-outline bg-surface-lowest py-2 pr-4 pl-10 text-sm transition-colors focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
			/>
		</div>
	</div>

	<div class="flex-1 overflow-y-auto bg-surface-lowest px-6 py-2">
		<div class="flex flex-col">
			{#each filteredStats as stat (stat.id)}
				<Preference
					{stat}
					capStat={getCapStatFor(stat.id)}
					preference={draftPreferences[stat.id]}
					onChange={handleUpdateDraft}
				/>
			{:else}
				<div
					class="flex flex-col items-center justify-center py-12 text-foreground-secondary opacity-50"
				>
					<Search size={48} class="mb-4" />
					<p>No stats found matching "{searchQuery}" in this category.</p>
				</div>
			{/each}
		</div>
	</div>

	<div class="flex border-t border-outline/50 bg-surface-lowest p-6">
		<button
			class="ml-auto w-64 cursor-pointer rounded-sm border border-primary/20 bg-primary py-4 text-sm font-bold tracking-widest text-surface-lowest uppercase shadow-md transition-all hover:brightness-110 active:scale-[0.98] active:shadow-sm"
			onclick={handleApply}
		>
			Apply Preferences
		</button>
	</div>
</div>
