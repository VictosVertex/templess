import {
	ItemSlot,
	StatCategory,
	type ClassResponse,
	type Gem,
	type Item,
	type Stat,
	type StatDefinition,
	type StatPreference
} from '$lib/types';
import { SvelteMap } from 'svelte/reactivity';

const ACUITY_ID = 156;
const ALL_MAGIC_SKILLS_ID = 163;
const ALL_MELEE_SKILLS_ID = 164;
const ALL_ARCHERY_SKILLS_ID = 168;
const ALL_DUAL_WIELD_SKILLS_ID = 167;

export enum EquipSource {
	User,
	Optimizer
}
export interface EquippedItem {
	item: Item;
	source: EquipSource;
	gems: Gem[];
}

export interface TemplateBuilderSnapshot {
	equipped_items: Record<number, number>;
	preferences: Record<number, StatPreference>;
}

export class TemplateBuilder {
	equippedItems = $state<Partial<Record<ItemSlot, EquippedItem>>>({});
	preferences = $state<Record<number, StatPreference>>({});
	getTemplateClass: () => ClassResponse;

	private getStatDictionary: () => Record<number, StatDefinition>;

	constructor(statDict: () => Record<number, StatDefinition>, templateClass: () => ClassResponse) {
		this.getStatDictionary = statDict;
		this.getTemplateClass = templateClass;
	}

	equipItem(slot: ItemSlot, item: Item, source: EquipSource, gems: Gem[] = []) {
		this.equippedItems[slot] = { item, source, gems };
	}

	applyOptimizationResult(
		equipMap: Partial<Record<ItemSlot, Item>>,
		slottedGems: Partial<Record<ItemSlot, Gem[]>>
	) {
		for (const [slotStr, item] of Object.entries(equipMap)) {
			const slot = parseInt(slotStr) as ItemSlot;
			if (this.equippedItems[slot] && this.equippedItems[slot]!.source === EquipSource.User) {
				continue;
			}

			this.equippedItems[slot] = {
				item: item!,
				source: EquipSource.Optimizer,
				gems: slottedGems[slot] ?? []
			};
		}
	}

	unequipItem(slot: ItemSlot) {
		delete this.equippedItems[slot];
	}

	userEquippedItems = $derived(() =>
		Object.values(this.equippedItems)
			.filter((equip) => equip.source === EquipSource.User)
			.map((equip) => equip.item)
	);

	setPreferences(preferences: Record<number, StatPreference>) {
		this.preferences = preferences;
	}

	toSnapshot(): TemplateBuilderSnapshot {
		const equipped_items = Object.fromEntries(
			Object.entries(this.equippedItems)
				.filter(([, equipped]) => equipped?.source === EquipSource.User)
				.map(([slot, equipped]) => [slot, equipped!.item.id])
		);

		return {
			equipped_items,
			preferences: { ...this.preferences }
		};
	}

	restoreSnapshot(snapshot: TemplateBuilderSnapshot, itemDictionary: Record<number, Item>) {
		for (const [slotStr, itemId] of Object.entries(snapshot.equipped_items ?? {})) {
			const slot = parseInt(slotStr, 10) as ItemSlot;
			const item = itemDictionary[itemId];

			if (item) {
				this.equipItem(slot, item, EquipSource.User);
			}
		}

		this.preferences = { ...(snapshot.preferences ?? {}) };
	}

	buckets = $derived.by(() => {
		const b = {
			baseStats: [] as Stat[],
			capStats: [] as Stat[],
			resists: [] as Stat[],
			skills: [] as Stat[],
			bonuses: [] as Stat[]
		};

		const stats = this.getStatDictionary();
		const currentClass = this.getTemplateClass();
		const uiMap = new SvelteMap<number, Stat>();

		for (const stat of Object.values(stats)) {
			let targetBucket = null;

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
				const StatPreference = this.preferences[stat.id] ?? { min: 0, weight: 0 };
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

		for (const equip of Object.values(this.equippedItems)) {
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
			if (stat.id in this.preferences) {
				const effectiveValue = Math.max(0, Math.min(stat.value, stat.currentCap));
				total += effectiveValue * stat.utility * (this.preferences[stat.id]?.weight ?? 0);
			}
		}

		return total;
	});
}
