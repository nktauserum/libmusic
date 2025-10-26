<script lang="ts">
	import '../app.css';
	import Player from '\$lib/components/Player.svelte';
	import Queue from '\$lib/components/Queue.svelte';
	import Playlists from '$lib/components/Playlists.svelte';
	import { playerStore } from '$lib/store';

	let { children } = $props();

	function title(): string {
		if ($playerStore.track) {
			return `${$playerStore.track.title} - ${$playerStore.track.artists.join(", ")} | libmusic`
		} else {
			return "libmusic";
		}
	}
</script>

<svelte:head>
	<title>{title()}</title>
</svelte:head>

<div class="app">
	<Player />
	<Playlists />
	<div class="playlist">
		{@render children()}
	</div>
	<Queue />
</div>

<style>
	.app {
		width: 100%;
		height: 100vh;
		display: grid;
		/* player | playlists | tracks (main) | queue */
		grid-template-columns: 25rem 18rem 1fr 20rem;
		gap: 1rem;
		align-items: start;
	}
</style>