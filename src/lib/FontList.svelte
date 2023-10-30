<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  let shortFontList = ""

  async function getFonts(){
    shortFontList = await invoke("get_fonts")
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={getFonts}>
    <button class="color-red-500" type="submit">Get Fonts</button>
  </form>
  <p class="border border-red-100">{shortFontList}</p>
</div>
