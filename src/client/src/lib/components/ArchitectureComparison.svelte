<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { getChains } from "$lib/api";

	let activeNode = $state(0);
	let phase = $state<"idle" | "to-gateway" | "to-node" | "response">("idle");
	let competitorPhase = $state<"idle" | "to-node" | "response">("idle");
	let interval: NodeJS.Timeout | null = null;
	const cols = 8;
	const rows = 4;
	const nodeCount = cols * rows;

	let totalChains = $state(0);

	function formatNumber(n: number) {
		return n.toLocaleString('en-US');
	}

	function runCycle() {
		phase = "to-gateway";
		competitorPhase = "to-node";

		setTimeout(() => {
			competitorPhase = "response";
		}, 800);

		setTimeout(() => {
			activeNode = Math.floor(Math.random() * nodeCount);
			phase = "to-node";
		}, 600);

		setTimeout(() => {
			phase = "response";
		}, 1200);

		setTimeout(() => {
			phase = "idle";
			competitorPhase = "idle";
		}, 2400);
	}

	onMount(() => {
		getChains().then((ids) => {
			totalChains = ids.length;
		});
		runCycle();
		interval = setInterval(runCycle, 3200);
	});

	onDestroy(() => {
		if (interval) {
			clearInterval(interval);
		}
	});
</script>

<section class="border-b border-white/10 bg-[#08080a]">
	<div class="border-b border-white/10 bg-white/[0.01] p-6">
		<h2 class="font-mono text-xs uppercase tracking-widest text-zinc-400">
			01 — Architecture Comparison
		</h2>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2">
		<!-- Competitors -->
		<div
			class="border-b border-white/10 bg-[#050507] p-8 lg:border-b-0 lg:border-r lg:p-16"
		>
			<div class="mb-8 font-mono text-xs uppercase text-zinc-500">
				Other provider
			</div>

			<pre
				class="mb-12 font-mono text-xs leading-loose text-zinc-400 sm:text-sm"><span
					class="fade {competitorPhase === 'to-node'
						? 'text-red-400'
						: 'text-zinc-400'}">APP</span
				>
 <span
					class="inline-block fade {competitorPhase === 'to-node'
						? 'text-red-400'
						: 'text-zinc-600'}">│</span
				>
 <span
					class="inline-block fade {competitorPhase === 'to-node'
						? 'text-red-400 pulse'
						: 'text-zinc-600'}">▼</span
				>
<span
					class="inline-block px-2 py-1 fade
	{competitorPhase === 'to-node' || competitorPhase === 'response'
						? 'bg-red-500/20 text-red-400'
						: 'bg-white/5 text-zinc-100'}">FIXED NODE</span
				> <span class="text-zinc-600">← (single point of failure)</span>
 <span
					class="inline-block fade {competitorPhase === 'response'
						? 'text-red-400'
						: 'text-zinc-600'}">│</span
				>
 <span
					class="inline-block fade {competitorPhase === 'response'
						? 'text-red-400 pulse'
						: 'text-zinc-600'}">▼</span
				>
<span
					class="fade {competitorPhase === 'response'
						? 'text-red-400'
						: 'text-zinc-400'}">APP</span
				></pre>

			<ul
				class="space-y-3 border-l border-white/10 pl-4 font-mono text-xs text-zinc-500"
			>
				<li>&gt; static node — single point of failure</li>
				<li>&gt; node gets compromised, protocol gets drained</li>
				<li>&gt; see: kelp dao, $290M (april 2026)</li>
			</ul>
		</div>

		<!-- AutoRPC -->
		<div class="bg-[#08080a] p-8 lg:p-16">
			<div class="mb-8 font-mono text-xs uppercase text-neon">
				nrs.pub
			</div>

			<div class="mb-12 font-mono text-xs sm:text-sm">
				<div class="mb-4 flex items-center gap-4">
					<span
						class="fade {phase === 'to-gateway'
							? 'text-neon'
							: 'text-zinc-300'}">APP</span
					>

					<span
						class="fade {phase === 'to-gateway'
							? 'text-neon'
							: 'text-zinc-500'}">──▶</span
					>

					<span
						class="border px-2 py-1 fade text-neon
							{phase === 'to-gateway' || phase === 'to-node'
							? 'border-neon bg-neon/20 shadow-[0_0_12px_var(--color-neon)]'
							: 'border-neon/20 bg-neon/10'}">nrs.pub (307)</span
					>

					<span
						class="fade {phase === 'to-node'
							? 'text-neon'
							: 'text-zinc-500'}">──▶</span
					>

					<span
						class="question-mark {phase === 'to-node'
							? 'text-neon'
							: 'text-zinc-500 dormant'}">?</span
					>
				</div>

				<div
					class="my-4 rounded border border-white/5 bg-white/[0.02] p-4"
				>
					<div class="mb-3 flex items-center justify-between">
						<span
							class="text-[10px] uppercase tracking-widest text-zinc-500"
							>node pool</span
						>
						<span class="text-[10px] text-zinc-600"
							>{totalChains ? formatNumber(totalChains) + ' chains supported' : 'loading…'}</span
						>
					</div>

					<div class="grid grid-cols-8 gap-1.5">
						{#each Array(nodeCount) as _, i}
							{@const isActive =
								activeNode === i &&
								(phase === "to-node" || phase === "response")}
							<div
								class="flex h-5 items-center justify-center rounded-sm node-cell
									{isActive ? 'bg-neon shadow-[0_0_8px_var(--color-neon)]' : 'bg-white/5'}"
							>
								<span
									class="inline-block h-1.5 w-1.5 rounded-full node-dot
										{isActive ? 'bg-neon' : 'bg-zinc-700'}"
								></span>
							</div>
						{/each}
					</div>

					<div class="mt-3 text-[10px] text-zinc-600">
						+ with monitoring of nodes
					</div>
				</div>

				<div class="flex items-center gap-4">
					<span
						class="question-mark {phase === 'response'
							? 'text-neon'
							: 'text-zinc-500 dormant'}">?</span
					>

					<span
						class="fade {phase === 'response'
							? 'text-neon'
							: 'text-zinc-500'}">──▶</span
					>

					<span
						class="fade {phase === 'response'
							? 'text-neon'
							: 'text-zinc-300'}">APP</span
					>

					<span
						class="text-xs fade
						{phase === 'response' ? 'text-neon/60' : 'text-zinc-500'}"
						>← (direct response)</span
					>
				</div>
			</div>

			<ul
				class="space-y-3 border-l border-neon/30 pl-4 font-mono text-xs text-zinc-300"
			>
				<li>
					<span class="text-neon">&gt;</span> randomized node selection
					per request
				</li>
				<li>
					<span class="text-neon">&gt;</span> no fixed endpoint to compromise
				</li>
				<li>
					<span class="text-neon">&gt;</span> attacker must compromise all
					providers simultaneously
				</li>
				<li>
					<span class="text-neon">&gt;</span> zero payload logging
				</li>
			</ul>
		</div>
	</div>
</section>

<style>
	.fade {
		transition:
			color 400ms ease,
			background-color 400ms ease,
			border-color 400ms ease,
			box-shadow 400ms ease,
			opacity 400ms ease;
	}

	.node-cell {
		transition:
			background-color 300ms ease,
			box-shadow 300ms ease;
		will-change: background-color, box-shadow;
	}

	.node-dot {
		transition: background-color 300ms ease;
	}

	@keyframes soft-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}

	.pulse {
		animation: soft-pulse 0.8s ease-in-out infinite;
	}

	.question-mark {
		animation: soft-pulse 1.2s ease-in-out infinite;
		transition:
			color 400ms ease,
			opacity 400ms ease;
	}

	.question-mark.dormant {
		animation-play-state: paused;
		opacity: 0.4;
	}
</style>
