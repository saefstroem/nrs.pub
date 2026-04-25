/** Chains whose error_pct exceeds this value are shown as degraded. */
export const DEGRADED_THRESHOLD = 20;

export function isDegraded(chain: ChainDetail): boolean {
	return chain.stats !== null && chain.stats.error_pct > DEGRADED_THRESHOLD;
}

export interface ChainInfo {
	name: string;
	chain_id: number;
	redundancy_set: number;
}

export interface ChainStats {
	avg_latency_ms: number;
	error_pct: number;
	success: boolean;
	hourly_status: ('Up' | 'Down' | 'Unknown')[];
	next_hourly_entry: number;
}

export interface ChainDetail {
	chain_info: ChainInfo;
	stats: ChainStats | null;
}

class ApiError extends Error {
	status: number;
	constructor(status: number, message: string) {
		super(message);
		this.status = status;
	}
}

async function request<T>(url: string, options?: RequestInit): Promise<T> {
	const resp = await fetch(url, options);
	if (!resp.ok) {
		const text = await resp.text().catch(() => 'Unknown error');
		throw new ApiError(resp.status, text);
	}
	return resp.json();
}

/** Returns a sorted array of all supported chain IDs */
export async function getChains(): Promise<number[]> {
	return request<number[]>('/api/v1/chains');
}

/** Returns full chain info + live stats for a single chain */
export async function getChain(chainId: number): Promise<ChainDetail> {
	return request<ChainDetail>(`/api/v1/chains/${chainId}`);
}

/** Fetches all chain details in parallel */
export async function getAllChainDetails(ids: number[]): Promise<ChainDetail[]> {
	return Promise.all(ids.map(getChain));
}
