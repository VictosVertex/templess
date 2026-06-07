import { API_BASE_URL } from './constants';
import type { StatDefinition, Gem, Item } from '$lib/types';

async function fetchWithError(
	url: string,
	options: RequestInit,
	customFetch: typeof window.fetch = fetch
) {
	const response = await customFetch(url, options);
	if (!response.ok) {
		throw new Error(`API Error at ${url}: ${response.statusText}`);
	}
	return response;
}

function toDictionary<T extends { id: number }>(array: T[]): Record<number, T> {
	return Object.fromEntries(array.map((item) => [item.id, item]));
}

export const api = {
	checkInitialization: async (customFetch: typeof window.fetch = fetch) => {
		const response = await fetchWithError(`${API_BASE_URL}/init`, { method: 'GET' }, customFetch);
		return (await response.json()) as boolean;
	},

	initialize: async (customFetch: typeof window.fetch = fetch) => {
		await fetchWithError(`${API_BASE_URL}/init`, { method: 'POST' }, customFetch);
		return;
	},

	getStats: async (customFetch: typeof window.fetch = fetch) => {
		const res = await fetchWithError(`${API_BASE_URL}/data/stats`, {}, customFetch);
		const data = await res.json();
		return toDictionary<StatDefinition>(data);
	},

	getClasses: async (customFetch: typeof window.fetch = fetch) => {
		const response = await fetchWithError(`${API_BASE_URL}/data/classes`, {}, customFetch);
		return await response.json();
	},

	getGems: async (customFetch: typeof window.fetch = fetch) => {
		const res = await fetchWithError(`${API_BASE_URL}/data/gems`, {}, customFetch);
		const data = await res.json();
		return toDictionary<Gem>(data);
	},

	getRealms: async (customFetch: typeof window.fetch = fetch) => {
		const response = await fetchWithError(`${API_BASE_URL}/data/realms`, {}, customFetch);
		return await response.json();
	},

	// --- TEMPLATES & ITEMS ---

	getTemplates: async (customFetch: typeof window.fetch = fetch) => {
		const response = await fetchWithError(`${API_BASE_URL}/templates`, {}, customFetch);
		return await response.json();
	},

	getTemplate: async (id: number, customFetch: typeof window.fetch = fetch) => {
		const response = await fetchWithError(`${API_BASE_URL}/templates/${id}`, {}, customFetch);
		return await response.json();
	},

	getItemsByClass: async (classId: number, customFetch: typeof window.fetch = fetch) => {
		const res = await fetchWithError(
			`${API_BASE_URL}/data/items?class_id=${classId}`,
			{},
			customFetch
		);
		const data = await res.json();
		return toDictionary<Item>(data);
	},

	deleteTemplate: async (id: number) => {
		await fetchWithError(`${API_BASE_URL}/templates/${id}`, { method: 'DELETE' });
		return true;
	}
};
