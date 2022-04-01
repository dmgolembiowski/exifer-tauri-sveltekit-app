<script lang='ts'>
  import { dispatch } from '$lib/tauri';

  async function onclick() {
    let res = await dispatch('add_image_root', {
      root: 'james',
    }).then((message) => console.log(message));
  }

</script>

<h1>Welcome to SvelteKit</h1>

<button on:click={() => onclick()}>Call Tauri</button>
<p>
  Visit <a href='https://kit.svelte.dev'>kit.svelte.dev</a> to read the documentation
</p>
