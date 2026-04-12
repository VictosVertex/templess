import {
	ItemSlot,
	StatCategory,
	type ClassResponse,
	type Item,
	type Stat,
	type StatDefinition,
	type StatPreference
} from '$lib/types';
import { SvelteMap } from 'svelte/reactivity';

const ACUITY_ID = 156;

export enum EquipSource {
	User,
	Optimizer
}
export interface EquippedItem {
	item: Item;
	source: EquipSource;
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

	equipItem(slot: ItemSlot, item: Item, source: EquipSource) {
		this.equippedItems[slot] = { item, source };
	}

	applyOptimizationResult(equipMap: Partial<Record<ItemSlot, Item>>) {
		for (const [slotStr, item] of Object.entries(equipMap)) {
			const slot = parseInt(slotStr) as ItemSlot;
			if (this.equippedItems[slot] && this.equippedItems[slot]!.source === EquipSource.User) {
				continue;
			}

			this.equippedItems[slot] = { item: item!, source: EquipSource.Optimizer };
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

		for (const equip of Object.values(this.equippedItems)) {
			const item = equip.item;

			if (!item || !item.bonuses) continue;

			for (const [statIdStr, value] of Object.entries(item.bonuses)) {
				const statId = parseInt(statIdStr, 10);
				const statDef = stats[statId];

				if (!statDef) continue;

				if (
					statDef.category_id === StatCategory.PhysicalStatCaps ||
					statDef.category_id === StatCategory.AcuityStatCaps
				) {
					if (statDef.base_stat_id) {
						const baseStatToCap = uiMap.get(statDef.base_stat_id);
						if (baseStatToCap) {
							baseStatToCap.currentCap += value;
						}
					}
				} else {
					const statToUpdate = uiMap.get(statId);
					if (statToUpdate) {
						statToUpdate.value += value;
					}
				}
			}
		}

		return b;
	});
}
