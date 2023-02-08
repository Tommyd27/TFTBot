<script>
    // @ts-nocheck
    //import required components
    import { invoke } from "@tauri-apps/api/tauri";
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    //create array of ids
    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
    function create_grid() { //create empty grid
        let grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
        return grid
    }

    //fetch list of previous results from ipc
    async function fetch_previous_result() {
        return await invoke("fetch_outcomes")    
    }
    //fetch specific board
    async function view_battle(battle_id) {
        let outcomes = await invoke("fetch_outcome_board", {id : battle_id}) //fetch board
        for(let i = 0; i < outcomes.length; i++) {
            outcomes[i].team -= 1 //reduce team by 1, as they are stored as 1 larger than they should be in database
            outcomes[i].placed_id = generate_id() //generate id
            grid[outcomes[i].location.x][outcomes[i].location.y] = outcomes[i] //update grid with new champ
        }
    }

    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }

    function generate_id() {
        num_placed_champs += 1 //add 1 to placed champs
        if(num_placed_champs == 0) { //cant log 0, return A
            return "A"
        }
        let num_to_id = num_placed_champs
        let id_str = "" //empty string
        let cap = log_base_n(ids.length, num_to_id) // get numbers of characters needed
        for(let i = 0; i < cap + 1; i++) {
            if(num_to_id == 0){ //cant log 0, add A to string
                id_str += "A"
                break
            }
            let log_n = log_base_n(ids.length, num_to_id)
            let divisor = Math.pow(ids.length, log_n)
            let id_index = Math.floor(num_to_id / divisor)
            id_str += ids[id_index] //add new char
            num_to_id -= divisor * id_index
        }
        return id_str //return ID
    }

    let num_placed_champs = -1
    function hex_click(i, j) { //update selected champ
        selected_champ = grid[i][j]
    }
    let outcomes = fetch_previous_result() //fetch previous outcomes
    let grid = create_grid() //create empty grid
    let selected_champ;
</script>
<div class="row">
    <div class="column1_noselect">
        {#if selected_champ} <!--if there is selected champ, show details-->
            <h1>Champ: {selected_champ.placed_id}</h1>
            <h1>Type: {selected_champ.id}</h1>
            <h1>Star Level: {selected_champ.star}</h1>
            <h1>Items: {selected_champ.items}</h1>
        {/if}
    </div>
    <div class="column2">
        {#each grid as row, i}
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent> <!--indent every other line-->
            {/if}
            {#each row as hex, j} <!-- create hexagon grid-->
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)}></Hexagon>
            {/each}
            <div class = "hex-row"></div>
        {/each}
    </div>
    <div class="column3">
        <h1>Previous Results</h1>
        <h1>Click for more information</h1>
        {#await outcomes} <!--wait for outcomes from backend-->
            <h1>Fetching Results</h1>
        {:then outcomes} <!--display outcomes, button for each to view board-->
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
