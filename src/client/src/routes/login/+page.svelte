<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';

	let password = $state('');
	let loading = $state(false);
	let error = $state('');

	async function handleLogin() {
		error = '';
		loading = true;
		try {
			const res = await fetch('/api/v1/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ password }),
			});
			if (res.ok) {
				const { token } = await res.json();
				localStorage.setItem('nrs_token', token);
				window.location.href = '/dashboard';
			} else {
				const text = await res.text();
				error = text || 'Invalid password.';
			}
		} catch {
			error = 'Could not reach the server.';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>nrs.pub — Login</title>
</svelte:head>

<div class="flex w-full max-w-7xl flex-col border border-white/10 bg-[#050507]">
	<Navbar />

	<div class="flex flex-1 items-center justify-center border-b border-white/10 bg-[#08080a] px-8 py-24">
		<div class="w-full max-w-sm">
			<div class="mb-2 font-mono text-[10px] uppercase tracking-widest text-zinc-500">admin</div>
			<h1 class="mb-8 font-sans text-3xl font-medium tracking-tighter text-zinc-100">
				sign in
			</h1>

			<div class="space-y-4">
				<div>
					<label for="password" class="mb-2 block font-mono text-[10px] uppercase tracking-widest text-zinc-600">
						password
					</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						autocomplete="current-password"
						onkeydown={(e) => { if (e.key === 'Enter' && password) handleLogin(); }}
						class="w-full rounded-sm border border-white/10 bg-black/50 px-4 py-3 font-mono text-sm text-zinc-300 placeholder-zinc-600 transition-all focus:border-neon/50 focus:outline-none focus:ring-1 focus:ring-neon/50"
						placeholder="••••••••"
					/>
				</div>

				{#if error}
					<div class="rounded border border-red-500/20 bg-red-500/5 px-4 py-3 font-mono text-xs text-red-400">
						{error}
					</div>
				{/if}

				<button
					type="button"
					disabled={loading || !password}
					onclick={handleLogin}
					class="flex w-full items-center justify-center border border-neon/30 bg-[#030304] p-3 font-mono text-sm text-neon transition-colors hover:bg-neon/10 disabled:opacity-50"
				>
					{#if loading}
						<span class="animate-pulse">authenticating…</span>
					{:else}
						<span class="uppercase tracking-widest">sign in</span>
					{/if}
				</button>
			</div>
		</div>
	</div>

	<Footer />
</div>