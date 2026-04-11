import { API_BASE_URL } from './constants';

export const api = {
	initialize: async () => {
		const response = await fetch(`${API_BASE_URL}/init`, { method: 'POST' });

		if (!response.ok) {
			throw new Error(`Failed to initialize: ${response.statusText}`);
		}

		return await response.json();
	},
	deleteTemplate: async (id: number) => {
		const response = await fetch(`${API_BASE_URL}/templates/${id}`, {
			method: 'DELETE'
		});

		if (!response.ok) {
			throw new Error(`Failed to delete: ${response.statusText}`);
		}

		return true;
	},
	getTemplate: async (id: number, customFetch: typeof window.fetch) => {
		const response = await customFetch(`${API_BASE_URL}/templates/${id}`);

		if (!response.ok) {
			throw new Error(`Failed to fetch template: ${response.statusText}`);
		}

		return await response.json();
	},
	getItemsByClass: async (classId: number, customFetch: typeof window.fetch) => {
		const response = await customFetch(`${API_BASE_URL}/data/items?class_id=${classId}`);

		if (!response.ok) {
			throw new Error(`Failed to fetch items: ${response.statusText}`);
		}

		return await response.json();
	}
};
