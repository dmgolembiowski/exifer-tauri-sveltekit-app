<script>
  import { locations } from '../stores/locations';
  import { openDirDialog } from '$lib/tauri.ts';
  import { onMount } from 'svelte';
  import { clearSavedLocations, loadLocationsFromFile, saveLocationsToFile } from '../lib/locations.ts';

  onMount(async () => {
    let savedLocations = await loadLocationsFromFile();
    locations.set(savedLocations);
  });

  async function saveLocation() {
    let location = await openDirDialog()
      .then(res => {
        return { root: res.root, paths: res.paths };
      });

    let updatedLocations = await saveLocationsToFile(location);
    locations.set(updatedLocations);
  }

  async function clearLocations() {
    let updatedLocations = await clearSavedLocations();
    locations.set(updatedLocations);
  }

</script>

<svelte:head>
  <title>Locations</title>
</svelte:head>

<button on:click={saveLocation}>Add location</button>
{#each $locations as loc}
  <h1>{loc.root}</h1>
  <h2>{loc.paths.length}</h2>
{/each}

<button on:click={() => clearLocations()}>Clear</button>
