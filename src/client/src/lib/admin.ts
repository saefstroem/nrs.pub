export interface RpcChain {
	chain_id: number;
	name: string;
	rpcs: string[];
}

// Helper to get auth headers for admin requests
function authHeaders(): Record<string, string> {
	const token = (typeof localStorage !== 'undefined' ? localStorage.getItem('nrs_token') : null) ?? '';
	return {
		'Content-Type': 'application/json',
		Authorization: `Bearer ${token}`,
	};
}

// Generic helper for making authenticated admin API requests
async function adminRequest<T>(url: string, options: RequestInit = {}): Promise<T> {
	const res = await fetch(url, {
		...options,
		headers: { ...authHeaders(), ...(options.headers ?? {}) },
	});
	// If unauthorized, clear token and redirect to login
	if (res.status === 401) {
		if (typeof localStorage !== 'undefined') localStorage.removeItem('nrs_token');
		window.location.href = '/login';
		throw new Error('Unauthorized');
	}
	if (res.status === 204 || res.status === 201) {
		return undefined as T;
	}
	if (!res.ok) {
		const text = await res.text();
		throw new Error(text || `Request failed: ${res.status}`);
	}
	return res.json();
}

// Admin API client with methods for managing chains and RPCs
export const admin = {
	getChains: () => adminRequest<RpcChain[]>('/api/v1/admin/chains'),

	addChain: (chain_id: number, name: string, rpcs: string[]) =>
		adminRequest<void>('/api/v1/admin/chains', {
			method: 'POST',
			body: JSON.stringify({ chain_id, name, rpcs }),
		}),

	removeChain: (chain_id: number) =>
		adminRequest<void>(`/api/v1/admin/chains/${chain_id}`, { method: 'DELETE' }),

	addRpc: (chain_id: number, url: string) =>
		adminRequest<void>(`/api/v1/admin/chains/${chain_id}/rpcs`, {
			method: 'POST',
			body: JSON.stringify({ url }),
		}),

	removeRpc: (chain_id: number, url: string) =>
		adminRequest<void>(`/api/v1/admin/chains/${chain_id}/rpcs`, {
			method: 'DELETE',
			body: JSON.stringify({ url }),
		}),
};
