const BASE_URL = 'http://localhost:3000';

export const api = {
	deleteTemplate: async (id: number) => {
		const response = await fetch(`${BASE_URL}/templates/${id}`, {
			method: 'DELETE'
		});

		if (!response.ok) {
			throw new Error(`Failed to delete: ${response.statusText}`);
		}

		return true;
	},
	getTemplate: async (id: number, customFetch: typeof window.fetch) => {
		const response = await customFetch(`${BASE_URL}/templates/${id}`);

		if (!response.ok) {
			throw new Error(`Failed to fetch template: ${response.statusText}`);
		}

		return await response.json();
	}
};
