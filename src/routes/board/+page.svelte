<script>
    // @ts-nocheck 
    import UnitItemDragDrop from "$lib/UnitItemDragDrop.svelte";
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    import HexagonIndentFr from "$lib/HexagonIndentFr.svelte";
    import HexagonFr from "$lib/HexagonFr.svelte";
    import { invoke } from "@tauri-apps/api/tauri"
    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
        let champ = champs_list[0]
        champ.placed_id = "A"
        champ.items = [3, 4, 6];
        grid[1][1] = champ
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
    }
    function hex_click(i, j) {
        console.log(i, j)
        champ = grid[i][j]
        console.log(champ)
        if (champ == show) {
            show = null
        }
        else {
            console.log("hello")
            show = champ
            pos = [50 * i, 50 * j];
        }
    }
    function create_grid() {
        let grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
        return grid
    }
    let grid = create_grid()
    //grid[1][2] = obj
    let champs_list = []
    let items_list = []
    let show;
    let pos = [0, 0];
    fetch_champs()
    fetch_items()

    let champ = champs_list[0]
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
        {#each grid as row, i}
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent>
            {/if}
            {#each row as hex, j}
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)} ></Hexagon>
            {/each}
            <div class = "hex-row"></div>
        {/each}
    </div>
    <div class="column3">
        <h1>borger</h1>
    </div>
</div>

{#if show} 
    <div style = "position: absolute; top: {pos[0]}; right: {pos[1]}">
        <h1>Champ: {show.placed_id}</h1>
        <h1>Type: {show.id}</h1>
        <h1>Star Level: {show.star_level}</h1>
        <h1>Items: {show.items}</h1>
    </div>
{/if}


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
    .hex-row {
        clear: left;
    }
</style>