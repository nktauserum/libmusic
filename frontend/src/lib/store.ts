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

    play: (playlist: Track[], num: number) =>
        update((state) => ({
            ...state,
            queue: playlist,
            track: playlist[num],
            isPlaying: true,
            current_pos: num,
        })),

    next: () =>
        update((state) => {
            const nextPos = state.current_pos + 1;
            if (nextPos < state.queue.length) {
                return {
                    ...state,
                    track: state.queue[nextPos],
                    isPlaying: true,
                    current_pos: nextPos,
                };
            }
            return { ...state, isPlaying: false };
        }),

    prev: () =>
        update((state) => {
            const prevPos = state.current_pos - 1;
            if (prevPos >= 0 && state.queue[prevPos]) {
                return {
                    ...state,
                    track: state.queue[prevPos],
                    isPlaying: true,
                    current_pos: prevPos,
                };
            }
            return state;
        }),

    setTrack: (track: Track) =>
        update((state) => ({ ...state, track, isPlaying: true })),

    togglePlayback: () =>
        update((state) => ({ ...state, isPlaying: !state.isPlaying })),
};