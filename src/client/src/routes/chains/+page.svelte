<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { getChains, getChain, isDegraded, type ChainDetail } from '$lib/api';
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';

	let chains = $state<ChainDetail[]>([]);
	let loading = $state(true);
	let selectedId = $state<number | null>(null);
	let searchQuery = $state('');
	let error = $state('');

	let filteredChains = $derived.by(() => {
		if (!searchQuery.trim()) return chains;
		const q = searchQuery.trim().toLowerCase();
		return chains.filter(
			(c) =>
				c.chain_info.name.toLowerCase().includes(q) ||
				String(c.chain_info.chain_id).includes(q)
		);
	});

	let selectedDetail = $derived(
		selectedId !== null ? chains.find((c) => c.chain_info.chain_id === selectedId) ?? null : null
	);

	onMount(async () => {
		try {
			const ids = await getChains();
			chains = await Promise.all(ids.map(getChain));
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load chains';
		} finally {
			loading = false;
		}
	});

	function statusColor(s: 'Up' | 'Down' | 'Unknown') {
		if (s === 'Up') return 'bg-neon';
		if (s === 'Down') return 'bg-red-500';
		return 'bg-zinc-700';
	}

	function chainStatus(chain: ChainDetail): { label: string; color: string } {
		if (!chain.stats) return { label: 'no data', color: 'text-zinc-600' };
		if (!chain.stats.success) return { label: 'down', color: 'text-red-400' };
		if (isDegraded(chain)) return { label: 'degraded', color: 'text-amber-500' };
		return { label: 'up', color: 'text-neon' };
	}

	function dotColor(chain: ChainDetail): string {
		if (!chain.stats) return 'bg-zinc-700';
		if (!chain.stats.success) return 'bg-red-500';
		if (isDegraded(chain)) return 'bg-amber-500';
		return 'bg-neon';
	}
</script>

<svelte:head>
	<title>nrs.pub — Chains</title>
</svelte:head>

<div class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />

	<!-- Header -->
	<div class="border-b border-white/10 bg-[#08080a] p-8 lg:p-16">
		<div class="mb-4 font-mono text-xs uppercase tracking-widest text-zinc-500">supported networks</div>
		<h1 class="mb-4 font-sans text-3xl font-medium tracking-tighter text-zinc-100 lg:text-5xl">
			all chains. one endpoint.
		</h1>
		<p class="max-w-xl text-lg leading-relaxed text-zinc-400">
			search for any chain to see latency, error rate, and 24h uptime history.
		</p>
		<div class="mt-6">
			<input
				type="text"
				placeholder="search by chain name or id..."
				bind:value={searchQuery}
				class="w-full max-w-md rounded-sm border border-white/10 bg-black/50 px-4 py-2.5 font-mono text-sm text-zinc-300 placeholder-zinc-600 transition-all focus:border-neon/50 focus:outline-none focus:ring-1 focus:ring-neon/50"
			/>
		</div>
	</div>

	{#if error}
		<div class="border-b border-red-500/20 bg-red-500/5 p-6 font-mono text-xs text-red-400">{error}</div>
	{/if}

	<div class="grid grid-cols-1 lg:grid-cols-[320px_1fr]">
		<!-- Chain List -->
		<div class="border-b border-white/10 lg:border-b-0 lg:border-r">
			{#if loading}
				<div class="p-8 font-mono text-xs text-zinc-500 animate-pulse">loading chains...</div>
			{:else}
				<div class="max-h-[220px] overflow-y-auto lg:max-h-[600px]">
					{#each filteredChains as chain (chain.chain_info.chain_id)}
						{@const status = chainStatus(chain)}
						<button
							onclick={() => (selectedId = chain.chain_info.chain_id)}
							class="flex w-full items-center justify-between border-b border-white/5 px-5 py-3.5 text-left transition-colors hover:bg-white/[0.03] {selectedId === chain.chain_info.chain_id ? 'bg-neon/5 border-l-2 border-l-neon' : ''}"
						>
							<div class="min-w-0">
								<div class="font-sans text-sm text-zinc-100">{chain.chain_info.name}</div>
								<div class="mt-0.5 flex items-center gap-1 font-mono text-[10px] text-zinc-500">
									<a
										href="https://nrs.pub/{chain.chain_info.chain_id}"
										target="_blank"
										rel="noopener noreferrer"
										onclick={(e) => e.stopPropagation()}
										class="hover:text-neon transition-colors"
									>nrs.pub/{chain.chain_info.chain_id}</a>
									<CopyButton text="https://nrs.pub/{chain.chain_info.chain_id}" />
								</div>
							</div>
							<div class="ml-3 flex shrink-0 items-center gap-1.5">
								<span class="h-1.5 w-1.5 rounded-full {dotColor(chain)}"></span>
								<span class="font-mono text-[10px] {status.color}">{status.label}</span>
							</div>
						</button>
					{/each}
					{#if filteredChains.length === 0 && searchQuery.trim()}
						<div class="p-8 font-mono text-xs text-zinc-500">no chains matching "{searchQuery}"</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Stats Panel -->
		<div class="p-8 lg:p-12">
			{#if selectedId === null}
				<div transition:fade={{ duration: 150 }} class="hidden h-full min-h-[300px] flex-col items-center justify-center text-center lg:flex">
					<div class="mb-3 font-mono text-xs uppercase tracking-widest text-zinc-600">select a chain</div>
					<p class="font-mono text-sm text-zinc-500">click any chain to view its data.</p>
				</div>
			{:else if selectedDetail}
				{@const info = selectedDetail.chain_info}
				<div transition:slide={{ duration: 200 }}>
					<div class="mb-6">
						<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-600">chain</div>
						<h2 class="text-xl font-medium tracking-tight text-zinc-100">{info.name}</h2>
						<div class="mt-1 flex items-center gap-2 font-mono text-xs text-zinc-500">
							<span>id: {info.chain_id}</span>
							<span>·</span>
							<span>{info.redundancy_set} nodes</span>
							<span>·</span>
							<a
								href="https://nrs.pub/{info.chain_id}"
								target="_blank"
								rel="noopener noreferrer"
								class="text-zinc-400 transition-colors hover:text-neon"
							>nrs.pub/{info.chain_id}</a>
							<CopyButton text="https://nrs.pub/{info.chain_id}" />
						</div>
					</div>

					{#if selectedDetail.stats}
						{@const stats = selectedDetail.stats}
						{@const status = chainStatus(selectedDetail)}
						<!-- Key metrics -->
						<div class="mb-8 grid grid-cols-3 gap-px bg-white/5">
							<div class="bg-[#050507] p-3 sm:p-5">
								<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-600">status</div>
								<div class="font-mono text-sm font-medium sm:text-lg {status.color}">
									{status.label}
								</div>
							</div>
							<div class="bg-[#050507] p-3 sm:p-5">
								<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-600">avg latency</div>
								<div class="font-mono text-sm font-medium text-zinc-100 sm:text-lg">
									{stats.avg_latency_ms.toFixed(0)}<span class="text-xs text-zinc-500">ms</span>
								</div>
							</div>
							<div class="bg-[#050507] p-3 sm:p-5">
								<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-600">error rate</div>
								<div class="font-mono text-sm font-medium sm:text-lg {stats.error_pct > 20 ? 'text-amber-400' : stats.error_pct > 0 ? 'text-zinc-300' : 'text-zinc-100'}">
									{stats.error_pct.toFixed(1)}<span class="text-xs text-zinc-500">%</span>
								</div>
							</div>
						</div>

						<!-- 24h status bar -->
						<div>
							<div class="mb-3 font-mono text-[10px] uppercase tracking-widest text-zinc-600">24-hour history</div>
							<div class="flex items-end gap-0.5">
								{#each stats.hourly_status as s, i}
									<div
										title="hour {i + 1}: {s.toLowerCase()}"
										class="h-5 w-full rounded-sm {statusColor(s)}"
									></div>
								{/each}
							</div>
							<div class="mt-2 flex justify-between font-mono text-[10px] text-zinc-600">
								<span>24h ago</span>
								<span>now</span>
							</div>
							<div class="mt-3 flex items-center gap-4 font-mono text-[10px] text-zinc-500">
								<span class="flex items-center gap-1.5"><span class="h-2 w-2 rounded-sm bg-neon"></span> up</span>
								<span class="flex items-center gap-1.5"><span class="h-2 w-2 rounded-sm bg-red-500"></span> down</span>
								<span class="flex items-center gap-1.5"><span class="h-2 w-2 rounded-sm bg-zinc-700"></span> no data</span>
							</div>
						</div>
					{:else}
						<div class="rounded border border-white/5 bg-white/[0.02] p-6 font-mono text-sm text-zinc-500">
							statistics are not yet available for this chain. the monitor collects data continuously — check back shortly.
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<Footer />
</div>
