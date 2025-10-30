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

{#if playlist && playlist.tracks }
    <TrackList tracks={ playlist.tracks }></TrackList>
{/if}