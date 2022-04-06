<script lang='ts'>
  import { locations } from '../stores/locations';
  import { onMount } from 'svelte';
  import { clearSavedLocations, loadLocationsFromFile, saveLocationsToFile } from '../lib/locations.ts';
  import { analyseLocation, openDirDialog } from '../lib/tauri.ts';

  export interface Location {
    root: string;
    parsed: boolean;
    totalFileCount: number,
    parsableFileCount: number,
    largestImageSize: number,
    extensionMap: Map<string, number>,
    errors: string[],
  }

  onMount(async () => {
    let savedLocations = await loadLocationsFromFile();
    locations.set(savedLocations);
  });

  async function saveLocation() {
    let location = await openDirDialog()
      .then(async res => {
        const loc: Location = {
          root: res.root,
          parsed: false,
          extensionMap: new Map<string, number>([
            ['key1', 1],
            ['key2', 2],
          ]),
          totalFileCount: 0,
          parsableFileCount: 0,
          largestImageSize: 0,
          errors: [],
        };

        return loc;
      });

    let updatedLocations = await saveLocationsToFile(location);
    locations.set(updatedLocations);
    await getLocationAnalysis(location);
  }

  async function getLocationAnalysis(location: Location) {
    await analyseLocation(location)
      .then(async res => {
        const updatedLoc: Location = {
          root: location.root as string,
          parsed: true,
          totalFileCount: res.total_files as number,
          parsableFileCount: res.parsable_files as number,
          largestImageSize: res.largest_image_size as number,
          extensionMap: res.extension_map as Map<string, number>,
          errors: res.errors,
        };
        console.log(typeof updatedLoc );
        let updatedLocations = await saveLocationsToFile(updatedLoc);
        locations.set(updatedLocations);
      });
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
  {#if loc.parsed}
    <p>Total files: {loc.totalFileCount}</p>
    <p>Parsable files: {loc.parsableFileCount}</p>
    <p>Largest image size: {loc.largestImageSize}</p>
    <p>Extension map: {loc.extensionMap.size}</p>
    <!--{#each [...loc.extensionMap.entries()] as [key, value]}-->
    <!--      <li>{key} => {value}</li>-->
    <!--    {/each}-->
    <p>Errors: {loc.errors.size}</p>
  {:else}
    <p>Parsing</p>  {/if}
{/each}

<button on:click={() => clearLocations()}>Clear</button>
