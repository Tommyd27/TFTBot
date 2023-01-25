<script>
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
        console.log(selected_champ)
    }
    let champs_list = []
    let items_list = []
    fetch_champs()
    fetch_items()
    
    let selected_champ = fetch_champs()
    let selected_item = fetch_items()

    function handle_submit (e) {
        console.log(e)
    }

</script>


<h1>Change Stats</h1>

<form on:submit|preventDefault={handle_submit}>
    <select bind:value = {selected_champ} on:change={update_champ} required>
        {#each champs_list as champ}
            <option value = {champ}>
                {champ.id}
            </option>
        {/each}
    </select>
    <br>
    <label>AD</label>
    <input type= bind:value="{selected_champ.ad}" required><br>
    <label>HP</label>
    <input type="number" bind:value="{selected_champ.hp}" required><br>
    <label>Attack Speed</label>
    <input type="number" bind:value="{selected_champ.attack_speed}" required><br>
    <label>Ar</label>
    <input type="number" bind:value="{selected_champ.ar}" required><br>
    <label>Mr</label>
    <input type="number" bind:value="{selected_champ.mr}" required><br>
    <label>Mc</label>
    <input type="number" bind:value="{selected_champ.mc}" required><br>
    <label>Ra</label>
    <input type="number" bind:value="{selected_champ.ra}" required><br>
    <label>Sm</label>
    <input type="number" bind:value="{selected_champ.sm}" required><br>

    <button type = "submit">Submit</button>

</form>
