import { writable } from 'svelte/store';
import type { Track } from './types/track';

interface PlayerState {
    track: Track | null;
    current_pos: number;
    queue: Track[];
    isPlaying: boolean;
}

const initialState: PlayerState = {
    track: null,
    current_pos: 0,
    queue: [],
    isPlaying: false,
};

const { subscribe, update } = writable<PlayerState>(initialState);

export const playerStore = {
    subscribe,

    play: (playlist: Track[], num: number) => update((state) => ({ ...state, queue: playlist, track: playlist[num], isPlaying: true, current_pos: num})),
    next: () => update((state) => ({ ...state, track: state.queue[state.current_pos + 1], isPlaying: true, current_pos: state.current_pos+1})),
    prev: () => update((state) => ({ ...state, track: state.queue[state.current_pos - 1], isPlaying: true, current_pos: state.current_pos-1})),

    setTrack: (track: Track) =>
        update((state) => ({ ...state, track, isPlaying: true })),
    togglePlayback: () =>
        update((state) => ({ ...state, isPlaying: !state.isPlaying })),
};