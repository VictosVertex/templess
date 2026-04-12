import { API_BASE_URL } from '$lib/constants';
import type { LayoutLoad } from './$types';
import type { AppData, ClassResponse, Realm, StatDefinition, Template } from '$lib/types';

export const load: LayoutLoad = async ({ fetch }): Promise<AppData> => {
	const initRes = await fetch(`${API_BASE_URL}/init`);
	const isInitialized: boolean = await initRes.json();

	if (!isInitialized) {
		return {
			isInitialized: false,
			stats: {},
			classes: [],
			realms: [],
			templates: []
		};
	}

	const [statsRes, classesRes, realmsRes, templatesRes] = await Promise.all([
		fetch(`${API_BASE_URL}/data/stats`),
		fetch(`${API_BASE_URL}/data/classes`),
		fetch(`${API_BASE_URL}/data/realms`),
		fetch(`${API_BASE_URL}/templates`)
	]);

	const rawStats: StatDefinition[] = await statsRes.json();

	const statDict = rawStats.reduce(
		(acc, stat) => {
			acc[stat.id] = stat;
			return acc;
		},
		{} as Record<number, StatDefinition>
	);

	return {
		isInitialized: true,
		stats: statDict,
		classes: (await classesRes.json()) as ClassResponse[],
		realms: (await realmsRes.json()) as Realm[],
		templates: (await templatesRes.json()) as Template[]
	};
};
