import { writable } from 'svelte/store';

export interface Location {
  root: string;
}

export const locations = writable([]);
