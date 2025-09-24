<script lang="ts">
	import '../app.css';
	import Player from '\$lib/components/Player.svelte';

	let { children } = $props();
</script>

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