import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import type { Location } from '../stores/locations';

export async function openDirDialog() {
  return open({
    directory: true,
    multiple: true,
    title: 'Select directories',
    defaultPath: '/Users/james/sorted/',
  })
    .then((dir) => invoke('add_location', { root: dir }));
}

export async function analyseLocation(location: Location) {
  return invoke('analyse_location', { location });
}
