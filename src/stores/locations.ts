import { writable } from 'svelte/store';
import type { Location } from '../types/Location';

export const locations = writable<Location[]>([]);
