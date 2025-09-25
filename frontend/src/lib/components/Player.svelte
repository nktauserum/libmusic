<script lang="ts">
    import { playerStore } from '$lib/store';

    let audio: HTMLAudioElement | null = null;
    let currentTime: number = 0;
    let duration: number = 0;
    let volume: number = 1;

    function formatTime(time: number): string {
        if (isNaN(time)) return '00:00';
        const minutes = Math.floor(time / 60);
        const seconds = Math.floor(time % 60);
        return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }

    $: if (audio && $playerStore.track) {
        if ($playerStore.isPlaying) {
            audio.play().catch(console.error);
        } else {
            audio.pause();
        }
    }
</script>

<div class="player">
    <div class="track">
        <div class="track__cover"></div>
        {#if $playerStore.track}
        <div class="track__info" >
            <div class="info__title">{$playerStore.track?.title}</div>
            <a href="/artist" class="info__artist">{$playerStore.track?.artists.join(", ")}</a>
        </div>
        {/if}
    </div>

    {#if $playerStore.track}
    <audio
            bind:this={audio}
            src={`http://localhost:6432/track/${$playerStore.track?.id}`}
            bind:currentTime
            bind:duration
            bind:volume
            autoplay
    ></audio>

    <div class="progress">
        <span>{formatTime(currentTime)}</span>
        <input type="range" bind:value={currentTime} min="0" max={duration} step="0.1" />
        <span>{formatTime(duration)}</span>
    </div>

    <div class="controls">
        <button class="ctrl" aria-label="previous">
            <svg color="var(--txt-second)" width="32" height="32" viewBox="0 0 16 16"><path fill="currentColor" d="M2 2.5a.5.5 0 0 1 1 0v11a.5.5 0 0 1-1 0v-11Zm12 .502a1 1 0 0 0-1.579-.816l-7 4.963a1 1 0 0 0-.006 1.628l7 5.037A1 1 0 0 0 14 13.003V3.002ZM6 7.965l7-4.963v10L6 7.966Z"/></svg>
        </button>

        <button
                class="ctrl play"
                aria-label={$playerStore.isPlaying ? 'Pause' : 'Play' }
                on:click={() => playerStore.togglePlayback()}
        >
            {#if $playerStore.isPlaying}
                <!-- Pause icon -->
                <svg
                        color="var(--txt-first)"
                        width="50"
                        height="50"
                        viewBox="0 0 16 16">
                    <path fill="currentColor" d="M5.5 3.5A1.5 1.5 0 0 1 7 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5zm5 0A1.5 1.5 0 0 1 12 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5z"/>
                </svg>
            {:else}
                <!-- Play icon -->
                <svg
                        color="var(--txt-first)"
                        width="50"
                        height="50"
                        viewBox="0 0 16 16">
                    <path fill="currentColor" d="M10.804 8L5 4.633v6.734L10.804 8zm.792-.696a.802.802 0 0 1 0 1.392l-6.363 3.692C4.713 12.69 4 12.345 4 11.692V4.308c0-.653.713-.998 1.233-.696l6.363 3.692z"/>
                </svg>
            {/if}
        </button>

        <button class="ctrl" aria-label="next">
            <svg color="var(--txt-second)" width="32" height="32" viewBox="0 0 16 16"><path fill="currentColor" d="M14 2.5a.5.5 0 1 0-1 0v11a.5.5 0 0 0 1 0v-11ZM2 3.002a1 1 0 0 1 1.579-.816l7 4.963a1 1 0 0 1 .006 1.628l-7 5.037A1 1 0 0 1 2 13.003V3.002Zm8 4.963L3 3.002v10l7-5.037Z"/></svg>
        </button>
    </div>

    <div class="volume">
        <label>Громкость:</label>
        <input type="range" bind:value={volume} min="0" max="1" step="0.01" />
    </div>

    {/if}
</div>

<style>
    .progress {
        display: flex;
        align-items: center;
        margin-top: 10px;
    }

    input[type="range"] {
        flex: 1;
        margin: 0 2.5rem;
    }

    button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 6px;
        background: transparent;
        border: none;
        cursor: pointer;
        border-radius: 6px;
    }

    button:focus{
        outline: none;
    }

    .controls .ctrl:hover{
        background: rgba(0,0,0,0.06);
    }

    .controls svg{
        display: block;
        pointer-events: none;
    }

    .controls {
        justify-self: center;
        display:flex;
        gap:10px;
        align-items: center;
    }

    .track__info {
        justify-content: center;
        font-size: 1rem;
    }

    .info__artist {
        color: var(--txt-second);
        font-size: 1.2rem;
    }

    .info__title {
        font-size: 1.4rem;
        color: var(--txt-first);
        margin-bottom: 5px;
    }

    .track__cover {
        width: 20rem;
        height: 20rem;
        margin: 2.5rem 0;
        border-radius: 5%;
        background-color: var(--bg-third);
    }

    .track {
        justify-self: center;
        text-align: center;
    }

    .player {
        position: relative;
        background: var(--bg-first);
        z-index: 1000;

        height: 100%;
        min-width: 25rem;

        display: grid;
        grid-template-rows: auto auto auto 1fr;
        gap: 16px;
    }
</style>