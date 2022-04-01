<script lang='ts'>
  import { createConfigDir, openDirDialog } from '$lib/tauri';
  import { locations } from '../stores/locations';

  let newLocation = '/Users/james/sorted/test/';

  createConfigDir();

  interface Location {
    root: string;
    paths: string[];
  }

  async function addLocation() {
    await openDirDialog()
      .then(res => {
        locations.update(locs => [
          {
            root: res.root,
            paths: res.paths,
          },
          ...locs]);
      })
      .catch((error) => console.error(error));
  }

  function printLocations() {
    $locations.forEach((location) => {
      console.log(location);
    });
  }

  function clearLocations() {
    locations.update(() => []);
  }
</script>

<h1>Welcome to SvelteKit</h1>

<button on:click={addLocation}>Add location</button>
<button on:click={() => printLocations()}>Print Locations</button>
{#each $locations as loc}
  <h1>{loc.root}</h1>
  <h2>{loc.paths.length}</h2>
{/each}

<button on:click={() => clearLocations()}>Clear</button>
<p>
  Visit <a href='https://kit.svelte.dev'>kit.svelte.dev</a> to read the documentation
</p>
