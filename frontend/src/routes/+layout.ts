import { api } from '$lib/api';
import type { LayoutLoad } from './$types';
import type { AppData, Template, ClassResponse, OptimizationContext } from '$lib/types';

export const load: LayoutLoad = async ({ url, fetch }): Promise<AppData> => {
	const isInitialized = await api.checkInitialization(fetch);

	if (!isInitialized) {
		return {
			isInitialized: false,
			stats: {},
			classes: [],
			gems: [],
			realms: [],
			templates: [],
			activeContext: null
		};
	}

	const [stats, classes, gems, realms, templates] = await Promise.all([
		api.getStats(fetch),
		api.getClasses(fetch),
		api.getGems(fetch),
		api.getRealms(fetch),
		api.getTemplates(fetch)
	]);

	const templateParam = url.searchParams.get('template');
	let activeContext: OptimizationContext | null = null;

	if (templateParam) {
		const templateId = parseInt(templateParam, 10);
		const activeTemplate = templates.find((t: Template) => t.id === templateId);

		if (activeTemplate) {
			const items = await api.getItemsByClass(activeTemplate.class_id, fetch);
			const templateClass = classes.find((c: ClassResponse) => c.id === activeTemplate.class_id);

			if (!templateClass) throw new Error('Class not found for template');

			activeContext = {
				template: activeTemplate,
				templateClass,
				items,
				storageKey: `template-draft:${activeTemplate.id}`
			};
		}
	}

	return {
		isInitialized: true,
		stats,
		classes,
		gems,
		realms,
		templates,
		activeContext
	};
};
