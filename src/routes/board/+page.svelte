<script>
    // @ts-nocheck 
    import UnitItemDragDrop from "$lib/UnitItemDragDrop.svelte";
    import Hexagon from "$lib/Hexagon.svelte";
    import { invoke } from "@tauri-apps/api/tauri"
    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
    }
    let champs_list = []
    let items_list = []
    fetch_champs()
    fetch_items()
</script>

<h1>Board</h1>

<div class="row">
    <div class="column1">
        <h2 style = "font-size: 20px">Champs</h2><br>
        {#each champs_list as champ}
            <UnitItemDragDrop champ_or_item = {champ}></UnitItemDragDrop>
        {/each}
        <h2 style = "font-size: 20px">Items</h2><br>
        {#each items_list as item}
            <UnitItemDragDrop champ_or_item = {item}></UnitItemDragDrop>
        {/each}
    </div>
    <div class="column2">
        <Hexagon></Hexagon>
    </div>
    <div class="column3">
        <h1>borger</h1>
    </div>
</div>

<style>
    .row {
        display: flex;
    }
    
    .column1 {
        flex: 5%;
        background-color: grey;
        display:grid;
        grid-template-columns: 40px 40px;
    }

    .column3 {
        flex: 15%;
        background-color: grey;
        display:grid;
        grid-template-columns: 40px 40px;
    }

    .column2 {
        flex : 80%;
        background-color: aliceblue;
    }
</style>