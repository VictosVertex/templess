import type { LayoutLoad } from './$types';
import type { AppData, ClassResponse, Realm, Stat } from '$lib/types';

export const load: LayoutLoad = async ({ fetch }): Promise<AppData> => {
	const initRes = await fetch('http://localhost:3000/init');
	const isInitialized: boolean = await initRes.json();

	if (!isInitialized) {
		return {
			isInitialized: false,
			stats: {},
			classes: [],
			realms: []
		};
	}

	const [statsRes, classesRes, realmsRes] = await Promise.all([
		fetch('http://localhost:3000/data/stats'),
		fetch('http://localhost:3000/data/classes'),
		fetch('http://localhost:3000/data/realms')
	]);

	const rawStats: Stat[] = await statsRes.json();

	const statDict = rawStats.reduce(
		(acc, stat) => {
			acc[stat.id] = stat.name;
			return acc;
		},
		{} as Record<number, string>
	);

	return {
		isInitialized: true,
		stats: statDict,
		classes: (await classesRes.json()) as ClassResponse[],
		realms: (await realmsRes.json()) as Realm[]
	};
};
