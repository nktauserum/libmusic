<script lang="ts">
    import {onMount} from "svelte";
    import type {Track} from "$lib/types/track";
    import { playerStore } from '$lib/store';

    let tracks: Track[] | null = $state(null);

    async function getTracks(): Promise<Track[]> {
        let response = await fetch("http://localhost:6432/tracks");
        return JSON.parse(await response.text())["tracks"];
    }

    async function playTrack(track_position: number) {
        if (tracks) {
            playerStore.play(
                tracks, track_position
            );
        }
    }

    onMount(async () => {
        tracks = await getTracks();
        console.log(tracks);
    });
</script>

<div class="page">
    <div class="tracks">
        {#if tracks !== null}
            {#each tracks as track, i}
                <button class="track" onclick={() => playTrack(i)} aria-label="track">
                    <span class="number"></span>
                    <img src="{`http://localhost:6432/cover/${track.id}`}" class="track__cover" alt="cover" loading="lazy"/>
                    <span class="title">{ track.title }</span>
                    <span class="artist">{ track.artists.join(', ') }</span>
                    <span class="album">{ track.album }</span>
                    <span class="controls">
                        <a href="#" class="favorite" aria-label="favorite">
                          <svg color="var(--txt-second)" width="28" height="28" viewBox="0 0 32 32">
                            <path fill="currentColor" d="M22.45 6a5.47 5.47 0 0 1 3.91 1.64a5.7 5.7 0 0 1 0 8L16 26.13L5.64 15.64a5.7 5.7 0 0 1 0-8a5.48 5.48 0 0 1 7.82 0l2.54 2.6l2.53-2.58A5.44 5.44 0 0 1 22.45 6m0-2a7.47 7.47 0 0 0-5.34 2.24L16 7.36l-1.11-1.12a7.49 7.49 0 0 0-10.68 0a7.72 7.72 0 0 0 0 10.82L16 29l11.79-11.94a7.72 7.72 0 0 0 0-10.82A7.49 7.49 0 0 0 22.45 4Z"/>
                          </svg>
                        </a>
                        <a href="#" class="tools" aria-label="tools">
                          <svg color="var(--txt-second)" width="32" height="32" viewBox="0 0 24 24">
                            <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 1 1 0-2a1 1 0 0 1 0 2Zm0 7a1 1 0 1 1 0-2a1 1 0 0 1 0 2Zm0 7a1 1 0 1 1 0-2a1 1 0 0 1 0 2Z"/>
                          </svg>
                        </a>
                    </span>
                </button>
            {/each}
        {/if}
    </div>
</div>

<style>
    /* Track */
    .track {
        display: grid;
        grid-template-columns: 0 3.5rem 1fr 1.5fr 1fr auto;
        align-items: center;
        gap: 16px;
        width: 100%;
        height: 5.5rem;

        border: none;
        font-size: 1.2rem;
        color: var(--txt-first);
        border-bottom: 1px solid var(--bg-third);

        cursor: pointer;
        outline: none;
    }

    .tracks {
        overflow: auto;
        height: 100vh;
        border-right: var(--accent-color) 1px solid;
    }

    .track:hover {
        background-color: var(--bg-second);
    }

    .track:hover span, .track:hover .track__cover, .track:hover svg {
        background-color: var(--bg-second);
    }

    .track:hover svg, .track:hover svg * {
        fill: currentColor;
        stroke: currentColor;
    }

    .track > * {
        min-width: 0;
    }

    .track__cover {
        width: 3rem;
        height: 3rem;
        background-color: var(--bg-first);
        border-radius: 10%;
    }

    .title {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        text-align: left;
    }

    .artist {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        color: var(--txt-second);
        text-align: left;
    }

    .album {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        color: var(--txt-second);
        text-align: left;
    }

    .controls {
        justify-self: end;
        display:flex;
        gap:8px;
        align-items:center;
    }

    .controls button {
        background: transparent;
        border: none;
        outline: none;
        cursor: pointer;
    }
</style>