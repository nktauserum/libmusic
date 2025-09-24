import { writable } from 'svelte/store';
import type { Track } from './types/track';

interface PlayerState {
    track: Track | null;
    audioSrc: string;
    isPlaying: boolean;
}

const initialState: PlayerState = {
    track: null,
    audioSrc: '',
    isPlaying: false,
};

const { subscribe, update } = writable<PlayerState>(initialState);

export const playerStore = {
    subscribe,
    setTrack: (track: Track) =>
        update((state) => ({ ...state, track, isPlaying: true })),
    togglePlayback: () =>
        update((state) => ({ ...state, isPlaying: !state.isPlaying })),
    stop: () => update((state) => ({ ...state, isPlaying: false })),
};