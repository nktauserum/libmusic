<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';

    type Playlist = {
        id: string;
        name: string;
    };

    let playlists: Playlist[] = [];

    async function loadPlaylists() {
        try {
            const res = await fetch('http://localhost:6432/playlists');
            const data = await res.json();
            console.log(data);
            playlists = data.playlists ?? [];
        } catch (e) {
            console.error('Failed to load playlists', e);
            playlists = [];
        }
    }

    onMount(() => {
        loadPlaylists();
    });

    function isActive(playlist: Playlist) {
        return $page.url.pathname.startsWith(`/playlist/${playlist.id}`);
    }
</script>

<nav class="playlists" aria-label="Playlists">
    {#each playlists as pl}
        <a href={`/playlist/${pl.id}`} class:active={isActive(pl)}>{pl.name}</a>
    {/each}
</nav>

<style>
    .playlists {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        padding: 1rem;
    }

    .playlists a {
        text-decoration: none;
        color: var(--txt-second);
        padding: 0.5rem 0.75rem;
        border-radius: 6px;
        transition: background .15s, color .15s;
    }

    .playlists a:hover {
        background: var(--bg-second);
        color: var(--txt-first);
    }

    .playlists a.active {
        color: var(--txt-first);
        font-weight: 700;
    }
</style>