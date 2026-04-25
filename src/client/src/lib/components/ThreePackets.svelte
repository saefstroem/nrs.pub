<script lang="ts">
	import { getChains, getAllChainDetails, isDegraded, type ChainDetail } from '$lib/api';
	import { onMount } from 'svelte';
	import { Check, X, MoveRight } from 'lucide-svelte';

	let chains: ChainDetail[] = [];

	onMount(async () => {
		try {
			const ids = await getChains();
			const all = await getAllChainDetails(ids);
			chains = all.filter((c) => !isDegraded(c));
		} catch (e) {
			console.error('Failed to fetch chains:', e);
		}
	});
</script>

<section class="border-b border-white/10 bg-[#08080a]">
	<div class="border-b border-white/10 bg-white/[0.01] p-6">
		<h2 class="font-mono text-xs uppercase tracking-widest text-zinc-400">
			02 — Why nrs.pub
		</h2>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2">
		<!-- Left: Robustness + Security -->
		<div class="border-b border-white/10 bg-[#050507] lg:border-b-0 lg:border-r">
			<!-- Robustness -->
			<div class="border-b border-white/10 p-8 lg:p-12">
				<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">01</div>
				<div class="mb-4 font-mono text-sm uppercase tracking-wider text-zinc-100">Robustness</div>
				<p class="mb-6 font-mono text-sm leading-relaxed text-zinc-400">
					your rpc provider goes down. your app goes down with it.
						nrs.pub routes across <span class="text-zinc-100">every public node</span> on every chain.
					if one provider is down, the next request hits another. no failover config needed. 
				</p>
				<div class="font-mono text-xs leading-loose text-zinc-500 space-y-0.5">
					<div class="flex items-center gap-2">node_14  <span class="flex items-center gap-1 text-red-400"><X size={12} />down</span></div>
					<div class="flex items-center gap-2">node_07  <span class="flex items-center gap-1 text-red-400"><X size={12} />down</span></div>
					<div class="flex items-center gap-2">node_31  <span class="flex items-center gap-1 text-neon"><Check size={12} />routed</span></div>
					<div class="text-zinc-600">— automatic, per request</div>
				</div>
			</div>

			<!-- Security -->
			<div class="p-8 lg:p-12">
				<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">02</div>
				<div class="mb-4 font-mono text-sm uppercase tracking-wider text-neon">Security</div>
				<p class="mb-6 font-mono text-sm leading-relaxed text-zinc-400">
					a compromised rpc node feeds your verifier false state. your protocol acts on it.
						funds are gone. nrs.pub selects a <span class="text-neon">random node every request</span>.
					an attacker cannot predict or control which node serves your safety-critical call.
				</p>
				<div class="font-mono text-xs leading-loose text-zinc-500 space-y-0.5">
					<div class="flex items-center gap-2">call 1  <MoveRight size={12} /> <span class="text-zinc-300">node_22</span></div>
					<div class="flex items-center gap-2">call 2  <MoveRight size={12} /> <span class="text-zinc-300">node_09</span></div>
					<div class="flex items-center gap-2">call 3  <MoveRight size={12} /> <span class="text-zinc-300">node_41</span></div>
					<div class="text-zinc-600">- no pattern, no fixed target</div>
				</div>
			</div>
		</div>

		<!-- Right: Simplicity -->
		<div class="bg-[#08080a] p-8 lg:p-12">
			<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">03</div>
			<div class="mb-4 font-mono text-sm uppercase tracking-wider text-zinc-100">Simplicity</div>
			<p class="mb-8 font-mono text-sm leading-relaxed text-zinc-400">
				no sdk. no config. no provider-specific setup. one url. every chain.
				swap your rpc endpoint — everything else stays the same.
			</p>

			<div class="mb-8 rounded border border-white/5 bg-white/[0.02] p-6">
				<div class="mb-4 font-mono text-[10px] uppercase tracking-widest text-zinc-600">before — single provider, single point of failure</div>
				<pre class="overflow-x-auto font-mono text-sm leading-relaxed text-zinc-500">const rpc =
  "<span class="text-zinc-400">https://eth-mainnet.g.alchemy.com/v2/KEY</span>"</pre>
			</div>

			<div class="mb-8 rounded border border-neon/10 bg-neon/5 p-6">
						<div class="mb-4 font-mono text-[10px] uppercase tracking-widest text-neon/60">after — all providers, one free endpoint</div>
						<pre class="overflow-x-auto font-mono text-sm leading-relaxed text-zinc-300">const rpc =
  "<span class="text-neon">https://nrs.pub/1</span>"</pre>
			</div>

			<p class="mb-8 font-mono text-sm leading-relaxed text-zinc-400">
				works with anything that follows HTTP redirects —
				ethers.js, viem, web3.py, curl.
			</p>

			<div class="rounded border border-white/5 bg-white/[0.02] p-6">
				<div class="mb-3 font-mono text-[10px] uppercase tracking-widest text-zinc-600">
					operational chains — {chains.length || '…'}
				</div>
				<p class="max-h-32 overflow-y-auto font-mono text-xs leading-loose text-zinc-500">
					{#if chains.length}
						{#each chains as chain, i}
							{#if i === 0}
									<span class="text-zinc-300">{chain.chain_info.name.toLowerCase()}</span>
								{:else}
									<span> · {chain.chain_info.name.toLowerCase()}</span>
							{/if}
						{/each}
					{:else}
						<span class="animate-pulse text-zinc-600">loading chains…</span>
					{/if}
				</p>
			</div>
		</div>
	</div>
</section>