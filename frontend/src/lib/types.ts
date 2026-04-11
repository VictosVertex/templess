export interface Realm {
	id: number;
	name: string;
}

export interface ClassResponse {
	id: number;
	name: string;
	realm_id: number;
	acuity_stat_id: number | null;
	skill_line_ids: number[];
}

export enum StatCategory {
	/// Represents general stats like strength, dexterity, etc.
	PhysicalStats = 0,

	/// Represents acuity stats, which are specific to caster classes.
	AcuityStats = 1,

	/// Represents stat cap increases for regular stats.
	PhysicalStatCaps = 2,

	/// Represents stat cap increases for acuity stats.
	AcuityStatCaps = 3,

	/// Represents resistances, such as heat, cold, body, etc.
	Resists = 4,

	/// Represents magic skills, such as Regrowth.
	MagicSkills = 5,

	/// Represents melee skills, such as Large Weapons.
	MeleeSkills = 6,

	/// Represents archery lines, such as Long Bow.
	ArcherySkills = 7,

	/// Represents dual wielding skills, such as Left Axe.
	DualWieldingSkills = 8,

	/// Represents other skills, such as Stealth.
	OtherSkills = 9,

	/// Represents Trials of Atlantis bonuses, such as spell duration.
	ToaBonuses = 10,

	/// Represents other stats that do not fit into the above categories.
	OtherStats = 11
}

export interface StatDefinition {
	id: number;
	name: string;
	cap: number;
	category_id: StatCategory;
	base_stat_id: number | null;
}

export interface Stat extends StatDefinition {
	value: number;
	currentCap: number;
}

export interface AppData {
	isInitialized: boolean;
	stats: Record<number, StatDefinition>;
	classes: ClassResponse[];
	realms: Realm[];
	templates: Template[];
}

export interface Template {
	id: number;
	name: string;
	class_id: number;
}

export interface Item {
	id: number;
	name: string;
	object_type_id: number;
	item_slot_id: number;
	weapon_hand: number;
	utility_single: number;
	utility: number;
	bonuses: Record<number, number>;
}

export enum ItemSlot {
	RightHand = 10,
	LeftHand = 11,
	TwoHanded = 12,
	Ranged = 13,
	Head = 21,
	Hands = 22,
	Feet = 23,
	Jewel = 24,
	Chest = 25,
	Cloak = 26,
	Legs = 27,
	Arms = 28,
	Necklace = 29,
	Belt = 32,
	Bracer = 33,
	Bracer2 = 34,
	Ring = 35,
	Ring2 = 36
}

export interface OptimizationRequest {
	class_id: number;
	equipped_items: Record<number, number>;
}

export enum ClientMessageType {
	Start = 'start',
	Cancel = 'cancel'
}

export enum ServerMessageType {
	Setup = 'setup',
	Grounding = 'grounding',
	Solving = 'solving',
	NewModel = 'new_model',
	Finished = 'finished',
	Canceled = 'canceled',
	Error = 'error'
}

export enum OptimizationStatus {
	Ready,
	Setup,
	Grounding,
	Solving,
	Finished
}

export type ClientMessage =
	| { type: ClientMessageType.Start; data: OptimizationRequest }
	| { type: ClientMessageType.Cancel };

export type ServerMessage =
	| { type: ServerMessageType.Setup }
	| { type: ServerMessageType.Grounding }
	| { type: ServerMessageType.Solving }
	| { type: ServerMessageType.NewModel; data: { optimized_items: Record<number, number> } }
	| { type: ServerMessageType.Finished }
	| { type: ServerMessageType.Canceled }
	| { type: ServerMessageType.Error; data: { message: string } };
