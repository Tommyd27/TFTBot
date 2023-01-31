<script>
    // @ts-nocheck
    import { invoke } from "@tauri-apps/api/tauri";
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";

    
    function create_grid() {
        let grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
        return grid
    }
    async function fetch_previous_result() {
        return await invoke("fetch_outcomes")    
    }
    async function view_battle(battle_id) {
        let outcomes = await invoke("fetch_outcome_board", {id : battle_id})
        console.log(outcomes)
        console.log(battle_id)
    }

    function hex_click(i, j) {
        selected_champ = grid[i][j]
    }
    let outcomes = fetch_previous_result()
    let grid = create_grid()
    let selected_champ;
</script>
<div class="row">
    <div class="column1_noselect">
        {#if selected_champ}
            <div>C</div>
        {/if}
    </div>
    <div class="column2">
        {#each grid as row, i}
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent>
            {/if}
            {#each row as hex, j}
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)}></Hexagon>
            {/each}
            <div class = "hex-row"></div>
        {/each}
    </div>
    <div class="column3">
        <h1>Previous Results</h1>
        <h1>Click for more information</h1>
        {#await outcomes}
            <h1>Fetching Results</h1>
        {:then outcomes} 
            {#each outcomes as outcome}
                {#if outcome[0] != 0}
                    <button on:click = {() => view_battle(outcome[1])}>Match : {outcome[1]}, Winner : {outcome[0]}</button>
                {:else}
                    <button on:click = {() => view_battle(outcome[1])}>Match : {outcome[1]}, Draw</button>
                {/if}
                <br>
            {/each}
        {/await}
    </div>
</div>




<style>
    .row {
        display: flex;
    }
    
    .column1_noselect {
        flex: 20%;
        background-color: grey;
        user-select: none;
    }

    .column3 {
        flex: 10%;
        background-color: grey;
        display:grid;
    }

    .column2 {
        flex : 70%;
        background-color: aliceblue;
    }
    .hex-row {
        clear: left;
    }
</style>
