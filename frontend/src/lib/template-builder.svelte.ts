import {
	EquipSource,
	ItemSlot,
	StatCategory,
	type ClassResponse,
	type Gem,
	type Item,
	type Stat,
	type StatDefinition,
	type StatPreference,
	type Template
} from '$lib/types';
import { SvelteMap } from 'svelte/reactivity';

const ACUITY_ID = 156;
const ESSENCE_RESIST = 116;
const ALL_MAGIC_SKILLS_ID = 163;
const ALL_MELEE_SKILLS_ID = 164;
const ALL_ARCHERY_SKILLS_ID = 168;
const ALL_DUAL_WIELD_SKILLS_ID = 167;

export class TemplateBuilder {
	template: Template = $state() as Template;
	templateClass: ClassResponse;
	draftRevision = $state(0);

	items: Record<number, Item>;
	gems: Record<number, Gem>;
	stats: Record<number, StatDefinition>;

	constructor(
		template: Template,
		templateClass: ClassResponse,
		items: Record<number, Item>,
		gems: Record<number, Gem>,
		stats: Record<number, StatDefinition>
	) {
		this.template = template;
		this.templateClass = templateClass;
		this.items = items;
		this.gems = gems;
		this.stats = stats;
	}

	resetToTemplate(template: Template) {
		this.template = {
			...template,
			preferences: { ...template.preferences },
			equipped_items: { ...template.equipped_items }
		};
		this.draftRevision = 0;
	}

	private markDraftChanged() {
		this.draftRevision += 1;
	}

	equipItem(slot: ItemSlot, itemId: number, gemIds: number[] = []) {
		this.template.equipped_items[slot] = {
			item_id: itemId,
			source: EquipSource.User,
			gem_ids: gemIds
		};
		this.markDraftChanged();
	}

	applyOptimizationResult(
		optimizerItems: Record<number, number>,
		optimizerGems: Record<number, number[]>
	) {
		for (const [slotStr, itemId] of Object.entries(optimizerItems)) {
			const slot = parseInt(slotStr, 10);
			const existing = this.template.equipped_items[slot];

			if (!existing || existing.source !== EquipSource.User) {
				this.template.equipped_items[slot] = {
					item_id: itemId,
					source: EquipSource.Optimizer,
					gem_ids: optimizerGems[itemId] || []
				};
				this.markDraftChanged();
			}
		}
	}

	clearOptimizationResults() {
		let removedAny = false;

		for (const [slotStr, state] of Object.entries(this.template.equipped_items)) {
			if (state.source !== EquipSource.Optimizer) {
				continue;
			}

			delete this.template.equipped_items[parseInt(slotStr, 10)];
			removedAny = true;
		}

		if (removedAny) {
			this.markDraftChanged();
		}
	}

	unequipItem(slot: ItemSlot) {
		delete this.template.equipped_items[slot];
		this.markDraftChanged();
	}

	userEquippedItems = $derived.by(() =>
		Object.values(this.template.equipped_items)
			.filter((equip) => equip.source === EquipSource.User)
			.map((equip) => equip.item_id)
	);

	setPreferences(preferences: Record<number, StatPreference>) {
		this.template.preferences = preferences;
		this.markDraftChanged();
	}

	resolvedEquipment = $derived.by(() => {
		const equipment: Partial<Record<ItemSlot, { item: Item; gems: Gem[]; source: EquipSource }>> =
			{};

		for (const [slotStr, state] of Object.entries(this.template.equipped_items)) {
			const slot = parseInt(slotStr, 10) as ItemSlot;
			const item = this.items[state.item_id];

			if (item) {
				const resolvedGems = state.gem_ids.map((id) => this.gems[id]).filter(Boolean) as Gem[];

				equipment[slot] = {
					item,
					gems: resolvedGems,
					source: state.source
				};
			}
		}
		return equipment;
	});

	buckets = $derived.by(() => {
		const b = {
			baseStats: [] as Stat[],
			capStats: [] as Stat[],
			resists: [] as Stat[],
			skills: [] as Stat[],
			bonuses: [] as Stat[]
		};

		const stats = this.stats;
		const currentClass = this.templateClass;
		const uiMap = new SvelteMap<number, Stat>();

		for (const stat of Object.values(stats)) {
			let targetBucket = null;

			if (stat.id == ESSENCE_RESIST) {
				continue;
			}

			switch (stat.category_id) {
				case StatCategory.PhysicalStats:
					targetBucket = b.baseStats;
					break;
				case StatCategory.AcuityStats:
					if (stat.id === currentClass.acuity_stat_id) {
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
				case StatCategory.ToaBonuses:
					targetBucket = b.bonuses;
					break;
				default:
					if (currentClass.skill_line_ids.includes(stat.id)) {
						targetBucket = b.skills;
					}
					break;
			}

			if (targetBucket) {
				const StatPreference = this.template.preferences[stat.id] ?? { min: 0, weight: 0 };
				const activeStat: Stat = {
					...stat,
					value: 0,
					currentCap: stat.cap,
					min: StatPreference.min,
					weight: StatPreference.weight
				};

				targetBucket.push(activeStat);
				uiMap.set(stat.id, activeStat);

				if (stat.id === currentClass.acuity_stat_id) {
					uiMap.set(ACUITY_ID, activeStat);
				}
			}
		}

		function applyStatValue(statId: number, value: number) {
			const statDef = stats[statId];

			if (!statDef) {
				return;
			}

			if (
				statDef.category_id === StatCategory.PhysicalStatCaps ||
				statDef.category_id === StatCategory.AcuityStatCaps
			) {
				if (statDef.base_stat_id) {
					const baseStatToCap = uiMap.get(statDef.base_stat_id);
					if (baseStatToCap) {
						const maxCurrentCap = baseStatToCap.cap + statDef.cap;
						baseStatToCap.currentCap = Math.min(baseStatToCap.currentCap + value, maxCurrentCap);
					}
				}

				return;
			}

			const statToUpdate = uiMap.get(statId);
			if (statToUpdate) {
				statToUpdate.value += value;
			}

			let targetSkillCategory = null;
			switch (statId) {
				case ALL_MAGIC_SKILLS_ID:
					targetSkillCategory = StatCategory.MagicSkills;
					break;
				case ALL_MELEE_SKILLS_ID:
					targetSkillCategory = StatCategory.MeleeSkills;
					break;
				case ALL_ARCHERY_SKILLS_ID:
					targetSkillCategory = StatCategory.ArcherySkills;
					break;
				case ALL_DUAL_WIELD_SKILLS_ID:
					targetSkillCategory = StatCategory.DualWieldingSkills;
					break;
			}

			if (targetSkillCategory !== null) {
				for (const skill of b.skills) {
					if (skill.category_id === targetSkillCategory) {
						skill.value += value;
					}
				}
			}
		}

		for (const equip of Object.values(this.resolvedEquipment)) {
			const item = equip.item;

			if (!item || !item.bonuses) continue;

			for (const [statIdStr, value] of Object.entries(item.bonuses)) {
				const statId = parseInt(statIdStr, 10);
				applyStatValue(statId, value);
			}

			for (const gem of equip.gems) {
				applyStatValue(gem.stat_id, gem.value);
			}
		}

		return b;
	});

	totalUtility = $derived.by(() => {
		let total = 0;

		const allTrackedStats = [
			...this.buckets.baseStats,
			...this.buckets.capStats,
			...this.buckets.resists,
			...this.buckets.skills,
			...this.buckets.bonuses
		];

		for (const stat of allTrackedStats) {
			if (stat.id in this.template.preferences) {
				const effectiveValue = Math.max(0, Math.min(stat.value, stat.currentCap));
				total += effectiveValue * stat.utility * (this.template.preferences[stat.id]?.weight ?? 0);
			}
		}

		return total;
	});
}
