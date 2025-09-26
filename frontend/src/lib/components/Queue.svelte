<script lang="ts">
    import { playerStore } from '$lib/store';

    async function playTrack(track_position: number) {
        if ($playerStore.queue) {
            playerStore.play(
                $playerStore.queue, track_position
            );
        }
    }
</script>

<div class="queue">
    <div class="tracks">
        {#if $playerStore.queue !== null}
            {#each $playerStore.queue as track, i}
                {#if $playerStore.current_pos === i}
                <button class="track playing" onclick={() => playTrack(i)} aria-label="track">
                    <span class="number">{i + 1}</span>
                    <span class="info">{ track.title } - { track.artists.join(', ') }</span>
                </button>
                {:else}
                <button class="track" onclick={() => playTrack(i)} aria-label="track">
                    <span class="number">{i + 1}</span>
                    <span class="info">{ track.title } - { track.artists.join(', ') }</span>
                </button>
                {/if}
            {/each}
        {/if}
    </div>
</div>

<style>
    .tracks {
        display: flex;
        flex-direction: column;
    }

    .playing, .playing span {
        background: var(--bg-second);
    }

    .track {
        width: 100%;
        display: flex;
        align-items: center;
        text-align: left;
        margin-bottom: 1rem;
        padding: 1rem;
        font-size: 1.2rem;
        cursor: pointer;
    }

    .number {
        margin-right: 1rem;
    }

    .queue {
        background: var(--bg-first);
        z-index: 1000;
        height: 100%;
        min-width: 20rem;
    }
</style>