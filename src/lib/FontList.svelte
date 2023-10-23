<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let shortFontList = ""

  async function getFonts(){
    shortFontList = await invoke("get_fonts")
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={getFonts}>
    <button type="submit">Get Fonts</button>
  </form>
  <p>{shortFontList}</p>
</div>