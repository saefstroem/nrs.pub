<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { getChain, type ChainDetail } from '$lib/api';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	const chainId = Number($page.params.chain_id);
	const endpoint = `https://nrs.pub/${chainId}`;

	let detail = $state<ChainDetail | null>(null);
	let error = $state('');

	onMount(async () => {
		try {
			detail = await getChain(chainId);
		} catch {
			error = 'Chain not found.';
		}
	});
</script>

<svelte:head>
	<title>nrs.pub — {detail ? detail.chain_info.name : `chain ${chainId}`} gateway</title>
</svelte:head>

<div class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />

	<div class="p-8 lg:p-16">
		{#if error}
			<div class="font-mono text-sm text-red-400">{error}</div>
		{:else if detail}
			<div class="mb-1 font-mono text-[10px] uppercase tracking-widest text-zinc-500">rpc gateway</div>
			<h1 class="mb-1 font-sans text-3xl font-medium tracking-tighter text-zinc-100 lg:text-5xl">
				{detail.chain_info.name}
			</h1>
			<div class="mb-10 font-mono text-xs text-zinc-500">chain id: {chainId}</div>

			<div class="space-y-6 max-w-xl">
				<div>
					<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">your rpc endpoint</div>
					<div class="flex items-center gap-2 border border-white/10 bg-black/50 px-4 py-3">
						<span class="flex-1 truncate font-mono text-sm text-neon">{endpoint}</span>
						<CopyButton text={endpoint} />
					</div>
				</div>

				<div>
					<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">metamask / wallet</div>
					<div class="border border-white/10 bg-black/50 px-4 py-3 font-mono text-xs text-zinc-400">
						Settings → Networks → Add network → RPC URL: <span class="text-neon">{endpoint}</span>
					</div>
				</div>

				<div>
					<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">ethers.js / viem / web3.py</div>
					<div class="border border-white/10 bg-black/50 px-4 py-3 font-mono text-xs text-zinc-400">
						const provider = new JsonRpcProvider("<span class="text-neon">{endpoint}</span>")
					</div>
				</div>

				<div>
					<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">curl</div>
					<div class="overflow-x-auto border border-white/10 bg-black/50 px-4 py-3 font-mono text-xs text-zinc-400 leading-relaxed">
						<pre class="whitespace-pre">curl -X POST {endpoint} \
  -H 'Content-Type: application/json' \
  -d '{`{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}`}'</pre>
					</div>
				</div>
			</div>
		{:else}
			<div class="font-mono text-xs text-zinc-600 animate-pulse">loading…</div>
		{/if}
	</div>

	<Footer />
</div>
