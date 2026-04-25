<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Hero from '$lib/components/Hero.svelte';
	import ArchitectureComparison from '$lib/components/ArchitectureComparison.svelte';
	import ThreePackets from '$lib/components/ThreePackets.svelte';
	import ChainsBanner from '$lib/components/ChainsBanner.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<main class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />
	<Hero />
	<ArchitectureComparison />
	<ThreePackets />
	<ChainsBanner />
	<Footer />
</main>
