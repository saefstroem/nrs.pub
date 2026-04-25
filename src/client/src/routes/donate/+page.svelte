<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import CopyButton from '$lib/components/CopyButton.svelte';

	const WALLET = '0x0000000000000000000000000000000000000000';
</script>

<svelte:head>
	<title>nrs.pub — Donate</title>
</svelte:head>

<div class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />

	<!-- Header -->
	<div class="border-b border-white/10 bg-[#08080a] p-8 lg:p-16">
		<div class="mb-4 font-mono text-xs uppercase tracking-widest text-zinc-500">support nrs.pub</div>
		<h1 class="mb-4 font-sans text-3xl font-medium tracking-tighter text-zinc-100 lg:text-5xl">
			keep it free.
		</h1>
		<p class="max-w-xl text-lg leading-relaxed text-zinc-400">
			nrs.pub is a completely free, open-source node resolution service. no plans, no keys, no paywalls.
			if it's useful to you, consider supporting its operation.
		</p>
	</div>

	<!-- What we do -->
	<div class="grid grid-cols-1 border-b border-white/10 lg:grid-cols-2">
		<div class="border-b border-white/10 p-8 lg:border-b-0 lg:border-r lg:p-12">
			<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">what nrs.pub provides</div>
			<ul class="mt-6 space-y-4 font-mono text-sm text-zinc-400">
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-neon">&gt;</span>
					<span>free rpc routing for every public evm chain</span>
				</li>
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-neon">&gt;</span>
					<span>random node selection per request to prevent targeted attacks</span>
				</li>
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-neon">&gt;</span>
					<span>24/7 monitoring with live latency and uptime stats</span>
				</li>
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-neon">&gt;</span>
					<span>no rate limits, no registration, no api keys</span>
				</li>
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-neon">&gt;</span>
					<span>always open-source and transparent</span>
				</li>
			</ul>
		</div>

		<div class="p-8 lg:p-12">
			<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-600">what donations fund</div>
			<ul class="mt-6 space-y-4 font-mono text-sm text-zinc-400">
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-zinc-300">&gt;</span>
					<span>server infrastructure and bandwidth</span>
				</li>
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-zinc-300">&gt;</span>
					<span>continuous development and new chain support</span>
				</li>
				<li class="flex items-center gap-3">
					<span class="mt-0.5 text-zinc-300">&gt;</span>
					<span>keeping the service free for everyone, forever</span>
				</li>
			</ul>
		</div>
	</div>

	<!-- Donation address -->
	<div class="border-b border-white/10 bg-[#030304] p-8 lg:p-12">
		<div class="mb-4 font-mono text-[10px] uppercase tracking-widest text-zinc-600">donate with eth</div>
		<div class="flex items-center gap-3 rounded border border-neon/20 bg-neon/5 p-5">
			<span class="break-all font-mono text-sm text-neon">{WALLET}</span>
			<CopyButton text={WALLET} />
		</div>
		<p class="mt-4 font-mono text-xs text-zinc-600">
			any erc-20 compatible token or native eth accepted. donations go directly to operational costs.
		</p>
	</div>

	<Footer />
</div>
