<script lang="ts">
	import { onMount } from 'svelte';
	import { getChains, getAllChainDetails, isDegraded, type ChainDetail } from '$lib/api';

	let chains = $state<ChainDetail[]>([]);

	onMount(() => {
		getChains()
			.then((ids) => getAllChainDetails(ids))
			.then((all) => {
				chains = all.filter((c) => !isDegraded(c));
			});
	});
</script>

<section
	class="relative overflow-hidden border-b border-white/10 bg-[#030304] px-4 py-6 font-mono text-xs text-zinc-600 sm:text-sm"
>
	<div
		class="pointer-events-none absolute bottom-0 left-0 top-0 z-10 w-16 bg-gradient-to-r from-[#030304] to-transparent"
	></div>
	<div
		class="pointer-events-none absolute bottom-0 right-0 top-0 z-10 w-16 bg-gradient-to-l from-[#030304] to-transparent"
	></div>
	{#if chains.length > 0}
		<p class="inline-block whitespace-nowrap">
			<span class="text-zinc-400">operational chains — {chains.length}</span>
			{#each chains as chain}
				&nbsp;· {chain.chain_info.name}
			{/each}
		</p>
	{:else}
		<p class="inline-block whitespace-nowrap">
			loading chains...
		</p>
	{/if}
</section>
