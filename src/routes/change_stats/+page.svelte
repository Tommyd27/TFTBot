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

    }
    let champs_list = []
    let items_list = []
    fetch_champs()
    fetch_items()
    
    let selected_champ = fetch_champs()
    let selected_item = fetch_items()
</script>

<select value = {selected_champ} on:change={update_champ}>
    {#each champs_list as champ}
        <option value = {champ}>
            {champ.id}
        </option>
    {/each}
</select>

{#await selected_champ}
    <p>loading...</p>
{:then selected_champ} 
    <input value="{champs_list[selected_champ].hp}">
{/await}

<h1>Change Stats</h1>
