<script lang="ts">
	import '../app.css';
	import Player from '\$lib/components/Player.svelte';
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
	<div class="sources"></div>
	<div class="playlist">
		{@render children()}
	</div>
</div>

<style>
	.content {
		width: 100%;
		height: 100vh;
		background-color: var(--bg-second);
	}
</style>