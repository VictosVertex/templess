export interface Realm {
	id: number;
	name: string;
}

export interface ClassResponse {
	id: number;
	name: string;
	realm_id: number;
	acuity_stat_id: number;
	skill_line_ids: number[];
}

export interface Stat {
	id: number;
	name: string;
}

export interface AppData {
	isInitialized: boolean;
	stats: Record<number, string>;
	classes: ClassResponse[];
	realms: Realm[];
	templates: Template[];
}

export interface Template {
	id: number;
	name: string;
	class_id: number;
}
