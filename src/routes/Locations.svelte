<script lang='ts'>
  import { locations } from '../stores/locations';
  import type { Location } from '../types/Location';
  import { onMount } from 'svelte';
  import { clearSavedLocations, loadLocationsFromFile, saveLocationsToFile } from '../lib/storage.ts';
  import { analyseLocation, openDirDialog } from '../lib/tauri.ts';

  onMount(async () => {
    let savedLocations: Location[] = await loadLocationsFromFile();
    locations.set(savedLocations);
  });

  async function saveLocation() {
    let location = await openDirDialog();

    let updatedLocations: Location[] = await saveLocationsToFile(location);
    locations.set(updatedLocations);
    await saveLocationAnalysis(location);
  }

  async function saveLocationAnalysis(location: Location) {
    let result = await analyseLocation(location.root);
    //
    // if (result === null) {
    //   return;
    // }
    //
    // let updatedLocations = await saveLocationsToFile(result);
    // locations.set(updatedLocations);
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
  <!--{#if loc.parsed}-->
    <p>Total files: {loc.totalFiles}</p>
    <p>Parsable files: {loc.parsableFiles}</p>
    <p>Largest image size: {loc.largestImageSize}</p>
    <p>Extension map: {loc.extensionMap}</p>
    <!--{#each [...loc.extensionMap] as [key, value]}-->
    <!--              <li>{key} => {value}</li>-->
    <!--            {/each}-->
    <p>Errors: {loc.errors}</p>
  <!--{:else}-->
<!--    <p>Parsing</p>  {/if}-->
{/each}

<button on:click={() => clearLocations()}>Clear</button>
