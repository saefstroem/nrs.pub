<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { admin, type RpcChain } from '$lib/admin';
	import { onMount } from 'svelte';

	let chains = $state<RpcChain[]>([]);
	let loading = $state(true);
	let error = $state('');
	let selectedId = $state<number | null>(null);
	let searchQuery = $state('');

	// New chain form
	let newChainId = $state('');
	let newChainName = $state('');
	let newChainRpcs = $state('');
	let addingChain = $state(false);
	let addChainError = $state('');

	// New RPC form for selected chain
	let newRpcUrl = $state('');
	let addingRpc = $state(false);
	let rpcError = $state('');

	let selectedChain = $derived(selectedId !== null ? chains.find((c) => c.chain_id === selectedId) ?? null : null);

	let filteredChains = $derived.by(() => {
		if (!searchQuery.trim()) return chains;
		const q = searchQuery.trim().toLowerCase();
		return chains.filter(
			(c) => c.name.toLowerCase().includes(q) || String(c.chain_id).includes(q)
		);
	});

	onMount(async () => {
		await load();
	});

	async function load() {
		loading = true;
		error = '';
		try {
			chains = await admin.getChains();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load chains';
		} finally {
			loading = false;
		}
	}

	async function handleAddChain(e: SubmitEvent) {
		e.preventDefault();
		addChainError = '';
		addingChain = true;
		try {
			const id = parseInt(newChainId, 10);
			if (isNaN(id)) throw new Error('Chain ID must be a number');
			const rpcs = newChainRpcs
				.split('\n')
				.map((u) => u.trim())
				.filter(Boolean);
			await admin.addChain(id, newChainName.trim(), rpcs);
			newChainId = '';
			newChainName = '';
			newChainRpcs = '';
			await load();
		} catch (e) {
			addChainError = e instanceof Error ? e.message : 'Failed to add chain';
		} finally {
			addingChain = false;
		}
	}

	async function handleRemoveChain(chain_id: number) {
		try {
			await admin.removeChain(chain_id);
			if (selectedId === chain_id) selectedId = null;
			await load();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to remove chain';
		}
	}

	async function handleAddRpc(e: SubmitEvent) {
		e.preventDefault();
		if (!selectedId) return;
		rpcError = '';
		addingRpc = true;
		try {
			await admin.addRpc(selectedId, newRpcUrl.trim());
			newRpcUrl = '';
			await load();
		} catch (e) {
			rpcError = e instanceof Error ? e.message : 'Failed to add RPC';
		} finally {
			addingRpc = false;
		}
	}

	async function handleRemoveRpc(chain_id: number, url: string) {
		try {
			await admin.removeRpc(chain_id, url);
			await load();
		} catch (e) {
			rpcError = e instanceof Error ? e.message : 'Failed to remove RPC';
		}
	}

	function logout() {
		localStorage.removeItem('nrs_token');
		window.location.href = '/login';
	}
</script>

<svelte:head>
	<title>nrs.pub — Dashboard</title>
</svelte:head>

<div class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />

	<!-- Header -->
	<div class="flex items-center justify-between border-b border-white/10 bg-[#08080a] px-8 py-6 lg:px-16">
		<div>
			<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-500">admin</div>
			<h1 class="font-sans text-2xl font-medium tracking-tighter text-zinc-100 lg:text-3xl">dashboard</h1>
		</div>
		<button
			onclick={logout}
			class="border border-white/10 px-4 py-2 font-mono text-xs text-zinc-500 transition-colors hover:border-white/20 hover:text-zinc-300"
		>
			sign out
		</button>
	</div>

	{#if error}
		<div class="border-b border-red-500/20 bg-red-500/5 px-8 py-4 font-mono text-xs text-red-400">{error}</div>
	{/if}

	<div class="grid grid-cols-1 lg:grid-cols-[340px_1fr]">
		<!-- Chain list -->
		<div class="border-b border-white/10 lg:border-b-0 lg:border-r">
			<div class="border-b border-white/10 p-4">
				<input
					type="text"
					placeholder="search chains…"
					bind:value={searchQuery}
					class="w-full rounded-sm border border-white/10 bg-black/50 px-3 py-2 font-mono text-xs text-zinc-300 placeholder-zinc-600 focus:border-neon/50 focus:outline-none"
				/>
			</div>

			{#if loading}
				<div class="p-6 font-mono text-xs text-zinc-600 animate-pulse">loading chains…</div>
			{:else}
				<div class="max-h-[400px] overflow-y-auto lg:max-h-[640px]">
					{#each filteredChains as chain (chain.chain_id)}
						<div
							role="button"
							tabindex="0"
							onclick={() => (selectedId = chain.chain_id)}
							onkeydown={(e) => e.key === 'Enter' && (selectedId = chain.chain_id)}
							class="flex w-full items-center justify-between border-b border-white/5 px-5 py-3 text-left transition-colors hover:bg-white/[0.03] cursor-pointer {selectedId === chain.chain_id ? 'border-l-2 border-l-neon bg-neon/5' : ''}"
						>
							<div>
								<div class="font-sans text-sm text-zinc-100">{chain.name}</div>
								<div class="font-mono text-[10px] text-zinc-500">id: {chain.chain_id} · {chain.rpcs.length} rpc{chain.rpcs.length === 1 ? '' : 's'}</div>
							</div>
							<button
								onclick={(e) => { e.stopPropagation(); handleRemoveChain(chain.chain_id); }}
								class="ml-2 rounded px-2 py-1 font-mono text-[10px] text-zinc-600 transition-colors hover:bg-red-500/10 hover:text-red-400"
								title="Remove chain"
							>✕</button>
						</div>
					{/each}
					{#if filteredChains.length === 0 && !loading}
						<div class="p-6 font-mono text-xs text-zinc-600">no chains found</div>
					{/if}
				</div>
			{/if}

			<!-- Add chain form -->
			<div class="border-t border-white/10 p-5">
				<div class="mb-3 font-mono text-[10px] uppercase tracking-widest text-zinc-600">add chain</div>
				<form onsubmit={handleAddChain} class="space-y-2">
					<input
						type="number"
						placeholder="chain id"
						bind:value={newChainId}
						required
						class="w-full rounded-sm border border-white/10 bg-black/50 px-3 py-2 font-mono text-xs text-zinc-300 placeholder-zinc-600 focus:border-neon/50 focus:outline-none"
					/>
					<input
						type="text"
						placeholder="chain name"
						bind:value={newChainName}
						required
						class="w-full rounded-sm border border-white/10 bg-black/50 px-3 py-2 font-mono text-xs text-zinc-300 placeholder-zinc-600 focus:border-neon/50 focus:outline-none"
					/>
					<textarea
						placeholder="rpc urls (one per line)"
						bind:value={newChainRpcs}
						rows={3}
						class="w-full rounded-sm border border-white/10 bg-black/50 px-3 py-2 font-mono text-xs text-zinc-300 placeholder-zinc-600 focus:border-neon/50 focus:outline-none resize-none"
					></textarea>
					{#if addChainError}
						<div class="font-mono text-[10px] text-red-400">{addChainError}</div>
					{/if}
					<button
						type="submit"
						disabled={addingChain}
						class="w-full border border-neon/30 py-2 font-mono text-xs text-neon transition-colors hover:bg-neon/10 disabled:opacity-50"
					>
						{addingChain ? 'adding…' : '+ add chain'}
					</button>
				</form>
			</div>
		</div>

		<!-- RPC detail panel -->
		<div class="p-8 lg:p-12">
			{#if selectedChain}
				<div class="mb-6">
					<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-600">chain</div>
					<h2 class="text-xl font-medium tracking-tight text-zinc-100">{selectedChain.name}</h2>
					<div class="mt-1 font-mono text-xs text-zinc-500">chain id: {selectedChain.chain_id}</div>
				</div>

				<!-- RPC list -->
				<div class="mb-6">
					<div class="mb-3 font-mono text-[10px] uppercase tracking-widest text-zinc-600">
						rpc endpoints — {selectedChain.rpcs.length}
					</div>
					{#if selectedChain.rpcs.length === 0}
						<div class="rounded border border-white/5 bg-white/[0.02] p-4 font-mono text-xs text-zinc-600">
							no rpc endpoints configured
						</div>
					{:else}
						<div class="space-y-1">
							{#each selectedChain.rpcs as rpc}
								<div class="flex items-center justify-between gap-3 rounded border border-white/5 bg-white/[0.02] px-4 py-2.5">
									<span class="truncate font-mono text-xs text-zinc-300">{rpc}</span>
									<button
										onclick={() => handleRemoveRpc(selectedChain!.chain_id, rpc)}
										class="shrink-0 font-mono text-[10px] text-zinc-600 transition-colors hover:text-red-400"
										title="Remove RPC"
									>remove</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>

				<!-- Add RPC form -->
				<form onsubmit={handleAddRpc} class="flex gap-2">
					<input
						type="url"
						placeholder="https://…"
						bind:value={newRpcUrl}
						required
						class="flex-1 rounded-sm border border-white/10 bg-black/50 px-3 py-2 font-mono text-xs text-zinc-300 placeholder-zinc-600 focus:border-neon/50 focus:outline-none"
					/>
					<button
						type="submit"
						disabled={addingRpc}
						class="shrink-0 border border-neon/30 px-4 py-2 font-mono text-xs text-neon transition-colors hover:bg-neon/10 disabled:opacity-50"
					>
						{addingRpc ? '…' : '+ add'}
					</button>
				</form>
				{#if rpcError}
					<div class="mt-2 font-mono text-[10px] text-red-400">{rpcError}</div>
				{/if}
			{:else}
				<div class="flex h-full min-h-[300px] flex-col items-center justify-center text-center">
					<div class="mb-3 font-mono text-[10px] uppercase tracking-widest text-zinc-600">select a chain</div>
					<p class="font-mono text-sm text-zinc-500">click any chain on the left to manage its rpc endpoints.</p>
				</div>
			{/if}
		</div>
	</div>

	<Footer />
</div>
