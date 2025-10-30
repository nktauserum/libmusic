import type {Track} from "$lib/types/track";

export interface Playlist {
    id: number,
    name: string,
    created_at: string,
    tracks: Track[],
}