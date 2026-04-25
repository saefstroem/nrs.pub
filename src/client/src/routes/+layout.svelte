<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.png';

	let { children } = $props();

</script>
<svelte:head>
	<title>nrs.pub — node resolution service</title>
	<link rel="icon" href={favicon} />
</svelte:head>

<div
	class="flex min-h-screen flex-col items-center bg-[#08080a] p-4 text-zinc-400 antialiased md:p-8"
>{@render children()}
</div>
