import { api } from '$lib/api';
import type { Template } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const templateId = parseInt(params.id, 10);

	const template: Template = await api.getTemplate(templateId, fetch);

	return {
		template
	};
};
