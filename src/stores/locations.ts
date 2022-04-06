import { writable } from 'svelte/store';

export interface Location {
  root: string;
  parsed: boolean;
  totalFileCount: number,
  parsableFileCount: number,
  largestImageSize: number,
  extensionMap: Map<string, number>,
  errors: string[],
}

export const locations = writable([]);
