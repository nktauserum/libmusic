<script lang="ts">
    import { playerStore } from '$lib/store';

    let currentTime: number = 0;
    let duration: number = 0;
    let volume: number = 1;

    // Функция для форматирования времени (мм:сс)
    function formatTime(time: number): string {
        if (isNaN(time)) return '00:00';
        const minutes = Math.floor(time / 60);
        const seconds = Math.floor(time % 60);
        return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
</script>

<div class="player">
    <h2>{$playerStore.track?.title}</h2>
    <p>{$playerStore.track?.artists.join(", ")}</p>

    <audio
            src={$playerStore.audioSrc}
            autoplay={$playerStore.isPlaying}
            bind:paused={$playerStore.isPlaying}
            bind:currentTime
            bind:duration
            bind:volume
    ></audio>

    <button on:click={() => playerStore.togglePlayback()}>
        {$playerStore.isPlaying ? 'Pause' : 'Play'}
    </button>

    <div class="progress">
        <span>{formatTime(currentTime)}</span>
        <input type="range" bind:value={currentTime} min="0" max={duration} step="0.1" />
        <span>{formatTime(duration)}</span>
    </div>

    <div class="volume">
        <label>Громкость:</label>
        <input type="range" bind:value={volume} min="0" max="1" step="0.01" />
    </div>
</div>

<style>
    .player {
        max-width: 400px;
        margin: 0 auto;
        padding: 20px;
        border: 1px solid #ccc;
        border-radius: 8px;
    }
    button {
        padding: 10px 20px;
        background-color: #007bff;
        color: white;
        border: none;
        cursor: pointer;
    }
    .progress,
    .volume {
        display: flex;
        align-items: center;
        margin-top: 10px;
    }
    .progress span {
        min-width: 40px;
    }
    input[type="range"] {
        flex: 1;
        margin: 0 10px;
    }
</style>