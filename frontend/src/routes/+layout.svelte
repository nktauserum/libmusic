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

<Player />
<div class="content">
	<div class="sources">
		<Playlists />
	</div>
	<div class="playlist">
		{@render children()}
	</div>
</div>
<Queue />

<style>
	.content {
		width: 100%;
		height: 100vh;
		display: grid;
		grid-template-columns: 20rem 1fr;
		gap: 1rem;
		align-items: start;
		padding: 1rem 0;
	}
</style>