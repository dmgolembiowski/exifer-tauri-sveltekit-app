import { configDir } from '@tauri-apps/api/path';
import { readTextFile, removeFile, writeFile } from '@tauri-apps/api/fs';
import type { Location } from '../types/Location';

const locationsFile = configDir().then(dir => dir.concat('exifer-tauri-sveltekit-app/locations.json'));

export async function loadLocationsFromFile(): Promise<Location[]> {
  const file = await locationsFile;
  return readTextFile(file)
    .then(JSON.parse)
    .catch((err) => {
      console.log('No locations file found. Creating one now. ' + err);
      return writeFile({
        path: file,
        contents: JSON.stringify([]),
      })
        .then(() => []);
    });
}

export async function saveLocationsToFile(location: Location): Promise<Location[]> {
  const file = await locationsFile;
  let locations = await loadLocationsFromFile();

  if (locations.filter(loc => {
    if (loc.root === location.root && loc.parsed === location.parsed) {
      return locations;
    } else if (loc.root === location.root) {
      locations = locations.filter(loc => loc.root !== location.root);
    }
  }))

    locations.push(location);

  return writeFile({
    path: file,
    contents: JSON.stringify(locations),
  }).then(() => locations);
}

export async function clearSavedLocations(): Promise<Location[]> {
  const file = await locationsFile;
  return removeFile(file).then(() => []);
}
