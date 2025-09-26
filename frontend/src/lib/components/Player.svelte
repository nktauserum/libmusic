<script lang="ts">
    import { playerStore } from '$lib/store';

    let audio: HTMLAudioElement | null = null;
    let currentTime: number = 0;
    let duration: number = 0;
    let volume: number = 0.5;

    function formatTime(time: number): string {
        if (isNaN(time)) return '00:00';
        const minutes = Math.floor(time / 60).toString().padStart(2, '0');
        const seconds = Math.floor(time % 60).toString().padStart(2, '0');
        return `${minutes}:${seconds}`;
    }

    function next() {
        playerStore.next();
    }

    function prev() {
        playerStore.prev();
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
        {#if $playerStore.track}
        <img src="{`http://localhost:6432/cover/${$playerStore.track?.id}`}" class="track__cover" alt="cover" />
        <div class="track__info" >
            <div class="info__title">{$playerStore.track?.title}</div>
            <a href="/artist" class="info__artist">{$playerStore.track?.artists.join(", ")}</a>
        </div>
        {:else}
        <div class="track__cover"></div>
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

    <div class="audio-progress">
        <input
                type="range"
                class="audio-progress__slider"
                bind:value={currentTime}
                min="0"
                max={duration}
                step="0.1"
        />
        <div class="audio-progress__time-display">
            <time class="audio-progress__current-time">{formatTime(currentTime)}</time>
            <time class="audio-progress__total-time">{formatTime(duration)}</time>
        </div>
    </div>


        <div class="controls">
        <button class="ctrl" aria-label="previous"  on:click="{() => prev()}">
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

        <button class="ctrl" aria-label="next" on:click="{() => next()}">
            <svg color="var(--txt-second)" width="32" height="32" viewBox="0 0 16 16"><path fill="currentColor" d="M14 2.5a.5.5 0 1 0-1 0v11a.5.5 0 0 0 1 0v-11ZM2 3.002a1 1 0 0 1 1.579-.816l7 4.963a1 1 0 0 1 .006 1.628l-7 5.037A1 1 0 0 1 2 13.003V3.002Zm8 4.963L3 3.002v10l7-5.037Z"/></svg>
        </button>
    </div>

    <div class="volume">
        <button class="volume__btn volume__btn--min" aria-label="Mute">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path d="M11 5v14l-6-6H2V11h3l6-6z" fill="currentColor"/>
            </svg>
        </button>

        <input
                class="volume__slider"
                type="range"
                bind:value={volume}
                min="0"
                max="1"
                step="0.01"
                aria-label="Volume"
        />

        <button class="volume__btn volume__btn--max" aria-label="Max volume">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path d="M11 5v14l-6-6H2V11h3l6-6z" fill="currentColor"/>
                <path d="M16.5 8.5a4.5 4.5 0 010 7" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round"/>
                <path d="M19 6a7 7 0 010 12" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round"/>
            </svg>
        </button>
    </div>
    {/if}
</div>

<style>
    .audio-progress__slider {
        -webkit-appearance: none;
        appearance: none;
        width: calc(100% - 5rem);
        margin: 0 2.5rem;
        height: 5px;
        background: var(--txt-first, #ccc);
        opacity: 0.9;
        outline: none;
        transition: opacity 0.2s;
        border-radius: 5px;
    }

    .audio-progress__slider:hover {
        opacity: 1;
    }

    .audio-progress__slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 15px;
        height: 15px;
        background: var(--txt-first, #000);
        cursor: pointer;
        border-radius: 50%;
        box-shadow: 0 0 5px rgba(0, 0, 0, 0.3);
    }

    .audio-progress__slider::-moz-range-thumb {
        width: 15px;
        height: 15px;
        background: var(--txt-first, #000);
        cursor: pointer;
        border-radius: 50%;
        border: none;
        box-shadow: 0 0 5px rgba(0, 0, 0, 0.3);
    }

    .audio-progress__slider::-moz-range-progress {
        background: var(--txt-first, #ccc);
        height: 5px;
    }

    .audio-progress__time-display {
        display: grid;
        margin: 0 2.5rem;
        grid-template-columns: 1fr auto;
    }

    .audio-progress__current-time {
        justify-self: start;
    }

    .audio-progress__total-time {
        justify-self: end;
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

    .volume {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin: 0 2.5rem;
    }

    .volume__btn {
        background: transparent;
        border: none;
        padding: 0.25rem;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        color: var(--txt-first);
        cursor: pointer;
    }
    .volume__btn:focus {
        outline: 2px solid color-mix(in srgb, var(--txt-first) 40%, transparent);
        outline-offset: 2px;
    }

    .volume__slider {
        -webkit-appearance: none;
        appearance: none;
        width: calc(100% - 3.25rem);
        height: 6px;
        background: color-mix(in srgb, var(--txt-first) 30%, transparent);
        border-radius: 6px;
        outline: none;
    }

    .volume__slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 14px;
        height: 14px;
        background: var(--txt-first);
        border-radius: 50%;
        box-shadow: 0 0 4px rgba(0,0,0,0.25);
        cursor: pointer;
        margin-top: -4px;
    }

    .volume__slider::-moz-range-thumb {
        width: 14px;
        height: 14px;
        background: var(--txt-first);
        border-radius: 50%;
        border: none;
        box-shadow: 0 0 4px rgba(0,0,0,0.25);
        cursor: pointer;
    }

    .volume__slider::-moz-range-progress {
        background: var(--txt-first);
        height: 6px;
        border-radius: 6px;
    }

    .volume__slider::-webkit-slider-runnable-track {
        height: 6px;
        border-radius: 6px;
        background: linear-gradient(
                to right,
                var(--txt-first) calc(var(--volume-percent, 50%) * 1%),
                color-mix(in srgb, var(--txt-first) 30%, transparent) calc(var(--volume-percent, 50%) * 1%)
        );
    }

    .volume__slider:hover,
    .volume__btn:hover {
        opacity: 0.95;
    }

    .volume__btn svg {
        display: block;
        width: 1rem;
        height: 1rem;
    }

</style>