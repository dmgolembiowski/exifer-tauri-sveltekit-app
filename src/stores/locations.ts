import { writable } from 'svelte/store';

export interface Location {
  root: string;
  paths: string[];
}

export const locations = writable([]);
