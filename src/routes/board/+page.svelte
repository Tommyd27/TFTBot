<script>
    // @ts-nocheck 
    import UnitItemDragDrop from "$lib/UnitItemDragDrop.svelte";
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
        <UnitItemDragDrop champ_or_item = {1}></UnitItemDragDrop>
        <UnitItemDragDrop champ_or_item = {2}></UnitItemDragDrop>
        <UnitItemDragDrop champ_or_item = {3}></UnitItemDragDrop>
        <UnitItemDragDrop champ_or_item = {4}></UnitItemDragDrop>

    </div>
    <div class="column2">
        <h2>bURGER</h2>
    </div>
    <div class="column1">
        <h1>borger</h1>
    </div>
</div>

<style>
    .row {
        display: flex;
    }
    
    .column1 {
        flex: 10%;
        background-color: grey;
        display:grid;
        grid-template-columns: 40px 40px;
    }

    .column2 {
        flex : 80%;
        background-color: aliceblue;
    }
</style>