<script lang="ts">
    import {onMount} from "svelte";
    import type {Track} from "$lib/types/track";
    import TrackList from '$lib/components/TrackList.svelte';

    let tracks: Track[] | null = $state(null);

    async function getTracks(): Promise<Track[]> {
        let response = await fetch("http://localhost:6432/tracks");
        return JSON.parse(await response.text())["tracks"];
    }

    onMount(async () => {
        tracks = await getTracks();
        console.log(tracks);
    });
</script>

<TrackList {tracks} />