import {
	ItemSlot,
	StatCategory,
	type ClassResponse,
	type Item,
	type Stat,
	type StatDefinition
} from '$lib/types';
import { SvelteMap } from 'svelte/reactivity';

const ACUITY_ID = 156;

export class TemplateBuilder {
	equippedItems = $state<Partial<Record<ItemSlot, Item>>>({});
	activeSlot = $state<ItemSlot | null>(null);

	private getStatDictionary: () => Record<number, StatDefinition>;
	private getTemplateClass: () => ClassResponse;

	constructor(statDict: () => Record<number, StatDefinition>, templateClass: () => ClassResponse) {
		this.getStatDictionary = statDict;
		this.getTemplateClass = templateClass;
	}

	openSlot(slot: ItemSlot) {
		this.activeSlot = slot;
	}

	closeModal() {
		this.activeSlot = null;
	}

	equipItem(item: Item) {
		if (this.activeSlot) {
			this.equippedItems[this.activeSlot] = item;
			this.closeModal();
		}
	}

	unequipItem(slot: ItemSlot) {
		delete this.equippedItems[slot];
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
				const activeStat: Stat = {
					...stat,
					value: 0,
					currentCap: stat.cap
				};

				targetBucket.push(activeStat);
				uiMap.set(stat.id, activeStat);

				if (stat.id === currentClass.acuity_stat_id) {
					uiMap.set(ACUITY_ID, activeStat);
				}
			}
		}

		for (const item of Object.values(this.equippedItems)) {
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
