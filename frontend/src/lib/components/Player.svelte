<script lang="ts">
    import { playerStore } from '$lib/store';

    let currentTime: number = 0;
    let duration: number = 0;
    let volume: number = 1;

    function formatTime(time: number): string {
        if (isNaN(time)) return '00:00';
        const minutes = Math.floor(time / 60);
        const seconds = Math.floor(time % 60);
        return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
</script>

<div class="player">
    <div class="track">
        <div class="track__cover"></div>
        <div class="track__info" >
            <div class="info__title">{$playerStore.track?.title}</div>
            <a href="/artist" class="info__artist">{$playerStore.track?.artists.join(", ")}</a>
        </div>
    </div>

    <audio
            src={`http://localhost:6432/track/${$playerStore.track?.id}`}
            autoplay={$playerStore.isPlaying}
            bind:paused={$playerStore.isPlaying}
            bind:currentTime
            bind:duration
            bind:volume
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

        <button class="ctrl play" aria-label="play" onclick={() => playerStore.togglePlayback()}>
            <svg
                    color="var(--txt-first)"
                    width="50"
                    height="50"
                    viewBox="0 0 16 16">
                <path fill="currentColor" d="M10.804 8L5 4.633v6.734L10.804 8zm.792-.696a.802.802 0 0 1 0 1.392l-6.363 3.692C4.713 12.69 4 12.345 4 11.692V4.308c0-.653.713-.998 1.233-.696l6.363 3.692z"/>
            </svg>
        </button>

        <!--		<button class="ctrl play" >-->
        <!--			<svg-->
        <!--					color="var(&#45;&#45;txt-first)"-->
        <!--					width="40"-->
        <!--					height="50"-->
        <!--					viewBox="0 0 56 56">-->
        <!--				<path fill="currentColor" d="M16.832 47.09h5.32c2.063 0 3.117-1.078 3.117-3.14V12.026c0-2.132-1.054-3.117-3.117-3.117h-5.32c-2.062 0-3.14 1.078-3.14 3.117V43.95c0 2.063 1.078 3.14 3.14 3.14m17.016 0h5.32c2.063 0 3.14-1.078 3.14-3.14V12.026c0-2.132-1.078-3.117-3.14-3.117h-5.32c-2.063 0-3.117 1.078-3.117 3.117V43.95c0 2.063 1.054 3.14 3.117 3.14"/>-->
        <!--			</svg>-->
        <!--		</button>-->

        <button class="ctrl" aria-label="next">
            <svg color="var(--txt-second)" width="32" height="32" viewBox="0 0 16 16"><path fill="currentColor" d="M14 2.5a.5.5 0 1 0-1 0v11a.5.5 0 0 0 1 0v-11ZM2 3.002a1 1 0 0 1 1.579-.816l7 4.963a1 1 0 0 1 .006 1.628l-7 5.037A1 1 0 0 1 2 13.003V3.002Zm8 4.963L3 3.002v10l7-5.037Z"/></svg>
        </button>
    </div>

<!--    <div class="tools">-->
<!--        <button class="tl" aria-label="">-->
<!--            <svg color="var(&#45;&#45;txt-second)" width="32" height="32" viewBox="0 0 512 512"><path fill="currentColor" d="M96 146.025V16H64v130.025a64.009 64.009 0 0 0 0 123.95V496h32V269.975a64.009 64.009 0 0 0 0-123.95ZM80 240a32 32 0 1 1 32-32a32.036 32.036 0 0 1-32 32Zm192 50.025V16h-32v274.025a64.009 64.009 0 0 0 0 123.95V496h32v-82.025a64.009 64.009 0 0 0 0-123.95ZM256 384a32 32 0 1 1 32-32a32.036 32.036 0 0 1-32 32ZM448 82.025V16h-32v66.025a64.009 64.009 0 0 0 0 123.95V496h32V205.975a64.009 64.009 0 0 0 0-123.95ZM432 176a32 32 0 1 1 32-32a32.036 32.036 0 0 1-32 32Z"/></svg>-->
<!--        </button>-->

<!--        <button class="tl">-->
<!--            <svg color="var(&#45;&#45;txt-second)" width="32" height="32" viewBox="0 0 32 32"><path fill="currentColor" d="M6 6h20.172l-3.586-3.586L24 1l6 6l-6 6l-1.414-1.414L26.172 8H6v7H4V8a2.002 2.002 0 0 1 2-2zm3.414 14.414L5.828 24H26v-7h2v7a2.002 2.002 0 0 1-2 2H5.828l3.586 3.586L8 31l-6-6l6-6z"/></svg>-->
<!--        </button>-->

<!--        <button class="tl">-->
<!--            <svg color="var(&#45;&#45;txt-second)" width="32" height="32" viewBox="0 0 1200 1200"><path fill="currentColor" d="M935.926 42.203v186.061H763.958c-54.408 0-114.484 26.559-164.729 77.32c-50.242 50.761-104.842 126.065-191.527 249.904c-87.076 124.394-135.567 199.565-165.807 233.346c-30.24 33.78-25.376 30.882-69.388 30.882H0v147.863h172.507c66.078 0 132.54-27.619 179.515-80.093c46.975-52.475 91.312-125.164 176.742-247.208c85.82-122.601 140.381-195.159 175.512-230.651c35.129-35.491 36.641-33.5 59.685-33.5h171.967v194.147L1200 306.276L935.926 42.203zM0 228.263v147.863h172.507c44.012 0 39.148-2.975 69.388 30.805c19.456 21.734 51.507 67.826 91.49 125.915c5.419-7.773 7.973-11.521 13.708-19.716c21.78-31.114 41.563-59.187 59.838-84.79c6.36-8.91 11.688-15.939 17.714-24.259c-27.021-39.039-49.525-70.001-72.623-95.803c-46.975-52.474-113.437-80.015-179.515-80.015H0zm935.926 401.464v189.988H763.958c-23.043 0-24.554 1.915-59.684-33.577c-23.237-23.477-56.146-65.093-99.809-124.76c-5.281 7.49-9.555 13.418-15.095 21.333c-30.571 43.674-51.648 75.183-73.777 107.816c31.395 41.578 58.12 73.875 83.637 99.652c50.242 50.763 110.319 77.397 164.729 77.397h171.968v190.22L1200 893.801L935.926 629.727z"/></svg>-->
<!--        </button>-->

<!--    </div>-->
    <div class="volume">
        <label>Громкость:</label>
        <input type="range" bind:value={volume} min="0" max="1" step="0.01" />
    </div>
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