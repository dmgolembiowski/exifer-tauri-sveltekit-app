import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

type Dict<T> = Record<string, T>;

export async function dispatch(command: string, args: Dict<string | number | string[]>) {
  return invoke(command, args);
}

export async function openDirDialog() {
  return open({
    directory: true,
    multiple: true,
    title: 'Select directories',
    defaultPath: '/Users/james/sorted/',
  })
    .then((dir) => invoke('add_location', { root: dir }));
}
