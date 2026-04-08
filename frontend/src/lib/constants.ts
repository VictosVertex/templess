import { Shield, Axe, Leaf } from 'lucide-svelte';
import { ItemSlot } from './types';

export const realmTheme: Record<number, { icon: typeof Shield; color: string; bg: string }> = {
	1: { icon: Shield, color: 'text-red-400', bg: 'bg-red-400/10' },
	2: { icon: Axe, color: 'text-blue-400', bg: 'bg-blue-400/10' },
	3: { icon: Leaf, color: 'text-green-400', bg: 'bg-green-400/10' }
};

export const defaultRealmTheme = { icon: Shield, color: 'text-primary', bg: 'bg-primary/10' };

export const STAT_CATEGORIES = {
	PHYSICAL: 1,
	RESISTS: 2,
	SKILLS: 3,
	TOA_BONUSES: 4,
	PHYSICAL_CAPS: 5
};

export const SLOT_NAMES: Record<ItemSlot, string> = {
	[ItemSlot.RightHand]: 'Right Hand',
	[ItemSlot.LeftHand]: 'Left Hand',
	[ItemSlot.TwoHanded]: 'Two Handed',
	[ItemSlot.Ranged]: 'Ranged',
	[ItemSlot.Head]: 'Head',
	[ItemSlot.Hands]: 'Hands',
	[ItemSlot.Feet]: 'Feet',
	[ItemSlot.Chest]: 'Chest',
	[ItemSlot.Legs]: 'Legs',
	[ItemSlot.Arms]: 'Arms',
	[ItemSlot.Cloak]: 'Cloak',
	[ItemSlot.Jewel]: 'Jewel',
	[ItemSlot.Necklace]: 'Necklace',
	[ItemSlot.Belt]: 'Belt',
	[ItemSlot.Bracer]: 'Bracer',
	[ItemSlot.Bracer2]: 'Bracer',
	[ItemSlot.Ring]: 'Ring',
	[ItemSlot.Ring2]: 'Ring'
};

export const INVENTORY_GROUPS = {
	jewelry: [
		ItemSlot.Ring2,
		ItemSlot.Bracer2,
		ItemSlot.Bracer,
		ItemSlot.Ring,
		ItemSlot.Jewel,
		ItemSlot.Necklace,
		ItemSlot.Cloak,
		ItemSlot.Belt
	],
	armor: [
		ItemSlot.Hands,
		ItemSlot.Feet,
		ItemSlot.Legs,
		ItemSlot.Arms,
		ItemSlot.Chest,
		ItemSlot.Head
	],
	weapons: [ItemSlot.RightHand, ItemSlot.LeftHand, ItemSlot.TwoHanded, ItemSlot.Ranged]
};
