<script lang="ts">
	import { isDegraded, type ChainDetail } from '$lib/api';
	import CopyButton from './CopyButton.svelte';

	let { chains = [], loading = false }: { chains: ChainDetail[]; loading?: boolean } = $props();
</script>

{#if loading}
	<div class="grid grid-cols-1 gap-px bg-white/10 sm:grid-cols-2 lg:grid-cols-3">
		{#each Array(12) as _}
			<div class="flex items-center justify-between bg-[#050507] p-6">
				<div>
					<div class="mb-1 h-4 w-32 animate-pulse rounded bg-white/5"></div>
					<div class="h-3 w-24 animate-pulse rounded bg-white/5"></div>
				</div>
				<div class="h-3 w-16 animate-pulse rounded bg-white/5"></div>
			</div>
		{/each}
	</div>
{:else}
	<div class="grid grid-cols-1 gap-px bg-white/10 sm:grid-cols-2 lg:grid-cols-3">
		{#each chains as chain}
			<div
				class="group flex items-center justify-between bg-[#050507] p-6 transition-colors hover:bg-white/[0.02]"
			>
				<div>
					<div class="mb-1 font-sans text-sm font-medium text-zinc-100">
						{chain.chain_info.name}
					</div>
					<div class="font-mono text-xs text-zinc-500">
						chain id: {chain.chain_info.chain_id}
					</div>
				</div>
				<div class="flex flex-col items-end gap-1.5">
					{#if isDegraded(chain)}
						<span class="font-mono text-xs text-amber-500">degraded</span>
					{:else}
						<span class="font-mono text-xs text-neon">operational</span>
					{/if}
					<div class="flex items-center gap-1 font-mono text-[10px] text-zinc-500">
						<span>nrs.pub/{chain.chain_info.chain_id}</span>
						<CopyButton text="https://nrs.pub/{chain.chain_info.chain_id}" />
					</div>
				</div>
			</div>
		{/each}
	</div>
{/if}
