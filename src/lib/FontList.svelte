<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  let shortFontList = ""

  async function getFonts(){
    shortFontList = await invoke("get_fonts")
  }
</script>

<div class="w-full">
  <form class="row" on:submit|preventDefault={getFonts}>
    <button type="submit">Get Fonts</button>
  </form> 
    {#if shortFontList!=""}
        {#each shortFontList.split(",") as font}
            <a href="/"><div class="w-full m-2 p-6 text-black hover:text-blue-600 bg-stone-300 rounded-md text-left">{font}</div></a>
        {/each}
    {/if}
</div>
