
<script>
    // @ts-nocheck    
    import { invoke } from "@tauri-apps/api/tauri"
    import { prevent_default } from "svelte/internal";
    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
        return 0
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
        return 0
    }
    function update_champ () {
        console.log(selected_unit)
    }
    let champs_list = []
    let items_list = []
    fetch_champs()
    fetch_items()
    
    let selected_unit = fetch_champs()
    let selected_item = fetch_items()

    async function handle_submit_update_champ (e) {
        await invoke("update_unit", {selectedUnit : selected_unit})
        fetch_champs()
    }

</script>


<h1>Change Stats</h1>

<form on:submit|preventDefault={handle_submit_update_champ}>
    <select bind:value = {selected_unit} on:change={update_champ} required>
        {#each champs_list as champ}
            <option value = {champ}>
                {champ.id}
            </option>
        {/each}
    </select>
    <br>
    <label>AD</label>
    <input type= "number" bind:value="{selected_unit.ad}" step = 0.01 required><br>
    <label>HP</label>
    <input type="number" bind:value="{selected_unit.hp}" step = 0.01 required><br>
    <label>Attack Speed</label>
    <input type="number" bind:value="{selected_unit.attack_speed}" step = 0.01 required><br>
    <label>Ar</label>
    <input type="number" bind:value="{selected_unit.ar}" step = 0.01 required><br>
    <label>Mr</label>
    <input type="number" bind:value="{selected_unit.mr}" step = 0.01 required><br>
    <label>Mc</label>
    <input type="number" bind:value="{selected_unit.mc}" required><br>
    <label>Ra</label>
    <input type="number" bind:value="{selected_unit.ra}" required><br>
    <label>Sm</label>
    <input type="number" bind:value="{selected_unit.sm}" required><br>

    <button type = "submit">Submit</button>

</form>
