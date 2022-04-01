<script lang='ts'>
  import { dispatch } from '$lib/tauri';

  async function onclick() {
    let output = await dispatch('greet', {
      name: 'Alice',
      age: 32
    });
    console.log('~>', output);

    let res =  await dispatch('my_custom_command', [])
      .then((message) => console.log(message));
    //=> "~> Hello Alice, 32 year-old human!"
  }

</script>

<h1>Welcome to SvelteKit</h1>

<button on:click={() => onclick()}>Call Tauri</button>
<p>
  Visit <a href='https://kit.svelte.dev'>kit.svelte.dev</a> to read the documentation
</p>
