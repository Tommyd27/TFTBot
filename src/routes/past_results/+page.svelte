<script>
    // @ts-nocheck
    import { invoke } from "@tauri-apps/api/tauri";
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";

    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
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
        for(let i = 0; i < outcomes.length; i++) {
            outcomes[i].team -= 1
            outcomes[i].placed_id = generate_id()
            grid[outcomes[i].location.x][outcomes[i].location.y] = outcomes[i]
        }
    }
    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }
    function generate_id() {
        num_placed_champs += 1
        if(num_placed_champs == 0) {
            return "A"
        }
        let num_to_id = num_placed_champs
        let id_str = ""
        let cap = log_base_n(ids.length, num_to_id)
        for(let i = 0; i < cap + 1; i++) {
            if(num_to_id == 0){
                id_str += "A"
                break
            }
            let log_n = log_base_n(ids.length, num_to_id)
            let divisor = Math.pow(ids.length, log_n)
            let id_index = Math.floor(num_to_id / divisor)
            id_str += ids[id_index]
            num_to_id -= divisor * id_index
        }
        return id_str
    }

    let num_placed_champs = -1
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
            <h1>Champ: {selected_champ.placed_id}</h1>
            <h1>Type: {selected_champ.id}</h1>
            <h1>Star Level: {selected_champ.star}</h1>
            <h1>Items: {selected_champ.items}</h1>
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
