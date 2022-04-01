import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { configDir } from '@tauri-apps/api/path';

export async function openDirDialog() {
  return open({
    directory: true,
    multiple: true,
    title: 'Select directories',
    defaultPath: '/Users/james/sorted/',
  })
    .then((dir) => invoke('add_location', { root: dir }));
}

export async function createConfigDir() {
  configDir().then(res => console.log(res));
}
