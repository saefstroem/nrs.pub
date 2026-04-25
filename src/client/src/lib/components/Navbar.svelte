<script lang="ts">
	import { X, Menu } from 'lucide-svelte';
	import logo from '$lib/assets/logo.png';

	let mobileOpen = $state(false);
</script>

<header class="sticky top-0 z-50 border-b border-white/10 bg-[#08080a] p-6 sm:relative sm:z-auto">
	<nav class="flex items-center justify-between">
		<a href="/" class="flex items-center">
			<img src={logo} alt="nrs.pub" class="h-8 w-auto" />
		</a>

		<!-- Desktop nav -->
		<div class="hidden items-center gap-6 sm:flex">
			<a href="/chains" class="font-mono text-xs text-zinc-500 transition-colors hover:text-zinc-300">chains</a>
			<a href="https://github.com/saefstroem/nrs.pub" target="_blank" rel="noopener noreferrer" class="font-mono text-xs text-zinc-500 transition-colors hover:text-zinc-300">github</a>
			<a href="https://x.com/saefstroem" target="_blank" rel="noopener noreferrer" class="font-mono text-xs text-zinc-500 transition-colors hover:text-zinc-300">x.com</a>
			<a href="/donate" class="font-mono text-xs text-neon/70 transition-colors hover:text-neon">donate</a>
		</div>

		<!-- Mobile toggle -->
		<button
			onclick={() => (mobileOpen = !mobileOpen)}
			class="text-zinc-400 transition-colors hover:text-zinc-200 sm:hidden"
			aria-label="Toggle menu"
		>
			<Menu size={20} />
		</button>
	</nav>
</header>

<!-- Mobile drawer backdrop -->
{#if mobileOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-40 bg-black/60 sm:hidden"
		onclick={() => (mobileOpen = false)}
	></div>

	<!-- Drawer -->
	<div
		class="fixed right-0 top-0 z-50 flex h-full w-1/2 min-w-[240px] flex-col bg-[#08080a] border-l border-white/10 p-6 sm:hidden"
		style="animation: slideIn 200ms ease forwards;"
	>
		<div class="mb-8 flex items-center justify-between">
			<span class="font-mono text-xs text-zinc-600 uppercase tracking-widest">menu</span>
			<button
				onclick={() => (mobileOpen = false)}
				class="text-zinc-500 transition-colors hover:text-zinc-200"
				aria-label="Close menu"
			>
				<X size={18} />
			</button>
		</div>

		<nav class="flex flex-col gap-5">
			<a href="/" onclick={() => (mobileOpen = false)} class="font-mono text-xs text-zinc-400 transition-colors hover:text-zinc-100">home</a>
			<a href="/chains" onclick={() => (mobileOpen = false)} class="font-mono text-xs text-zinc-400 transition-colors hover:text-zinc-100">chains</a>
			<a href="https://github.com/saefstroem/nrs.pub" target="_blank" rel="noopener noreferrer" onclick={() => (mobileOpen = false)} class="font-mono text-xs text-zinc-400 transition-colors hover:text-zinc-100">github</a>
			<a href="https://x.com/saefstroem" target="_blank" rel="noopener noreferrer" onclick={() => (mobileOpen = false)} class="font-mono text-xs text-zinc-400 transition-colors hover:text-zinc-100">x.com</a>
			<a href="/donate" onclick={() => (mobileOpen = false)} class="font-mono text-xs text-neon/70 transition-colors hover:text-neon">donate</a>
		</nav>

	
	</div>
{/if}

<style>
	@keyframes slideIn {
		from { transform: translateX(100%); }
		to   { transform: translateX(0); }
	}
</style>
