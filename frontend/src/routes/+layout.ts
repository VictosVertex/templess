import { api } from '$lib/api';
import type { LayoutLoad } from './$types';
import type { AppData, ClassResponse, OptimizationContext, Template } from '$lib/types';

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
			activeContext: null,
			preferencePresets: {}
		};
	}

	const [stats, classes, gems, realms, templates, preferencePresets] = await Promise.all([
		api.getStats(fetch),
		api.getClasses(fetch),
		api.getGems(fetch),
		api.getRealms(fetch),
		api.getTemplates(fetch),
		api.getPreferences(fetch)
	]);

	const templateParam = url.searchParams.get('template');
	let activeContext: OptimizationContext | null = null;

	if (templateParam) {
		const templateId = parseInt(templateParam, 10);
		const templateSummary = templates.find((template: Template) => template.id === templateId);

		if (templateSummary) {
			const activeTemplate = await api.getTemplate(templateId, fetch);
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
		activeContext,
		preferencePresets
	};
};
