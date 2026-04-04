<script lang="ts">
	import { StatCategory, type Stat } from '$lib/types';
	import Attributes from './components/Attributes.svelte';
	import Inventory from './components/Inventory.svelte';
	import TemplateHeader from './components/TemplateHeader.svelte';

	let { data } = $props();

	let template_class = $derived.by(() => {
		const foundClass = data.classes.find((c) => c.id === data.template.class_id);

		if (!foundClass) {
			throw new Error('Class not found for template');
		}

		return foundClass;
	});

	let buckets = $derived.by(() => {
		let b = {
			baseStats: [] as Stat[],
			capStats: [] as Stat[],
			resists: [] as Stat[],
			skills: [] as Stat[],
			bonuses: [] as Stat[]
		};

		for (const stat of Object.values(data.stats)) {
			let targetBucket = null;

			switch (stat.category_id) {
				case StatCategory.PhysicalStats:
					targetBucket = b.baseStats;
					break;
				case StatCategory.AcuityStats:
					if (stat.id === template_class.acuity_stat_id) {
						targetBucket = b.baseStats;
					}
					break;
				case StatCategory.PhysicalStatCaps:
				case StatCategory.AcuityStatCaps:
					targetBucket = b.capStats;
					break;
				case StatCategory.Resists:
					targetBucket = b.resists;
					break;
				default:
					if (template_class.skill_line_ids.includes(stat.id)) {
						targetBucket = b.skills;
					}

					break;
			}

			if (targetBucket) {
				let activeStat: Stat = {
					...stat,
					value: 10,
					currentCap: stat.cap
				};
				targetBucket.push(activeStat);
			}
		}

		return b;
	});
</script>

<div class="mx-auto flex w-full max-w-7xl flex-col items-center gap-2 px-4 pb-12">
	<TemplateHeader template={data.template} classes={data.classes} />

	<div class="mt-8 grid w-full grid-cols-1 items-start gap-8 lg:grid-cols-[1fr_auto_1fr]">
		<div class="flex w-full flex-col gap-6">
			<Attributes title="Base Stats" stats={buckets.baseStats} />
			<Attributes title="Resists" stats={buckets.resists} />
		</div>

		<div class="relative flex w-full justify-center">
			<div
				class="pointer-events-none absolute top-1/2 left-1/2 h-150 w-150 -translate-x-1/2
            -translate-y-1/2 rounded-full bg-primary/5 blur-[100px]"
			></div>

			<Inventory />
		</div>

		<div class="flex w-full flex-col gap-6">
			<Attributes title="Skills" stats={buckets.skills} />
			<Attributes title="Bonuses" stats={buckets.bonuses} />
		</div>
	</div>
</div>
