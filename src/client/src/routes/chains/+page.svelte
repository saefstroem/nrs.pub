<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import ChainsList from '$lib/components/ChainsList.svelte';
	import { getChains, getChain, isDegraded, type ChainDetail } from '$lib/api';

	const INITIAL_LIMIT = 24;

	const TESTNET_KEYWORDS = ['testnet', 'test', 'goerli', 'sepolia', 'holesky', 'mumbai', 'fuji', 'ropsten', 'rinkeby', 'kovan', 'coston', 'prenet', 'devnet', 'stagenet'];
	function isTestnet(chain: ChainDetail) {
		const n = chain.chain_info.name.toLowerCase();
		return TESTNET_KEYWORDS.some((k) => n.includes(k));
	}

	let chains = $state<ChainDetail[]>([]);
	let loading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let showAll = $state(false);
	let showTestnets = $state(false);
	let statusFilter = $state<'all' | 'operational' | 'degraded'>('all');

	let filteredChains = $derived.by(() => {
		let result = showTestnets ? chains : chains.filter((c) => !isTestnet(c));
		if (statusFilter === 'operational') result = result.filter((c) => !isDegraded(c));
		else if (statusFilter === 'degraded') result = result.filter((c) => isDegraded(c));
		if (!searchQuery.trim()) return result;
		const q = searchQuery.trim().toLowerCase();
		return result.filter(
			(c) =>
				c.chain_info.name.toLowerCase().includes(q) ||
				String(c.chain_info.chain_id).includes(q)
		);
	});

	let displayedChains = $derived.by(() => {
		if (searchQuery.trim() || showAll) return filteredChains;
		return filteredChains.slice(0, INITIAL_LIMIT);
	});

	let hasMore = $derived(displayedChains.length < filteredChains.length);

	async function fetchChains() {
		try {
			const ids = await getChains();
			// Fetch all chain details in parallel
			chains = await Promise.all(ids.map(getChain));
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load chains';
		} finally {
			loading = false;
		}
	}

	fetchChains();
</script>

<svelte:head>
	<title>nrs.pub — Supported Chains</title>
</svelte:head>

<div class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />

	<!-- Header -->
	<div class="border-b border-white/10 bg-[#08080a] p-8 lg:p-16">
		<div class="mb-4 font-mono text-xs uppercase tracking-widest text-zinc-500">
			supported networks
		</div>
		<h1 class="mb-4 font-sans text-3xl font-medium tracking-tighter text-zinc-100 lg:text-5xl">
			all chains. one endpoint.
		</h1>
		<p class="max-w-xl text-lg leading-relaxed text-zinc-400">
			every supported chain, one free endpoint. no keys required.
		</p>
		{#if !loading}
			<div class="mt-6">
				<div class="flex flex-wrap items-center gap-3">
					<input
						type="text"
						placeholder="search chains..."
						bind:value={searchQuery}
						class="w-full max-w-md rounded-sm border border-white/10 bg-black/50 px-4 py-2.5 font-mono text-sm text-zinc-300 placeholder-zinc-600 transition-all focus:border-neon/50 focus:outline-none focus:ring-1 focus:ring-neon/50"
					/>
				</div>
				<p class="mt-4 max-w-xl text-sm leading-relaxed text-zinc-500">
					Network availability is adjusted continuously. Degraded chains may be removed from routing if issues persist. Follow us on <a href="https://x.com/AutoRPC" target="_blank" rel="noopener noreferrer" class="text-zinc-300 underline underline-offset-2 transition-colors hover:text-neon">X</a> for updates.
				</p>
			</div>
		{/if}
	</div>

	<!-- Usage Example -->
	<div class="border-b border-white/10 bg-[#030304] p-6 lg:p-8">
		<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
			<div class="font-mono text-xs text-zinc-500">
				<span class="text-zinc-400">usage:</span> https://nrs.pub/<span class="text-zinc-300"
					>{'{'}</span
				><span class="text-zinc-400">chain_id</span><span class="text-zinc-300">{'}'}</span
				>
			</div>
			<div class="flex flex-col gap-1 font-mono text-xs text-zinc-500">
				<span><span class="text-neon">operational</span> — redundancy set sustains targeted attacks</span>
				<span><span class="text-amber-500">degraded</span> — chain is at risk due to a small redundancy set</span>
			</div>
		</div>
	</div>

	<!-- Error -->
	{#if error}
		<div class="border-b border-red-500/20 bg-red-500/5 p-6 font-mono text-xs text-red-400">
			{error}
		</div>
	{/if}

	<!-- Filter Bar -->
	{#if !loading}
		<div class="flex flex-wrap items-center gap-2 border-b border-white/10 bg-[#08080a] px-6 py-3">
			<span class="mr-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">filter:</span>
			<button
				onclick={() => (statusFilter = 'all')}
				class="border px-3 py-1 font-mono text-xs transition-colors {statusFilter === 'all' ? 'border-white/20 bg-white/5 text-zinc-200' : 'border-white/5 text-zinc-600 hover:border-white/10 hover:text-zinc-400'}"
			>all</button>
			<button
				onclick={() => (statusFilter = 'operational')}
				class="border px-3 py-1 font-mono text-xs transition-colors {statusFilter === 'operational' ? 'border-neon/50 bg-neon/10 text-neon' : 'border-white/5 text-zinc-600 hover:border-white/10 hover:text-zinc-400'}"
			>operational</button>
			<button
				onclick={() => (statusFilter = 'degraded')}
				class="border px-3 py-1 font-mono text-xs transition-colors {statusFilter === 'degraded' ? 'border-amber-500/50 bg-amber-500/10 text-amber-500' : 'border-white/5 text-zinc-600 hover:border-white/10 hover:text-zinc-400'}"
			>degraded</button>
			<div class="ml-auto">
				<button
					onclick={() => (showTestnets = !showTestnets)}
					class="flex items-center gap-2 border px-3 py-1 font-mono text-xs transition-colors {showTestnets ? 'border-neon/50 bg-neon/10 text-neon' : 'border-white/5 text-zinc-600 hover:border-white/10 hover:text-zinc-400'}"
				>
					<span class="inline-block h-1.5 w-1.5 rounded-full {showTestnets ? 'bg-neon' : 'bg-zinc-600'}"></span>
					testnets
				</button>
			</div>
		</div>
	{/if}

	<!-- Chains Grid -->
	<ChainsList chains={displayedChains} {loading} />

	<!-- Load More -->
	{#if !loading && hasMore}
		<div class="flex justify-center border-t border-white/10 bg-[#08080a] p-6">
			<button
				onclick={() => (showAll = true)}
				class="border border-white/10 px-6 py-2.5 font-mono text-xs uppercase tracking-widest text-zinc-400 transition-colors hover:border-neon/50 hover:text-neon"
			>
				show all {filteredChains.length} chains
			</button>
		</div>
	{/if}

	<!-- No Results -->
	{#if !loading && searchQuery.trim() && filteredChains.length === 0}
		<div class="flex justify-center border-t border-white/10 bg-[#08080a] p-12">
			<div class="font-mono text-sm text-zinc-500">no chains matching "{searchQuery}"</div>
		</div>
	{/if}

	<!-- CTA -->
	<div
		class="flex flex-col items-center gap-4 border-t border-white/10 bg-[#08080a] p-8 sm:flex-row sm:justify-between lg:p-12"
	>
		<div class="font-mono text-xs text-zinc-500">
			{#if loading}
				loading chains...
			{:else if searchQuery.trim()}
				{filteredChains.length} of {chains.length} chains
			{:else}
				{chains.length} chains supported
			{/if}
		</div>
	</div>

	<Footer />
</div>
