<script lang="ts">
    import { page } from '$app/state';
    import {onMount} from "svelte";
    import type {Playlist} from "$lib/types/playlist";
    import TrackList from "$lib/components/TrackList.svelte";
    const id = page.params.id;

    let playlist: Playlist | null = $state(null);

    async function getPlaylist() {
        const response = await fetch(`http://localhost:6432/playlists/${id}`);
        playlist = JSON.parse(await response.text());
    }

    onMount(async () => {
        await getPlaylist();
    });
</script>

{#if playlist && playlist.tracks}
    <header class="playlist-header" style="margin-bottom:1rem;">
        <div style="display:flex;align-items:center;gap:1rem;">
            <img class="playlist__cover" alt="cover" src={`http://localhost:6432/cover/${playlist.tracks[0].id}`}>

            <div>
                <h1 class="playlist__name">{playlist.name}</h1>
                <p class="playlist__info">
                    Создан: {new Date(playlist.created_at).toLocaleString()}
                </p>
                <p class="playlist__info">
                    Треков: {playlist.tracks?.length ?? 0}
                </p>
            </div>
        </div>
    </header>

    <TrackList tracks={ playlist.tracks }></TrackList>
{:else}
    <p>Загрузка плейлиста…</p>
{/if}

<style>
    header {
        margin: 2rem 0;
    }

    .playlist__cover {
        width: 96px;
        height: 96px;
        background: #eee;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 6px;
    }

    .playlist__name {
        margin: 0;
        font-size: 1.4rem;
    }

    .playlist__info {
        margin: 0.25rem 0;
        color: #666;
        font-size: 0.9rem;
    }
</style>