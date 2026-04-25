<script lang="ts">
	import LiveTrace from './LiveTrace.svelte';
</script>

<section class="grid grid-cols-1 border-b border-white/10 bg-[#08080a] lg:grid-cols-2">
	<div
		class="flex flex-col justify-center border-b border-white/10 p-8 lg:border-b-0 lg:border-r lg:p-16"
	>
		<h1
			class="mb-6 text-5xl font-medium leading-none tracking-tighter text-zinc-100 lg:text-7xl"
		>
			rpc without a<br /> single point of failure.
		</h1>
		<p class="mb-12 max-w-md text-lg leading-relaxed text-zinc-400 lg:text-xl">
			nrs.pub is a free node resolution service. get the best public rpc for any chain: no keys, no limits.
		</p>

		<a
			href="/chains"
			class="flex w-fit items-center border border-neon/30 bg-[#030304] p-3 font-mono text-sm text-neon transition-colors hover:bg-neon/10"
		>
			<span class="uppercase tracking-widest">browse chains</span>
		</a>
	</div>

	<LiveTrace />
</section>
