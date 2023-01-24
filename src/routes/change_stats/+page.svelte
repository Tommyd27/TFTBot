<script>
    import { invoke } from "@tauri-apps/api/tauri"
    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
        return 0
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
        return 0
    }
    function update_champ () {
        console.log(selected_champ)
    }
    let champs_list = []
    let items_list = []
    fetch_champs()
    fetch_items()
    
    let selected_champ = fetch_champs()
    let selected_item = fetch_items()
</script>


<h1>Change Stats</h1>

<select bind:value = {selected_champ} on:change={update_champ}>
    {#each champs_list as champ}
        <option value = {champ}>
            {champ.id}
        </option>
    {/each}
</select>


<input bind:value="{selected_champ.ad[0]}">
<input bind:value="{selected_champ.ad[1]}">
<input bind:value="{selected_champ.ad[2]}">
<input bind:value="{selected_champ.hp[0]}">
<input bind:value="{selected_champ.hp[1]}">
<input bind:value="{selected_champ.hp[2]}">
<input bind:value="{selected_champ.attack_speed}">
<input bind:value="{selected_champ.ar}">
<input bind:value="{selected_champ.mr}">
<input bind:value="{selected_champ.mc}">
<input bind:value="{selected_champ.ra}">
<input bind:value="{selected_champ.sm}">