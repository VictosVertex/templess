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
	}
};
